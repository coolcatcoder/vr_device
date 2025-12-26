use std::{array, sync::{Arc, mpsc::channel}, thread};

use autocxx::subclass::CppSubclass;
use cxx::UniquePtr;
use vulkano::{Handle, VulkanObject, command_buffer::allocator::StandardCommandBufferAllocator, format::Format, image::{Image, ImageCreateInfo, ImageType, ImageUsage, SampleCount}, memory::{ExternalMemoryHandleTypes, allocator::{AllocationCreateInfo, MemoryTypeFilter}}, pipeline::{GraphicsPipeline, graphics::viewport::Viewport}, render_pass::{Framebuffer, RenderPass}, single_pass_renderpass};
use vulkano_util::{context::{VulkanoConfig, VulkanoContext}, window::{VulkanoWindows, WindowDescriptor}};
use winit::{application::ApplicationHandler, event::StartCause, event_loop::{ActiveEventLoop, EventLoop}};

use crate::{RenderingHandle, debugging::log, dxgi_format::DxgiFormat, ffi::{RenderingHandleCpp, vr::{IVRDriverDirectModeComponent_SwapTextureSet_t, IVRDriverDirectModeComponent_SwapTextureSetDesc_t, IVRDriverDirectModeComponent_methods}}};

pub type UserEvent = Box<dyn FnOnce(&mut Rendering, &ActiveEventLoop) + Send + Sync>;

impl RenderingHandle {
    fn with_rendering<T: Send + Sync + 'static>(&self, f: impl FnOnce(&mut Rendering, &ActiveEventLoop) -> T + Send + Sync + 'static) -> T {
        let (sender, receiver) = channel();
        self.event_loop_proxy.send_event(Box::new(move |rendering, event_loop| {
            sender.send(f(rendering, event_loop)).unwrap();
        })).ok().unwrap();
        receiver.recv().unwrap()
    }

    pub fn create() -> UniquePtr<RenderingHandleCpp> {
        let context = VulkanoContext::new(VulkanoConfig::default());

        log(format!(
            "Using device: {} (type: {:?})",
            context.device().physical_device().properties().device_name,
            context.device().physical_device().properties().device_type,
        ));

        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(context.device().clone(), Default::default()));

        let (sender, receiver) = channel();
        thread::spawn(move || {
            let event_loop = EventLoop::<UserEvent>::with_user_event().build().expect("The eventloop should be able to be created.");
            let event_loop_proxy = event_loop.create_proxy();

            sender.send(event_loop_proxy).expect("The eventloop proxy must be able to be given to the renderer.");

            let windows = VulkanoWindows::default();

            let mut rendering = Rendering {
                context,
                windows,
                command_buffer_allocator,

                with_event_loop: None,
            };
            event_loop.run_app(&mut rendering).unwrap();
        });

        let event_loop_proxy = receiver.recv().expect("The event loop proxy must have been sent.");
        
        //windows.create_window(event_loop, vulkano_context, window_descriptor, swapchain_create_info_modify)

        RenderingHandle::new_cpp_owned(RenderingHandle { event_loop_proxy, cpp_peer: Default::default() })
    }
}

impl IVRDriverDirectModeComponent_methods for RenderingHandle {
    unsafe fn CreateSwapTextureSet(
        &mut self,
        _: u32,
        texture_description: *const crate::ffi::vr::IVRDriverDirectModeComponent_SwapTextureSetDesc_t,
        out_swap_texture_set: *mut crate::ffi::vr::IVRDriverDirectModeComponent_SwapTextureSet_t,
    ) {
        let texture_description = unsafe {& *texture_description};
        // TODO: This really isn't safe. I want to replace this with a proper conversion.
        let texture_format = unsafe { std::mem::transmute::<u32, DxgiFormat>(texture_description.nFormat) };
        let texture_format = match texture_format {
            DxgiFormat::UNKNOWN => Format::UNDEFINED,
            _ => todo!("Format not currently supported. {texture_format:?}"),
        };

        let image_create_info = ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: texture_format,
                extent: [texture_description.nWidth, texture_description.nHeight, 1],
                samples: texture_description.nSampleCount.try_into().unwrap(),
                usage: ImageUsage::TRANSFER_SRC,
                // We may need to use D3D11_TEXTURE instead. I don't know.
                external_memory_handle_types: ExternalMemoryHandleTypes::empty(),
                ..Default::default()
            };

        let allocation_create_info = AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            };

        let image_handles: [u64; 3] = self.with_rendering(move |rendering, _| {
            array::from_fn(|_| Image::new(rendering.context.memory_allocator().clone(), image_create_info.clone(), allocation_create_info.clone()).unwrap().handle().as_raw())
        });

        let swap_texture_set = IVRDriverDirectModeComponent_SwapTextureSet_t {
            rSharedTextureHandles: image_handles,
            unTextureFlags: 0,
        };
        unsafe { *out_swap_texture_set = swap_texture_set };
    }
}

pub struct Rendering {
    context: VulkanoContext,
    windows: VulkanoWindows,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,

    with_event_loop: Option<WithEventLoop>,
}

struct WithEventLoop {
    render_pass: Arc<RenderPass>,
    framebuffers: Vec<Arc<Framebuffer>>,
    pipeline: Arc<GraphicsPipeline>,
    viewport: Viewport,
}

impl ApplicationHandler<UserEvent> for Rendering {
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        event(self, event_loop);
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
        if !matches!(cause, StartCause::Init) {
            return;
        }

        let window_descriptor = WindowDescriptor {
            ..Default::default()
        };

        let left_eye = self.windows.create_window(event_loop, &self.context, &window_descriptor, |_| {});
        let right_eye = self.windows.create_window(event_loop, &self.context, &window_descriptor, |_| {});

        let window_renderer = self.windows.get_primary_renderer().expect("We just created two windows. There will be a window renderer.");

        let render_pass = single_pass_renderpass!(
            self.context.device().clone(),
            attachments: {
                color: {
                    format: window_renderer.swapchain_format(),
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                },
            },
            pass: {
                color: [color],
                depth_stencil: {},
            },
        ).unwrap();
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        
    }

    fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        
    }
}