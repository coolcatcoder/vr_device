use std::pin::Pin;

use autocxx::subclass::CppSubclass;

use crate::{
    DeviceProvider, Hmd, debugging::{self, log}, ffi::vr::{
        ETrackedDeviceClass, EVRInitError, IServerTrackedDeviceProvider_methods, IVRDriverContext, InitServerDriverContext, VRServerDriverHost
    }, interface_versions::k_InterfaceVersions
};

impl IServerTrackedDeviceProvider_methods for DeviceProvider {
    unsafe fn Init(&mut self, context: *mut IVRDriverContext) -> EVRInitError {
        let error = unsafe { InitServerDriverContext(context) };
        if error != EVRInitError::VRInitError_None {
            return error;
        }

        debugging::set_panic_hook();

        log(c"Hello world!");

        self.hmd = Hmd::new_cpp_owned(Hmd {cpp_peer: Default::default()});

        let host = unsafe { Pin::new_unchecked(&mut * VRServerDriverHost()) };
        unsafe { host.TrackedDeviceAdded(c"HMD".as_ptr(), ETrackedDeviceClass::TrackedDeviceClass_HMD, self.hmd.as_mut_ptr().cast()) };

        EVRInitError::VRInitError_None
    }

    // Secretly this returns a *const *const c_char
    fn GetInterfaceVersions(&mut self) -> *const autocxx::c_void {
        k_InterfaceVersions.as_ptr().cast()
    }

    fn ShouldBlockStandbyMode(&mut self) -> bool {
        false
    }

    fn RunFrame(&mut self) {}

    fn EnterStandby(&mut self) {}

    fn LeaveStandby(&mut self) {}

    fn Cleanup(&mut self) {}
}
