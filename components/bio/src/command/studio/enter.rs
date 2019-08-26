use std::{env,
          ffi::OsString,
          fs as stdfs,
          path::PathBuf};

use crate::{common::ui::UI,
            hcore::{crypto::CACHE_KEY_PATH_ENV_VAR,
                    env as henv,
                    fs}};

use crate::{config,
            error::Result,
            BLDR_URL_ENVVAR,
            CTL_SECRET_ENVVAR,
            ORIGIN_ENVVAR};

use biome_core::AUTH_TOKEN_ENVVAR;

pub const ARTIFACT_PATH_ENVVAR: &str = "ARTIFACT_PATH";
pub const CERT_PATH_ENVVAR: &str = "CERT_PATH";

const STUDIO_CMD: &str = "bio-studio";
const STUDIO_CMD_ENVVAR: &str = "HAB_STUDIO_BINARY";
const STUDIO_PACKAGE_IDENT: &str = "biome/bio-studio";

#[derive(Clone, Copy)]
enum Sensitivity {
    PrintValue,
    NoPrintValue,
}

fn set_env_var_from_config(env_var: &str, config_val: Option<String>, sensitive: Sensitivity) {
    if henv::var(env_var).is_err() {
        if let Some(val) = config_val {
            match sensitive {
                Sensitivity::NoPrintValue => {
                    debug!("Setting {}=REDACTED (sensitive) via config file", env_var)
                }
                Sensitivity::PrintValue => debug!("Setting {}={} via config file", env_var, val),
            }
            env::set_var(env_var, val);
        }
    }
}

pub fn start(ui: &mut UI, args: &[OsString]) -> Result<()> {
    let config = config::load()?;

    set_env_var_from_config(AUTH_TOKEN_ENVVAR,
                            config.auth_token,
                            Sensitivity::NoPrintValue);
    set_env_var_from_config(BLDR_URL_ENVVAR, config.bldr_url, Sensitivity::PrintValue);
    set_env_var_from_config(CTL_SECRET_ENVVAR,
                            config.ctl_secret,
                            Sensitivity::NoPrintValue);
    set_env_var_from_config(ORIGIN_ENVVAR, config.origin, Sensitivity::PrintValue);

    if henv::var(CACHE_KEY_PATH_ENV_VAR).is_err() {
        let path = fs::cache_key_path(None::<&str>);
        debug!("Setting {}={}", CACHE_KEY_PATH_ENV_VAR, path.display());
        env::set_var(CACHE_KEY_PATH_ENV_VAR, &path);
    };

    let artifact_path = match henv::var(ARTIFACT_PATH_ENVVAR) {
        Ok(p) => PathBuf::from(p),
        Err(_) => {
            let path = fs::cache_artifact_path(None::<&str>);
            debug!("Setting {}={}", ARTIFACT_PATH_ENVVAR, path.display());
            env::set_var(ARTIFACT_PATH_ENVVAR, &path);
            path
        }
    };
    if !artifact_path.is_dir() {
        debug!("Creating artifact_path at: {}", artifact_path.display());
        stdfs::create_dir_all(&artifact_path)?;
    }

    let ssl_path = match henv::var(CERT_PATH_ENVVAR) {
        Ok(p) => PathBuf::from(p),
        Err(_) => {
            let path = fs::cache_ssl_path(None::<&str>);
            debug!("Setting {}={}", CERT_PATH_ENVVAR, path.display());
            env::set_var(CERT_PATH_ENVVAR, &path);
            path
        }
    };
    if !ssl_path.is_dir() {
        debug!("Creating ssl_path at: {}", ssl_path.display());
        stdfs::create_dir_all(&ssl_path)?;
    }
    inner::start(ui, args)
}

#[cfg(target_os = "linux")]
mod inner {
    use crate::{command::studio::docker,
                common::ui::{UIWriter,
                             UI},
                error::{Error,
                        Result},
                exec,
                hcore::{crypto::init,
                        env as henv,
                        fs::{am_i_root,
                             find_command},
                        os::process,
                        package::{PackageIdent,
                                  PackageInstall},
                        users::linux as group},
                VERSION};
    use std::{env,
              ffi::OsString,
              path::PathBuf,
              str::FromStr};

    const SUDO_CMD: &str = "sudo";

    pub fn start(ui: &mut UI, args: &[OsString]) -> Result<()> {
        rerun_with_sudo_if_needed(ui, &args)?;
        if is_docker_studio(&args) {
            docker::start_docker_studio(ui, args)
        } else {
            let command = match henv::var(super::STUDIO_CMD_ENVVAR) {
                Ok(command) => PathBuf::from(command),
                Err(_) => {
                    init();
                    let version: Vec<&str> = VERSION.split('/').collect();
                    let ident = PackageIdent::from_str(&format!("{}/{}",
                                                                super::STUDIO_PACKAGE_IDENT,
                                                                version[0]))?;
                    let command = exec::command_from_min_pkg(ui, super::STUDIO_CMD, &ident)?;
                    // This is a duplicate of the code in `bio pkg exec` and
                    // should be refactored as part of or after:
                    // https://github.com/habitat-sh/habitat/issues/6633
                    // https://github.com/habitat-sh/habitat/issues/6634
                    let pkg_install = PackageInstall::load(&ident, None)?;
                    let cmd_env = pkg_install.environment_for_command()?;
                    for (key, value) in cmd_env.into_iter() {
                        debug!("Setting: {}='{}'", key, value);
                        env::set_var(key, value);
                    }

                    let mut display_args = super::STUDIO_CMD.to_string();
                    for arg in args {
                        display_args.push(' ');
                        display_args.push_str(arg.to_string_lossy().as_ref());
                    }
                    debug!("Running: {}", display_args);

                    command
                }
            };

            if let Some(cmd) = find_command(command.to_string_lossy().as_ref()) {
                process::become_command(cmd, args)?;
                Ok(())
            } else {
                Err(Error::ExecCommandNotFound(command))
            }
        }
    }

    fn is_docker_studio(args: &[OsString]) -> bool {
        if cfg!(not(target_os = "linux")) {
            return false;
        }

        for arg in args.iter() {
            let str_arg = arg.to_string_lossy();
            if str_arg == "-D" {
                return true;
            }
        }

        false
    }

    fn has_docker_group() -> bool {
        let current_user = group::get_current_username().unwrap();
        let docker_members = group::get_members_by_groupname("docker");
        docker_members.map_or(false, |d| d.contains(&current_user))
    }

    fn rerun_with_sudo_if_needed(ui: &mut UI, args: &[OsString]) -> Result<()> {
        // If I have root permissions or if I am executing a docker studio
        // and have the appropriate group - early return, we are done.
        if am_i_root() || (is_docker_studio(args) && has_docker_group()) {
            return Ok(());
        }

        // Otherwise we will try to re-run this program using `sudo`
        match find_command(SUDO_CMD) {
            Some(sudo_prog) => {
                let mut args: Vec<OsString> = vec!["-p".into(),
                                                   "[sudo bio-studio] password for %u: ".into(),
                                                   "-E".into(),];
                args.append(&mut env::args_os().collect());
                process::become_command(sudo_prog, &args)?;
                Ok(())
            }
            None => {
                ui.warn(format!("Could not find the `{}' command, is it in your PATH?",
                                SUDO_CMD))?;
                ui.warn("Running Biome Studio requires root or administrator privileges. \
                         Please retry this command as a super user or use a privilege-granting \
                         facility such as sudo.")?;
                ui.br()?;
                Err(Error::RootRequired)
            }
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod inner {
    use crate::{command::studio::docker,
                common::ui::UI,
                error::{Error,
                        Result},
                exec,
                hcore::{crypto::init,
                        env as henv,
                        fs::find_command,
                        os::process,
                        package::PackageIdent},
                VERSION};
    use std::{ffi::OsString,
              path::PathBuf,
              str::FromStr};

    pub fn start(_ui: &mut UI, args: &[OsString]) -> Result<()> {
        if is_windows_studio(&args) {
            start_windows_studio(_ui, args)
        } else {
            docker::start_docker_studio(_ui, args)
        }
    }

    pub fn start_windows_studio(ui: &mut UI, args: &[OsString]) -> Result<()> {
        let command = match henv::var(super::STUDIO_CMD_ENVVAR) {
            Ok(command) => PathBuf::from(command),
            Err(_) => {
                init();
                let version: Vec<&str> = VERSION.split('/').collect();
                let ident = PackageIdent::from_str(&format!("{}/{}",
                                                            super::STUDIO_PACKAGE_IDENT,
                                                            version[0]))?;
                exec::command_from_min_pkg(ui, super::STUDIO_CMD, &ident)?
            }
        };

        if let Some(cmd) = find_command(command.to_string_lossy().as_ref()) {
            process::become_command(cmd, args)?;
        } else {
            return Err(Error::ExecCommandNotFound(command));
        }
        Ok(())
    }

    fn is_windows_studio(args: &[OsString]) -> bool {
        if cfg!(not(target_os = "windows")) {
            return false;
        }

        for arg in args.iter() {
            let str_arg = arg.to_string_lossy();
            if str_arg == "-D" {
                return false;
            }
        }

        true
    }
}
