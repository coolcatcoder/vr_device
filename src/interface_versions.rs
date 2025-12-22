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

const IVRSettings_Version: &CStr = c"IVRSettings_003";
const ITrackedDeviceServerDriver_Version: &CStr = c"ITrackedDeviceServerDriver_005";
const IVRDisplayComponent_Version: &CStr = c"IVRDisplayComponent_003";
const IVRDriverDirectModeComponent_Version: &CStr = c"IVRDriverDirectModeComponent_009";
const IVRCameraComponent_Version: &CStr = c"IVRCameraComponent_003";
const IServerTrackedDeviceProvider_Version: &CStr = c"IServerTrackedDeviceProvider_004";
const IVRWatchdogProvider_Version: &CStr = c"IVRWatchdogProvider_001";
const IVRVirtualDisplay_Version: &CStr = c"IVRVirtualDisplay_002";
const IVRDriverManager_Version: &CStr = c"IVRDriverManager_001";
const IVRResources_Version: &CStr = c"IVRResources_001";
const IVRCompositorPluginProvider_Version: &CStr = c"IVRCompositorPluginProvider_001";
const IVRIPCResourceManagerClient_Version: &CStr = c"IVRIPCResourceManagerClient_002";
