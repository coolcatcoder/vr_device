use std::{ffi::CStr, ptr::null_mut};

use crate::{
    Hmd, RenderingHandle, ffi::vr::{EVRInitError, ITrackedDeviceServerDriver_methods}, interface_versions::IVRDriverDirectModeComponent_Version
};

impl ITrackedDeviceServerDriver_methods for Hmd {
    fn Activate(&mut self, unObjectId: u32) -> EVRInitError {
        EVRInitError::VRInitError_None
    }

    fn Deactivate(&mut self) {}

    unsafe fn GetComponent(
        &mut self,
        component_name_and_version: *const ::std::os::raw::c_char,
    ) -> *mut autocxx::c_void {
        if IVRDriverDirectModeComponent_Version == unsafe { CStr::from_ptr(component_name_and_version) } {
            self.rendering = RenderingHandle::create();
            return self.rendering.as_mut_ptr().cast();
        }

        null_mut()
    }

    unsafe fn DebugRequest(
        &mut self,
        pchRequest: *const ::std::os::raw::c_char,
        pchResponseBuffer: *mut ::std::os::raw::c_char,
        unResponseBufferSize: u32,
    ) {
    }

    fn GetPose(&mut self) -> cxx::UniquePtr<crate::ffi::vr::DriverPose_t> {
        unreachable!()
    }

    fn EnterStandby(&mut self) {}
}
