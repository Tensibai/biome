use biome_common::output::{self,
                             OutputFormat,
                             OutputVerbosity};
use biome_core::os::signals;
use biome_launcher::server;
use log::{error,
          log,
          Level};
use std::{env,
          process};

fn main() {
    // Set up signal handlers before anything else happens to ensure
    // that all threads spawned thereafter behave properly.
    signals::init();
    env_logger::init();
    let args: Vec<String> = env::args().skip(1).collect();
    set_global_logging_options(&args);

    match server::run(args) {
        Err(err) => {
            error!("Launcher exiting with 1 due to err: {:?}", err);
            process::exit(1);
        }
        Ok(code) => {
            let level = if code == 0 { Level::Info } else { Level::Error };
            log!(level, "Launcher exiting with code {}", code);
            process::exit(code);
        }
    }
}

/// In order to ensure that log output from the Launcher itself
/// behaves the same as the Supervisor, we'll eavesdrop on the
/// arguments being passed to the Supervisor in order to configure
/// ourselves.
fn set_global_logging_options(args: &[String]) {
    // Yeah, this is pretty weird, but it comes out of how the
    // bio-launch, bio, and bio-sup binaries interact.
    //
    // These flags are defined with CLAP on `bio`, so they can be
    // passed through `bio-launch` (and intercepted here), before
    // being passed on to `bio-sup`, where they are _also_ defined.
    //
    // What a tangled web we weave!

    // Note that each of these options has only one form, so we don't
    // have to check for long _and_ short options, for example.
    if args.contains(&String::from("--no-color")) {
        output::set_format(OutputFormat::NoColor)
    }
    if args.contains(&String::from("--json-logging")) {
        output::set_format(OutputFormat::Json)
    }
    if args.contains(&String::from("-v")) {
        output::set_verbosity(OutputVerbosity::Verbose);
    }
}
