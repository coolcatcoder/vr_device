use std::ptr::null_mut;

use crate::{Hmd, ffi::vr::{EVRInitError, ITrackedDeviceServerDriver_methods}};

impl ITrackedDeviceServerDriver_methods for Hmd {
    fn Activate(&mut self, unObjectId:u32) -> EVRInitError {
        EVRInitError::VRInitError_None
    }

    fn Deactivate(&mut self) {
        
    }

    unsafe fn GetComponent(&mut self, pchComponentNameAndVersion: *const ::std::os::raw::c_char) ->  *mut autocxx::c_void {
        null_mut()
    }

    unsafe fn DebugRequest(&mut self, pchRequest: *const ::std::os::raw::c_char, pchResponseBuffer: *mut ::std::os::raw::c_char, unResponseBufferSize:u32) {
        
    }

    fn GetPose(&mut self) -> cxx::UniquePtr<crate::ffi::vr::DriverPose_t> {
        unreachable!()
    }

    fn EnterStandby(&mut self) {
        
    }
}