//#![warn(clippy::pedantic)]

use std::sync::{Arc, mpsc::Sender};

use autocxx::{prelude::*, subclass::subclass};
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano_util::{context::VulkanoContext, window::VulkanoWindows};
use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
use crate::{ffi::{HmdCpp, RenderingHandleCpp}, rendering::{Rendering, UserEvent}};

mod debugging;
mod device_provider;
mod hmd;
mod hmd_driver_factory;
#[allow(nonstandard_style)]
mod interface_versions;
mod rendering;
#[allow(nonstandard_style)]
#[allow(clippy::upper_case_acronyms)]
mod dxgi_format;

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
    generate!("vr::IVRDriverDirectModeComponent")
    generate_pod!("vr::IVRDriverDirectModeComponent_SwapTextureSetDesc_t")
    generate_pod!("vr::IVRDriverDirectModeComponent_SwapTextureSet_t")

    subclass!("vr::IServerTrackedDeviceProvider", DeviceProvider)
    subclass!("vr::ITrackedDeviceServerDriver", Hmd)
    subclass!("vr::IVRDriverDirectModeComponent", RenderingHandle)
}

#[subclass]
pub struct DeviceProvider {
    hmd: UniquePtr<HmdCpp>,
}

#[subclass]
pub struct Hmd {
    rendering: UniquePtr<RenderingHandleCpp>,
}

#[subclass]
pub struct RenderingHandle {
    event_loop_proxy: EventLoopProxy<UserEvent>,
}