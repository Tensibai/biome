use std::{env,
          path::PathBuf};

use biome_win_users::account::Account;

use crate::error::{Error,
                   Result};

extern "C" {
    pub fn GetUserTokenStatus() -> u32;
}

fn get_sid_by_name(name: &str) -> Option<String> {
    match Account::from_name(name) {
        Some(acct) => {
            match acct.sid.to_string() {
                Ok(username) => Some(username),
                Err(_) => None,
            }
        }
        None => None,
    }
}

pub fn get_uid_by_name(owner: &str) -> Option<String> { get_sid_by_name(owner) }

// this is a no-op on windows
pub fn get_gid_by_name(group: &str) -> Option<String> { Some(String::new()) }

pub fn get_current_username() -> Option<String> {
    match env::var("USERNAME").ok() {
        Some(username) => Some(username.to_lowercase()),
        None => None,
    }
}

// this is a no-op on windows
pub fn get_current_groupname() -> Option<String> { Some(String::new()) }

pub fn get_effective_uid() -> u32 { unsafe { GetUserTokenStatus() } }

pub fn get_home_for_user(username: &str) -> Option<PathBuf> {
    unimplemented!();
}

pub fn root_level_account() -> String { env::var("COMPUTERNAME").unwrap().to_uppercase() + "$" }

/// Windows does not have a concept of "group" in a Linux sense
/// So we just validate the user
pub fn assert_pkg_user_and_group(user: &str, _group: &str) -> Result<()> {
    match get_uid_by_name(user) {
        Some(_) => Ok(()),
        None => {
            Err(Error::PermissionFailed(format!("Package requires user \
                                                 {} to exist, but it \
                                                 doesn't",
                                                user)))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn get_uid_of_current_user() {
        assert!(get_current_username().map(|s| get_uid_by_name(&s))
                                      .is_some())
    }

    #[test]
    fn downcase_current_username() {
        let orig_user = get_current_username().unwrap();
        env::set_var("USERNAME", "uSer");
        assert_eq!(get_current_username().unwrap(), "user");
        env::set_var("USERNAME", orig_user);
    }

    #[test]
    fn return_none_when_no_user() {
        let orig_user = get_current_username().unwrap();
        env::remove_var("USERNAME");
        assert_eq!(get_current_username(), None);
        env::set_var("USERNAME", orig_user);
    }
}
