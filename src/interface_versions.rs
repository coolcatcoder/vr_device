use std::{
    ffi::{CStr, c_char},
    ptr::null,
};

pub const k_InterfaceVersions: &[*const c_char; 13] = &[
    IVRSettings_Version.as_ptr(),
    ITrackedDeviceServerDriver_Version.as_ptr(),
    IVRDisplayComponent_Version.as_ptr(),
    IVRDriverDirectModeComponent_Version.as_ptr(),
    IVRCameraComponent_Version.as_ptr(),
    IServerTrackedDeviceProvider_Version.as_ptr(),
    IVRWatchdogProvider_Version.as_ptr(),
    IVRVirtualDisplay_Version.as_ptr(),
    IVRDriverManager_Version.as_ptr(),
    IVRResources_Version.as_ptr(),
    IVRCompositorPluginProvider_Version.as_ptr(),
    IVRIPCResourceManagerClient_Version.as_ptr(),
    null(),
];

pub const IVRSettings_Version: &CStr = c"IVRSettings_003";
pub const ITrackedDeviceServerDriver_Version: &CStr = c"ITrackedDeviceServerDriver_005";
pub const IVRDisplayComponent_Version: &CStr = c"IVRDisplayComponent_003";
pub const IVRDriverDirectModeComponent_Version: &CStr = c"IVRDriverDirectModeComponent_009";
pub const IVRCameraComponent_Version: &CStr = c"IVRCameraComponent_003";
pub const IServerTrackedDeviceProvider_Version: &CStr = c"IServerTrackedDeviceProvider_004";
pub const IVRWatchdogProvider_Version: &CStr = c"IVRWatchdogProvider_001";
pub const IVRVirtualDisplay_Version: &CStr = c"IVRVirtualDisplay_002";
pub const IVRDriverManager_Version: &CStr = c"IVRDriverManager_001";
pub const IVRResources_Version: &CStr = c"IVRResources_001";
pub const IVRCompositorPluginProvider_Version: &CStr = c"IVRCompositorPluginProvider_001";
pub const IVRIPCResourceManagerClient_Version: &CStr = c"IVRIPCResourceManagerClient_002";
