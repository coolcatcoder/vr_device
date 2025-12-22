use crate::{
    DeviceProvider,
    debugging::{self, log},
    ffi::vr::{
        EVRInitError, IServerTrackedDeviceProvider_methods, IVRDriverContext,
        InitServerDriverContext,
    },
    interface_versions::k_InterfaceVersions,
};

impl IServerTrackedDeviceProvider_methods for DeviceProvider {
    unsafe fn Init(&mut self, context: *mut IVRDriverContext) -> EVRInitError {
        let error = unsafe { InitServerDriverContext(context) };
        if error != EVRInitError::VRInitError_None {
            return error;
        }

        debugging::set_panic_hook();

        log(c"Working?");

        // Works up to here.
        return EVRInitError::VRInitError_Unknown;

        //let context = unsafe { &mut *context };

        // driver log
        // let driver_log = unsafe { ffi::vr::IVRDriverLog::allocate_uninitialized_cpp_storage() };
        // let mut driver_log = unsafe { UniquePtr::from_raw(driver_log) };
        // let message = c"Testing!";
        // unsafe { driver_log.as_mut().unwrap().Log(message.as_ptr()) };

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
