use cfg_if::cfg_if;
use log::Log;
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
// use winit::{event::{ElementState, Event, KeyEvent, WindowEvent}, event_loop::EventLoop, window::Window};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
};
use std::{cell::RefCell, time::Duration};

use crate::{
    // application::Application,
    // state::*,
    window::*,
};

/// The Engine that drives the game and application
pub struct Engine<'a> {
    logger: env_logger::Logger,
    // app: Application,
    // state: Option<State<'a>>,
    window: Option<RefCell<Window<'a>>>
}


impl<'a> Engine<'a> {
    /// Create a new Velvet3D Engine
    pub fn new() -> Self {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                // wasm logger
            } else {
                env_logger::init();
            }
        }

        Self {
            logger: env_logger::Logger::from_default_env(),
            // app: Application::default(),
            // state: None,
            window: None,
        }
    }

    /// Create a window for the engine to render to
    pub fn create_window(&mut self, name: &'a str, width: i32, height: i32) {
        self.window = Some(
            RefCell::new(Window::new(name, width, height))
        );
    }

    pub fn run(&mut self) {
        let mut window = match self.window {
            Some(ref window) => {
                window.borrow_mut()
            }
            
            None => {
                println!("FOR NOW UNABLE TO RUN APPLICATION WITHOUT WINDOW");
                return;
            }
        };

        let mut event_loop = window.create_event_loop();
        let canvas = window.canvas();

        'running: loop {
            for event in event_loop.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            canvas.clear();
            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
            // The rest of the game loop goes here...
        }
        

        // let event_loop = EventLoop::new().unwrap();
        // event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        // event_loop.run_app(&mut self.app).unwrap();
    }
}