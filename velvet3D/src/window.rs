use core::fmt;

use sdl2::{
    render::Canvas,
    EventPump,
};

#[derive(Debug)]
pub struct Window<'a> {
    name: &'a str,
    curr_width: i32,
    curr_height: i32,
    context: SDLContext,
}

impl<'a> Window<'a> {
    /// Create a new window
    pub fn new(name: &'a str, width: i32, height: i32) -> Self {
        Window {
            name,
            curr_width: width,
            curr_height: height,
            context: SDLContext::new(name, width, height),
        }
    }

    /// Create the Event Loop
    pub fn create_event_loop(&self) -> EventPump {
        return self.context.create_event_loop();
    }

    /// Get a mutable reference to the [Window]'s canvas
    pub fn canvas(&mut self) -> &mut Canvas<sdl2::video::Window> {
        return self.context.canvas_mut();
    }
}

pub struct SDLContext {
    context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    canvas: Canvas<sdl2::video::Window>,
}

impl SDLContext {
    pub fn new(name: &str, width: i32, height: i32) -> Self {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem.window(name, 640, 480)
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let mut canvas = window.clone().into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        Self {
            context,
            video_subsystem,
            window,
            canvas,
        }
    }

    /// Create the event loop from the SDL Context
    pub fn create_event_loop(&self) -> EventPump {
        return self.context.event_pump().expect("Unable to create event loop");
    }

    /// Return a mutable reference to the canvas 
    pub fn canvas_mut(&mut self) -> &mut Canvas<sdl2::video::Window> {
        &mut self.canvas
    }
}

// TODO: figure out what to display for debugging
impl fmt::Debug for SDLContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SDL CONTEXT DEBUG")
    }
}