use env_logger;
use biome_common as common;
use biome_pkg_export_tar as export_tar;
#[macro_use]
extern crate log;

use crate::{common::{ui::{UIWriter,
                          UI},
                     PROGRAM_NAME},
            export_tar::{Cli,
                         Result}};
use clap::App;

fn main() {
    let mut ui = UI::default_with_env();
    if let Err(e) = start(&mut ui) {
        ui.fatal(e).unwrap();
        std::process::exit(1)
    }
}

fn start(ui: &mut UI) -> Result<()> {
    env_logger::init();
    let cli = cli();
    let m = cli.get_matches();
    debug!("clap cli args: {:?}", m);

    export_tar::export_for_cli_matches(ui, &m)
}

fn cli<'a, 'b>() -> App<'a, 'b> {
    let name: &str = &*PROGRAM_NAME;
    let about = "Creates a tar package from a Biome package";
    Cli::new(name, about).add_base_packages_args()
                         .add_builder_args()
                         .add_pkg_ident_arg()
                         .app
}
