use super::util::CacheKeyPath;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(no_version)]
/// Commands relating to Biome rings
pub enum Ring {
    Key(Key),
}

#[derive(StructOpt)]
#[structopt(no_version)]
/// Commands relating to Biome ring keys
pub enum Key {
    /// Outputs the latest ring key contents to stdout
    Export {
        /// Ring key name
        #[structopt(name = "RING")]
        ring:           String,
        #[structopt(flatten)]
        cache_key_path: CacheKeyPath,
    },
    /// Generates a Biome ring key
    Generate {
        /// Ring key name
        #[structopt(name = "RING")]
        ring:           String,
        #[structopt(flatten)]
        cache_key_path: CacheKeyPath,
    },
    /// Reads a stdin stream containing ring key contents and writes the key to disk
    Import {
        #[structopt(flatten)]
        cache_key_path: CacheKeyPath,
    },
}
