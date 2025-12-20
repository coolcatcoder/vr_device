use crate::{bindings::{ServerTrackedDeviceProvider, ServerTrackedDeviceProviderVtable, VrInitError, k_InterfaceVersions}, ffi::vr::InitServerDriverContext};

unsafe extern "C" fn init(
        this: *mut ServerTrackedDeviceProvider,
        context: *mut crate::bindings::vr_IVRDriverContext,
    ) -> VrInitError {        
        let error = unsafe { InitServerDriverContext(context.cast()) } as u32;
        let error = VrInitError::try_from(0);

        vr::EVRInitError eError = vr::InitServerDriverContext( pContext );
        if ( eError != vr::VRInitError_None ) {
            return eError;
        }
    }

unsafe extern "C" fn cleanup(
    this: *mut ServerTrackedDeviceProvider,
) {
    
}

unsafe extern "C" fn get_interface_versions(
    this: *mut ServerTrackedDeviceProvider,
) -> *const *const i8 {
    unsafe { k_InterfaceVersions.as_ptr() }
}

unsafe extern "C" fn run_frame(
    this: *mut ServerTrackedDeviceProvider,
) {

}

unsafe extern "C" fn should_block_standby_mode(
    this: *mut ServerTrackedDeviceProvider,
) -> bool {
    false
}

unsafe extern "C" fn enter_standby(
    this: *mut ServerTrackedDeviceProvider,
) {

}

unsafe extern "C" fn leave_standby(
    this: *mut ServerTrackedDeviceProvider,
) {
    
}

static DEVICE_PROVIDER_VTABLE: ServerTrackedDeviceProviderVtable = ServerTrackedDeviceProviderVtable {
    init,
    cleanup,
    get_interface_versions,
    run_frame,
    should_block_standby_mode,
    enter_standby,
    leave_standby,
};