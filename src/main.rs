extern crate minifb;

use minifb::{Key, Window, WindowOptions, MouseButton};

extern crate hyber;
use hyber::Renderer;
use hyber::Event;
use hyber::Queue;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

struct Rendererxpto{
    
}

enum MessageXPTO{
    Abc,
    Dfg
}

enum EventClient{
    left_click,
    right_click
}

impl hyber::Renderer<Window,EventClient> for Rendererxpto{
    type Message = MessageXPTO;
    fn map_events(eventClient: EventClient) -> hyber::Event { 
        match eventClient {
            left_click => hyber::Event::Mouse(hyber::Mouse::ButtonPressed(hyber::MouseButton::Left)),
            right_click => hyber::Event::Mouse(hyber::Mouse::ButtonPressed(hyber::MouseButton::Right))
        }
    }
    fn detect_sys_events(queue: &mut hyber::Queue<hyber::Event>,window: &mut Window) {
        if(window.is_open() && !window.is_key_down(Key::Escape)){
            if window.get_mouse_down(minifb::MouseButton::Left) {
                let event = Rendererxpto::map_events(EventClient::left_click);
                queue.enqueue(event);
            }
        }
        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

}

fn main() {
    
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let mut render = Rendererxpto{};
    let events = render.create_events_queue();
    let messages = render.create_message_queue();
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    render.event_loop(events, messages,&mut window);
    
    // Limit to max ~60 fps update rate
    /*while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.get_mouse_down(minifb::MouseButton::Left) {
            let event = Rendererxpto::map_events(EventoCliente::left_click);
            queue.enqueue(event);
        }*/
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    }