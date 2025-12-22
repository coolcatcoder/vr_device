//#![warn(clippy::pedantic)]

use autocxx::{prelude::*, subclass::subclass};

use crate::ffi::HmdCpp;

mod debugging;
mod device_provider;
mod hmd_driver_factory;
#[allow(nonstandard_style)]
mod interface_versions;
mod hmd;

include_cpp! {
    #include "openvr_driver.h"
    safety!(unsafe)

    generate!("vr::InitServerDriverContext")
    generate!("vr::IServerTrackedDeviceProvider_Version")
    generate!("vr::IServerTrackedDeviceProvider")
    generate!("vr::IVRDriverLog")
    generate!("vr::VRDriverLog")
    generate!("vr::IVRServerDriverHost")
    generate!("vr::VRServerDriverHost")
    generate!("vr::ETrackedDeviceClass")
    generate!("vr::ITrackedDeviceServerDriver")

    subclass!("vr::IServerTrackedDeviceProvider", DeviceProvider)
    subclass!("vr::ITrackedDeviceServerDriver", Hmd)
}

#[subclass]
pub struct DeviceProvider {
    hmd: UniquePtr<HmdCpp>,
}

#[subclass]
#[derive(Default)]
pub struct Hmd;
