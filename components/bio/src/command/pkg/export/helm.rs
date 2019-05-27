use std::ffi::OsString;

use crate::common::ui::UI;

use crate::error::Result;

const EXPORT_CMD: &str = "bio-pkg-export-helm";
const EXPORT_CMD_ENVVAR: &str = "HAB_PKG_EXPORT_HELM_BINARY";
const EXPORT_PKG_IDENT: &str = "biome/bio-pkg-export-helm";
const EXPORT_PKG_IDENT_ENVVAR: &str = "HAB_PKG_EXPORT_HELM_PKG_IDENT";

pub fn start(ui: &mut UI, args: &[OsString]) -> Result<()> {
    crate::command::pkg::export::export_common::start(ui,
                                                      args,
                                                      EXPORT_CMD,
                                                      EXPORT_CMD_ENVVAR,
                                                      EXPORT_PKG_IDENT,
                                                      EXPORT_PKG_IDENT_ENVVAR)
}
