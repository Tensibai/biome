//! Runs a service lifecycle hook on a separate thread, and wraps the
//! whole execution in a future.
//!
//! Ideally, we'd want to use something like
//! [tokio_process](https://github.com/alexcrichton/tokio-process),
//! but we're not able to use that based on how our Windows hooks get
//! executed. If that were to be re-cast in terms of Rust's
//! `std::process::Command`, we could consider it. In the meantime,
//! this seems to do the trick.

use super::{hook_timer,
            Pkg};
use biome_common::templating::hooks::Hook;
use biome_core::service::ServiceGroup;
use std::{clone::Clone,
          sync::Arc,
          time::{Duration,
                 Instant}};
use tokio::{task,
            task::JoinError};

pub struct HookRunner<H: Hook + Sync> {
    hook:          Arc<H>,
    service_group: ServiceGroup,
    pkg:           Pkg,
    passwd:        Option<String>,
}

// We cannot use `#[derive(Clone)]` here because it unnecessarily requires `H` to be
// `Clone`. See https://github.com/rust-lang/rust/issues/44151.
impl<H: Hook + Sync> Clone for HookRunner<H> {
    fn clone(&self) -> Self {
        Self { hook:          self.hook.clone(),
               service_group: self.service_group.clone(),
               pkg:           self.pkg.clone(),
               passwd:        self.passwd.clone(), }
    }
}

impl<H> HookRunner<H> where H: Hook + Sync + 'static
{
    pub fn new(hook: Arc<H>,
               service_group: ServiceGroup,
               pkg: Pkg,
               passwd: Option<String>)
               -> HookRunner<H> {
        HookRunner { hook,
                     service_group,
                     pkg,
                     passwd }
    }

    pub async fn retryable_future(self) {
        loop {
            match self.clone().into_future().await {
                Ok((maybe_exit_value, _duration)) => {
                    // If we did not get an exit value always retry
                    if maybe_exit_value.as_ref().map_or(true, H::should_retry) {
                        debug!("Retrying the '{}' hook", H::file_name());
                    } else {
                        break;
                    }
                }
                Err(e) => error!("Error running the '{}' hook: {:?}", H::file_name(), e),
            }
        }
    }

    pub async fn into_future(self) -> Result<(Option<H::ExitValue>, Duration), JoinError> {
        // TODO (CM): May want to consider adding a configurable
        // timeout to how long this hook is allowed to run.
        task::spawn_blocking(move || {
            // _timer is for Prometheus metrics, but we also want
            // the runtime for other purposes. Unfortunately,
            // we're not able to use the same timer for both :(
            let _timer = hook_timer(H::file_name());
            let start = Instant::now();
            let exit_value = self.hook
                                 .run(&self.service_group, &self.pkg, self.passwd.as_ref())
                                 .ok();
            let run_time = start.elapsed();
            (exit_value, run_time)
        }).await
    }
}
