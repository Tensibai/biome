mod bldr;
mod cli;
mod config;
mod file;
mod license;
mod origin;
mod pkg;
mod plan;
mod ring;
mod studio;
pub mod sup;
mod svc;
#[cfg(test)]
mod tests;
mod user;
mod util;

use self::{bldr::Bldr,
           cli::Cli,
           config::ServiceConfig,
           file::File,
           license::License,
           origin::Origin,
           pkg::Pkg,
           plan::Plan,
           ring::Ring,
           studio::Studio,
           sup::Sup,
           svc::Svc,
           user::User};
use crate::VERSION;
use structopt::{clap::AppSettings,
                StructOpt};

#[derive(StructOpt)]
#[structopt(name = "bio",
            version = VERSION,
            about = "\"A Biome is the natural environment for your services\" - Alan Turing",
            author = "\nThe Biome Maintainers <humans@biome.sh>\n",
            global_settings = &[AppSettings::GlobalVersion],
        )]
#[allow(clippy::large_enum_variant)]
pub enum Hab {
    #[structopt(no_version)]
    Bldr(Bldr),
    #[structopt(no_version)]
    Cli(Cli),
    #[structopt(no_version)]
    Config(ServiceConfig),
    #[structopt(no_version)]
    File(File),
    #[structopt(no_version)]
    License(License),
    #[structopt(no_version)]
    Origin(Origin),
    #[structopt(no_version)]
    Pkg(Pkg),
    #[structopt(no_version)]
    Plan(Plan),
    #[structopt(no_version)]
    Ring(Ring),
    #[structopt(no_version)]
    Studio(Studio),
    #[structopt(no_version)]
    Sup(Sup),
    /// Create a tarball of Biome Supervisor data to send to support
    #[structopt(no_version)]
    Supportbundle,
    #[structopt(no_version)]
    Svc(Svc),
    #[structopt(no_version)]
    User(User),
}
