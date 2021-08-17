//! Defines types for sending information about "actions" from one
//! part of the Supervisor to another.

use super::service::ServiceSpec;
use biome_core::os::process::ShutdownTimeout;
use std::sync::mpsc;

/// Defines the parameters by which a service process is to be shut
/// down cleanly.
#[derive(Clone, Debug, Default)]
pub struct ShutdownInput {
    /// How long to wait after sending a process a Ctrl-C to shutdown
    /// until we forcibly terminate it.
    pub timeout: Option<ShutdownTimeout>,
}

/// Describe actions initiated by user interaction in terms that the
/// Supervisor itself can understand and operate on.
// TODO (CM): More actions will be added to this with future
// refactorings
#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum SupervisorAction {
    StopService {
        service_spec:   ServiceSpec,
        shutdown_input: ShutdownInput,
    },
    UnloadService {
        service_spec:   ServiceSpec,
        shutdown_input: ShutdownInput,
    },
    UpdateService {
        service_spec: ServiceSpec,
    },
}

pub type ActionSender = mpsc::Sender<SupervisorAction>;

#[allow(clippy::from_over_into)]
impl Into<ShutdownInput> for biome_sup_protocol::ctl::SvcUnload {
    fn into(self) -> ShutdownInput {
        ShutdownInput { timeout: self.timeout_in_seconds.map(ShutdownTimeout::from), }
    }
}

#[allow(clippy::from_over_into)]
impl Into<ShutdownInput> for biome_sup_protocol::ctl::SvcStop {
    fn into(self) -> ShutdownInput {
        ShutdownInput { timeout: self.timeout_in_seconds.map(ShutdownTimeout::from), }
    }
}
