use std::{path::PathBuf, ptr::null_mut, str::Utf8Error};

use xplm_sys::{
    XPLMGetMyID, XPLMGetNthAircraftModel, XPLMGetPluginInfo, XPLMGetPrefsPath, XPLMGetSystemPath,
};

use super::ffi::StringBuffer;

pub fn plugin_path() -> Result<PathBuf, Utf8Error> {
    let mut path = StringBuffer::new(512);
    unsafe {
        XPLMGetPluginInfo(
            XPLMGetMyID(),
            null_mut(),
            path.as_mut_ptr(),
            null_mut(),
            null_mut(),
        );
    }
    Ok(PathBuf::from(path.as_str()?))
}

pub fn prefs_path() -> Result<PathBuf, Utf8Error> {
    let mut path = StringBuffer::new(512);
    unsafe {
        XPLMGetPrefsPath(path.as_mut_ptr());
    }
    Ok(PathBuf::from(path.as_str()?))
}

pub fn system_path() -> Result<PathBuf, Utf8Error> {
    let mut path = StringBuffer::new(512);
    unsafe {
        XPLMGetSystemPath(path.as_mut_ptr());
    }
    Ok(PathBuf::from(path.as_str()?))
}

pub fn aircraft_path() -> Result<PathBuf, Utf8Error> {
    let mut path = StringBuffer::new(512);
    unsafe {
        XPLMGetNthAircraftModel(0, StringBuffer::new(256).as_mut_ptr(), path.as_mut_ptr());
    }
    Ok(PathBuf::from(path.as_str()?))
}
