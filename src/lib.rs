//#![warn(clippy::pedantic)]

use autocxx::{prelude::*, subclass::subclass};

mod debugging;
mod device_provider;
mod hmd_driver_factory;
#[allow(nonstandard_style)]
mod interface_versions;

include_cpp! {
    #include "openvr_driver.h"
    safety!(unsafe)
    generate!("vr::InitServerDriverContext")
    generate!("vr::IServerTrackedDeviceProvider_Version")
    generate!("vr::IServerTrackedDeviceProvider")
    generate!("vr::IVRDriverLog")
    generate!("vr::VRDriverLog")
    subclass!("vr::IServerTrackedDeviceProvider", DeviceProvider)
}

#[subclass]
#[derive(Default)]
pub struct DeviceProvider;
