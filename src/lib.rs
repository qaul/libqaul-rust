mod bindings;

use bindings::*;
use bindings::qaul::ql_error_t::*;
use std::ffi::*;

/// Alias the c-like error to a Rust type
pub type QaulStatus = ql_error_t;

/// Alias the c-like os to a Rust type
pub type QaulOS = qaul::qaul_os;

/// Alias the c-like auth_token to a Rust type
pub type AuthToken = *mut qaul::qaul_auth_token;

#[deprecated]
pub enum OS {
    Linux,
    MacOS,
    Windows,
}

pub struct Qaul {
    core: *mut bindings::qaul::qaul,
}

impl Drop for Qaul {
    /// Clear the qaul.net library if it goes out of scope
    fn drop(&mut self) {
        unsafe { ql_shutdown(self.core) };
    }
}

impl Qaul {

    /// Allocate and initialise a new library instance for qaul.net
    ///
    /// The home directory is where most of the data will be stored (keys, database, etc)
    /// while the resource directory is only really used for some front-ends (say, the web-server)
    ///
    pub fn new(os: qaul_os, home: &str, resources: &str) -> Result<Qaul, QaulStatus> {
        let mut me = Qaul {
            core: std::ptr::null_mut(),
        };

        let q_home = CString::new(home).unwrap().as_ptr();
        let q_res = CString::new(resources).unwrap().as_ptr();

        /* Initialise, then according to status – send a return */
        let status = unsafe { ql_initialise(&mut me.core, os, q_home, q_res) };
        return match status {
            SUCCESS => Ok(me),
            _ => Err(status),
        };
    }

    pub fn create_user(&mut self, username: &str, passphrase: &str) -> ql_error_t {
        let usr = CString::new(username).unwrap().as_ptr();
        let pass = CString::new(passphrase).unwrap().as_ptr();

        return unsafe { ql_create_user(self.core, usr, pass) };
    }

    /// 
    pub fn login(&mut self, username: &str, passphrase: &str) -> Result<AuthToken, QaulStatus> {
        let usr = CString::new(username).unwrap().as_ptr();
        let pass = CString::new(passphrase).unwrap().as_ptr();

        let mut auth = std::ptr::null_mut();
        let status = unsafe { ql_login(self.core, usr, pass, &mut auth) };
        return match status {
            SUCCESS => Ok(auth),
            _ => Err(status),
        };
    }

    /// Hand in a user token to log out a particular user
    /// 
    /// Returns a status which corresponds to the internal qaul.net status code
    pub fn logout(&mut self, token: AuthToken) -> Result<(), QaulStatus> {
        let status = unsafe { ql_logout(self.core, token) };
        return match status {
            SUCCESS => Ok(()),
            _ => Err(status),
        };
    }
}
