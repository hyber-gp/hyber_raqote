extern crate minifb;

use minifb::{Key, MouseButton, Window, WindowOptions};

extern crate hyber;
use hyber::Event;
use hyber::Queue;
use hyber::Display;
use hyber::Renderer;

// use std::os::raw; for window handle

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

struct Rendererxpto {}

struct DisplayMinifb {
    display: minifb::Window,
}

impl hyber::Display for DisplayMinifb {
    type Buffer = Vec<u32>;

    fn new(
        title: &str,
        width: usize,
        height: usize,
        display_descriptor: hyber::DisplayDescritor,
    ) -> Self {
        match minifb::Window::new(
            title,
            width,
            height,
            minifb::WindowOptions {
                borderless: display_descriptor.border,
                title: display_descriptor.titled,
                resize: display_descriptor.resizable,
                topmost: display_descriptor.topmost,
                scale: minifb::Scale::X1,
                scale_mode: minifb::ScaleMode::UpperLeft,
                transparency: false,
            },
        ) {
            Ok(display) => DisplayMinifb { display: display },
            Err(_) => panic!(),
        }
    }

    fn set_title(&mut self, title: &str) {
        self.display.set_title(title);
    }
    
    fn update(&mut self) {
        self.display.update();
    }

    fn update_with_buffer(&mut self, buffer: &Self::Buffer, width: usize, height: usize) {
        match self.display.update_with_buffer(buffer, width, height) {
            Ok(_) => (),
            Err(e) => {panic!("{:?}", e)}
        }
    }

    fn is_open(&self) -> bool {
        self.display.is_open()
    }

    fn set_position(&mut self, x: usize, y: usize) {
        self.display.set_position(x as isize, y as isize);
    }
    
    fn border(&mut self, border: bool) {
        unimplemented!();
    }

    fn resizable(&mut self, resizable: bool) {
        unimplemented!();
    }

    fn topmost(&mut self, topmost: bool) {
        self.display.topmost(topmost);
    }

    fn minimizable(&mut self, minimizable: bool) {
        unimplemented!();
    }

    fn set_background_color(&mut self, red: usize, green: usize, blue: usize) {
        self.display.set_background_color(red, green, blue);
    }
    
    fn get_size(&self) -> (usize, usize) {
        self.display.get_size()
    }

    fn is_active(&mut self) -> bool {
        self.display.is_active()
    }
}

enum MessageXPTO {
    Abc,
    Dfg,
}

enum EventClient {
    LeftClick,
    RightClick,
}

impl hyber::Renderer<DisplayMinifb, EventClient> for Rendererxpto {
    type Message = MessageXPTO;
    fn map_events(event_client: EventClient) -> hyber::Event {
        match event_client {
            LeftClick => {
                hyber::Event::Mouse(hyber::Mouse::ButtonPressed(hyber::MouseButton::Left))
            }
            RightClick => {
                hyber::Event::Mouse(hyber::Mouse::ButtonPressed(hyber::MouseButton::Right))
            }
        }
    }
    
    fn detect_sys_events(queue: &mut hyber::Queue<hyber::Event>, system: &mut DisplayMinifb) {
        if system.is_open() && !system.display.is_key_down(Key::Escape) {
            if system.display.get_mouse_down(minifb::MouseButton::Left) {
                let event = Rendererxpto::map_events(EventClient::LeftClick);
                queue.enqueue(event);
            }
        }
        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
        system
            .update_with_buffer(&buffer, WIDTH, HEIGHT);
    }
}

fn main() {
    let mut display = DisplayMinifb::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        hyber::DisplayDescritor::default(),
    );
    let mut render = Rendererxpto {};
    let events = render.create_events_queue();
    let messages = render.create_message_queue();
    display.display.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    render.event_loop(events, messages, &mut display);
    // Limit to max ~60 fps update rate
    /*while window.is_open() && !window.is_key_down(Key::Escape) {
    if window.get_mouse_down(minifb::MouseButton::Left) {
        let event = Rendererxpto::map_events(EventoCliente::left_click);
        queue.enqueue(event);
    }*/
    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
}
