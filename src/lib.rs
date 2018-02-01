mod bindings;

use bindings::qaul::*;
use std::ffi::*;

pub enum OS {
    Linux, MacOS, Windows
}

pub struct Qaul {
    core: *mut qaul,
}


impl Drop for Qaul {
    /// Clear the qaul.net library if it goes out of scope
    fn drop(&mut self) {
        ql_shutdown(self.core);
    }
}

impl Qaul {

    /// Allocate and initialise a new library instance for qaul.net
    /// 
    /// The home directory is where most of the data will be stored (keys, database, etc)
    /// while the resource directory is only really used for some front-ends (say, the web-server)
    /// 
    pub fn new(os: OS, home: &str, resources: &str) -> Option<Qaul> {
        let mut me = Qaul {
            core: std::ptr::null_mut(),
        };

        let q_os = match os {
            Linux => qaul_os_LINUX,
            MacOS => qaul_os_MACOS,
            Windows => qaul_os_WINDOWS,
        };

        let q_home = CString::new(home).unwrap().as_ptr();
        let q_res = CString::new(resources).unwrap().as_ptr();

        /* Initialise, then according to status – send a return */
        let status = ql_initialise(&mut me.core, q_os, q_home, q_res);
        return match status {
            ql_error_t_SUCCESS => Some(me),
            _ => None,
        };
    }
}