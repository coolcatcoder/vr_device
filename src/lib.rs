//#![warn(clippy::pedantic)]

use autocxx::prelude::*;

mod hmd_driver_factory;
mod device_provider;
// To generate new bindings:
// bindgen ./src/wrapper.hpp -o ./src/new_bindings.rs --vtable-generation --allowlist-item "vr::IServerTrackedDeviceProvider"
// Then copy across what you need.
#[allow(unused)]
#[allow(nonstandard_style)]
mod bindings;

include_cpp! {
    #include "openvr_driver.h"
    generate!("vr::InitServerDriverContext")
    generate!("vr::IServerTrackedDeviceProvider_Version")
}

// static DEVICE_PROVIDER: ffi::vr::IServerTrackedDeviceProvider = ffi::vr::IServerTrackedDeviceProvider {
//     vtable_: null(),
// };

// unsafe extern "C" fn init(
//         this: *mut bindings::ServerTrackedDeviceProvider,
//         context: *mut bindings::vr_IVRDriverContext,
//     ) -> bindings::VrInitError {
//         todo!()
//     }

// static DEVICE_PROVIDER_VTABLE: bindings::ServerTrackedDeviceProviderVtable = bindings::ServerTrackedDeviceProviderVtable {
//     init,
//     cleanup: todo!(),
//     get_interface_versions: todo!(),
//     run_frame: todo!(),
//     should_block_standby_mode: todo!(),
//     enter_standby: todo!(),
//     leave_standby: todo!(),
// };

// static device_provider = bindings::ServerTrackedDeviceProvider {
//     v
// }
