use winit::{
    dpi::PhysicalSize, event::WindowEvent, window::Window
};

/// Application structure to handle winit windowing
#[derive(Default)]
pub struct Application {
    window: Option<Window>,
}

impl Application {
    pub fn window_surface(&self, instance: wgpu::Instance) -> Result<wgpu::Surface, WindowSurfaceCreationError> {
        let window = match &self.window {
            Some(w) => {
                w
            }
            None => {
                return Err(WindowSurfaceCreationError::NoAvailableWindow);
            }
        };

        let surface = instance.create_surface(window);

        return match surface {
            Ok(s) => Ok(s),
            Err(_) => Err(WindowSurfaceCreationError::InstanceCallFail),
        }
    }

    /// Get the inner size of the window
    pub fn window_size(&self) -> PhysicalSize<u32> {
        let window = &self.window.as_ref().expect("Expected window to be created. Cannot retrieve size from unopened window");

        return window.inner_size();
    }
}

impl winit::application::ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        match self.window {
            Some(ref window) => if window.id() != window_id { return; },
            None => {}
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            _ => {}
        }
    }

    // fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {

    // }

    // fn device_event(
    //         &mut self,
    //         event_loop: &winit::event_loop::ActiveEventLoop,
    //         device_id: winit::event::DeviceId,
    //         event: winit::event::DeviceEvent,
    //     ) {
        
    // }

    // fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        
    // }

    // fn memory_warning(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        
    // }

    // fn new_events(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, cause: winit::event::StartCause) {
        
    // }


    // fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        
    // }

    // fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: ()) {
        
    // }

}

/// Errors that occur when creating the window surface
pub enum WindowSurfaceCreationError {
    /// Occurs when the window has not been created yet
    NoAvailableWindow,

    /// Occurs when using the `create_instance` call fails on the given instance
    InstanceCallFail,

}