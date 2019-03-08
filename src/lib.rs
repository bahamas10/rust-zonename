#![deny(warnings)]
#![deny(missing_docs)]

//! # rust-zonename
//!
//! The `rust-zonename` crate provides a rust abstraction over the illumos `getzoneid`,
//! `getzoneidbyname`, and `getzonenamebyid` C library functions.

extern crate libc;

use std::ffi::CString;
use std::io;

mod ffi;

/// Max length of a zone name.
pub const ZONENAME_MAX: usize = 64;

/// Returns the zone ID of the calling process.
///
/// # Example
/// ```
/// let zoneid = zonename::getzoneid().expect("failed to get zoneid");
/// ```
pub fn getzoneid() -> io::Result<i32> {
    let zoneid = unsafe { ffi::getzoneid() };

    match zoneid {
        -1 => Err(io::Error::last_os_error()),
        _ => Ok(zoneid),
    }
}

/// Returns the zone ID corresponding to the named zone.
///
/// # Example
/// ```
/// # let zid = zonename::getzoneid().expect("failed to get zoneid");
/// # let zonename = zonename::getzonenamebyid(zid).expect("failed to get zonename");
/// let zoneid = zonename::getzoneidbyname(&zonename).expect("failed to get zoneid");
/// ```
pub fn getzoneidbyname(zonename: &str) -> io::Result<i32> {
    let c_zonename = CString::new(zonename)?;
    let zoneid = unsafe { ffi::getzoneidbyname(c_zonename.as_ptr()) };

    match zoneid {
        -1 => Err(io::Error::last_os_error()),
        _ => Ok(zoneid),
    }
}

/// Returns the zone name for the corresponding zone ID.
///
/// # Example
/// ```
/// # let zoneid = zonename::getzoneid().expect("failed to get zoneid");
/// let zonename = zonename::getzonenamebyid(zoneid).expect("failed to get zonename");
/// ```
pub fn getzonenamebyid(id: i32) -> io::Result<String> {
    // since `CString::from_raw` is only supposed to be called after calling `into_raw` on a
    // CString, we make a bogus buffer here that the ffi function will fill in for us
    let c_string = CString::new(" ".repeat(ZONENAME_MAX)).unwrap();
    let ptr = c_string.into_raw();

    let len = unsafe { ffi::getzonenamebyid(id, ptr, ZONENAME_MAX) };
    if len < 0 {
        return Err(io::Error::last_os_error());
    }

    assert!(len >= 0);

    // Attempt to force unwrap cstring because the kernel really shouldn't be giving us back
    // invalid utf8 characters
    let cstring = unsafe {
        CString::from_raw(ptr)
            .into_string()
            .expect("found invalid UTF-8 converting from CString to String")
    };

    Ok(cstring)
}

/// Returns the current zones name for the calling process.
///
/// # Example
/// ```
/// let zid = zonename::getzoneid().expect("failed to get zoneid");
/// let zname = zonename::getzonenamebyid(zid).expect("failed to get zonename");
/// let zonename = zonename::getzonename().expect("failed to get zonename");
/// assert_eq!(zname, zonename);
/// ```
pub fn getzonename() -> io::Result<String> {
    let id = getzoneid()?;
    getzonenamebyid(id)
}
