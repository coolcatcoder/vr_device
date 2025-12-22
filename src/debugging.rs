use std::ffi::CStr;
use std::pin::Pin;
use std::{ffi::CString, fs::OpenOptions};
use std::io::Write;

use cxx::UniquePtr;

use crate::ffi::vr::{IVRDriverLog, VRDriverLog};

pub fn set_panic_hook() {
    const PANIC_LOG: &str = "/home/coolcatcoder/Documents/GitHub/vr_device/panic_log";

    // Clear out old logs.
    std::fs::write("/home/coolcatcoder/Documents/GitHub/vr_device/panic_log", "").expect("Should be able to clear logs.");

    std::panic::set_hook(Box::new(|panic_info| {
            // Everything here must succeed!
            // We can't panic, as we are the panic hook.
            let Ok(mut file) = OpenOptions::new().append(true).open(PANIC_LOG) else {
                return;
            };
            let Some(payload) = panic_info.payload_as_str() else {
                return;
            };
            let Some(location) = panic_info.location() else {
                return;
            };

            let _ = writeln!(&mut file, "{payload}\n{location}\n");
        }));
}

pub trait Log {
    fn log(self, driver_log: Pin<&mut IVRDriverLog>);
}

impl Log for String {
    fn log(self, driver_log: Pin<&mut IVRDriverLog>) {
        let message = CString::new(self).expect("No nul bytes.");
        unsafe { driver_log.Log(message.as_ptr()) };
    }
}

impl Log for &CStr {
    fn log(self, driver_log: Pin<&mut IVRDriverLog>) {
        unsafe { driver_log.Log(self.as_ptr()) };
    }
}

pub fn log(message: impl Log) {
    let mut driver_log = unsafe { UniquePtr::from_raw(VRDriverLog()) };
    message.log(driver_log.pin_mut());
    driver_log.into_raw();
}