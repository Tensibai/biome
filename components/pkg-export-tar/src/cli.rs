use clap::Parser;

use crate::common::{cli::clap_validators::{BioPackageInstallSourceValueParser,
                                           UrlValueParser},
                    consts::{DEFAULT_BUILDER_URL,
                             DEFAULT_HAB_LAUNCHER_PKG_IDENT,
                             DEFAULT_HAB_PKG_IDENT,
                             DEFAULT_HAB_SUP_PKG_IDENT}};

#[derive(Debug, Clone, Parser)]
#[command(
    name = "bio-pkg-export-tar",
    author = concat!("\nAuthors: ", clap::crate_authors!()),
    about = "Creates a tar package from a Biome package",
    version = crate::VERSION,
    help_template = "{name} {version} {author-section} {about-section} \
                    \n{usage-heading} {usage}\n\n{all-args}",
    max_term_width = 100)]
pub(crate) struct Cli {
    /// Biome CLI package identifier (ex: acme/redis) or filepath to a Biome artifact
    /// (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart) to install
    #[arg(name = "HAB_PKG",
          long = "bio-pkg",
          value_name = "HAB_PKG",
          value_parser = BioPackageInstallSourceValueParser,
          default_value = DEFAULT_HAB_PKG_IDENT)]
    pub(crate) bio_pkg: String,

    /// Launcher package identifier (ex: biome/bio-launcher) or filepath to a Biome artifact
    /// (ex: /home/biome-bio-launcher-13829-20200527165030-x86_64-linux.hart) to install
    #[arg(name = "HAB_LAUNCHER_PKG",
          long = "launcher-pkg",
          value_name = "HAB_LAUNCHER_PKG",
          value_parser = BioPackageInstallSourceValueParser,
          default_value = DEFAULT_HAB_LAUNCHER_PKG_IDENT)]
    pub(crate) bio_launcher_pkg: String,

    /// Supervisor package identifier (ex: biome/bio-sup) or filepath to a Biome artifact
    /// (ex: /home/biome-bio-sup-1.6.39-20200527165021-x86_64-linux.hart) to install
    #[arg(name = "HAB_SUP_PKG",
          long = "sup-pkg",
          value_name = "HAB_SUP_PKG",
          value_parser = BioPackageInstallSourceValueParser,
          default_value = DEFAULT_HAB_SUP_PKG_IDENT)]
    pub(crate) bio_sup_pkg: String,

    /// Builder URL to Install packages from
    #[arg(name = "BLDR_URL",
          long = "url",
          short = 'u',
          value_name = "BLDR_URL",
          value_parser = UrlValueParser,
          default_value = DEFAULT_BUILDER_URL)]
    pub(crate) bldr_url: String,

    /// Channel to install packages from
    #[arg(name = "CHANNEL",
          long = "channel",
          short = 'c',
          value_name = "CHANNEL",
          default_value = "stable")]
    pub(crate) channel: String,

    /// URL to install base packages from
    #[arg(name = "BASE_PKGS_BLDR_URL",
          long = "base-pkgs-url",
          value_name = "BASE_PKGS_BLDR_URL",
          value_parser = UrlValueParser,
          default_value = DEFAULT_BUILDER_URL)]
    pub(crate) base_pkgs_url: String,

    /// Channel to install base packages from
    #[arg(name = "BASE_PKGS_CHANNEL",
          long = "base-pkgs-channel",
          value_name = "BASE_PKGS_CHANNEL",
          default_value = "stable")]
    pub(crate) base_pkgs_channel: String,

    /// Provide a Builder auth token for private pkg export
    #[arg(name = "BLDR_AUTH_TOKEN",
          long = "auth",
          short = 'z',
          value_name = "BLDR_AUTH_TOKEN")]
    pub(crate) bldr_auth_token: Option<String>,

    /// A Biome package identifier (ex: acme/redis) and/or filepath to a Biome artifact
    /// (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[arg(name = "PKG_IDENT_OR_ARTIFACT",
          value_name = "PKG_IDENT_OR_ARTIFACT",
          value_parser = BioPackageInstallSourceValueParser,
          required = true)]
    pub(crate) pkg_ident: String,
}
