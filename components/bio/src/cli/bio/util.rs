use crate::cli::valid_fully_qualified_ident;
use configopt::ConfigOptDefaults;
use biome_core::{crypto::CACHE_KEY_PATH_ENV_VAR,
                   fs::CACHE_KEY_PATH,
                   package::PackageIdent};
use std::{ffi::OsString,
          net::SocketAddr,
          path::PathBuf};
use structopt::StructOpt;
use url::Url;

#[derive(StructOpt)]
#[structopt(no_version)]
#[allow(dead_code)]
pub struct AuthToken {
    /// Authentication token for Builder
    #[structopt(name = "AUTH_TOKEN", short = "z", long = "auth")]
    auth_token: Option<String>,
}

#[derive(StructOpt)]
#[structopt(no_version)]
#[allow(dead_code)]
pub struct BldrUrl {
    /// Specify an alternate Builder endpoint. If not specified, the value will be taken from
    /// the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
    // TODO (DM): This should probably use `env` and `default_value`
    #[structopt(name = "BLDR_URL", short = "u", long = "url")]
    bldr_url: Option<Url>,
}

#[derive(StructOpt, Debug, Deserialize)]
#[structopt(no_version)]
#[allow(dead_code)]
pub struct CacheKeyPath {
    /// Cache for creating and searching encryption keys. Default value is hab/cache/keys if root
    /// and .hab/cache/keys under the home directory otherwise.
    #[structopt(name = "CACHE_KEY_PATH",
                long = "cache-key-path",
                env = CACHE_KEY_PATH_ENV_VAR,
                required = true,
                // TODO (DM): This default value needs to be set dynamically based on user. We should set it
                // here instead of looking up the correct value later on. I dont understand why this value
                // has to be required.
                default_value = CACHE_KEY_PATH,
                hide_default_value = true)]
    cache_key_path: PathBuf,
}

impl ConfigOptDefaults for CacheKeyPath {
    fn arg_default(&self, _arg_path: &[String]) -> Option<OsString> {
        Some(self.cache_key_path.clone().into_os_string())
    }
}

#[derive(StructOpt)]
#[structopt(no_version)]
#[allow(dead_code)]
pub struct PkgIdent {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[structopt(name = "PKG_IDENT")]
    pkg_ident: PackageIdent,
}

#[derive(StructOpt)]
#[structopt(no_version)]
#[allow(dead_code)]
pub struct FullyQualifiedPkgIdent {
    /// A fully qualified package identifier (ex: core/busybox-static/1.42.2/20170513215502)
    #[structopt(name = "PKG_IDENT", validator = valid_fully_qualified_ident)]
    pkg_ident: PackageIdent,
}

#[derive(StructOpt)]
#[structopt(no_version)]
#[allow(dead_code)]
pub struct RemoteSup {
    /// Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]
    #[structopt(name = "REMOTE_SUP", long = "remote-sup", short = "r")]
    remote_sup: Option<SocketAddr>,
}
