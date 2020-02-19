//! Delete a package from Builder.
//!
//! # Examples
//!
//! ```bash
//! $ bio pkg delete acme/redis/2.0.7/2112010203120101
//! ```
//! //! This will delete the acme package specified from Builder
//! //! if certain conditions apply - for example, the package is not
//! //! in the stable channel, and does not have any other packages that
//! //! depend on it.
//!
//! //! Note: This command does not remove the package from disk

use crate::{api_client::{self,
                         Client},
            common::ui::{Status,
                         UIWriter,
                         UI},
            error::{Error,
                    Result},
            hcore::package::{PackageIdent,
                             PackageTarget},
            PRODUCT,
            VERSION};
use reqwest::StatusCode;

/// Delete a package from Builder.
///
/// # Failures
///
/// * Fails if it cannot find the specified package in Builder
pub async fn start(ui: &mut UI,
                   bldr_url: &str,
                   (ident, target): (&PackageIdent, PackageTarget),
                   token: &str)
                   -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;

    ui.begin(format!("Deleting {} ({}) from Builder", ident, target))?;

    match api_client.delete_package((ident, target), token).await {
        Ok(_) => {
            ui.status(Status::Deleted, format!("{} ({})", ident, target))?;
            Ok(())
        }
        Err(err @ api_client::Error::APIError(StatusCode::NOT_FOUND, _)) => {
            ui.fatal(format!("This package does not exist, or alternatively, you may need to \
                              specify a valid platform\ntarget argument other than {}.",
                             target))?;
            Err(Error::APIClient(err))
        }
        Err(err @ api_client::Error::APIError(StatusCode::UNPROCESSABLE_ENTITY, _)) => {
            ui.fatal("Before you can delete this package artifact, demote it from the `stable` \
                      channel\nand remove any reverse dependencies.")?;
            ui.fatal(format!("Demote the package artifact with the command:\nbio pkg demote {} \
                              stable {}",
                             ident, target))?;
            ui.fatal(format!("Discover any reverse dependencies with the command:\nbio pkg \
                              dependencies --reverse {}",
                             ident))?;
            Err(Error::APIClient(err))
        }
        Err(e) => {
            ui.fatal(format!("Failed to delete the package! {:?}.", e))?;
            Err(Error::from(e))
        }
    }
}
