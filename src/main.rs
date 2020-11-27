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

enum Message{
    Abc,
    Dfg
}

enum EventClient{
    left_click,
    right_click
}

impl hyber::Renderer for Rendererxpto{
    type Message = Message;
    fn map_events<EventClient>(eventClient: EventClient) -> hyber::Event { 
        match eventClient {
            left_click => hyber::Event::Mouse(hyber::Mouse::ButtonPressed(hyber::MouseButton::Left)),
            right_click => hyber::Event::Mouse(hyber::Mouse::ButtonPressed(hyber::MouseButton::Right))
        }
    }
    fn detect_sys_events(queue: &hyber::Queue<hyber::Event>) {
        queue.enqueue(hyber::Event::Mouse(hyber::Mouse::ButtonPressed(hyber::MouseButton::Left)));
        /*if window.get_mouse_down(minifb::MouseButton::Left) {
            let event = Rendererxpto::map_events(EventoCliente::left_click);
            queue.enqueue(event);
        }*/
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let render = Rendererxpto{};
    //let events = Renderer::create_events_queue();
    //let messages = hyber::Renderer::create_message_queue();
    let events: Queue<hyber::Event> = Queue::new();
    let messages: Queue<Message> = Queue::new();
    Renderer::event_loop(events,messages);
    
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.get_mouse_down(minifb::MouseButton::Left) {
            //let event = Rendererxpto::map_events(EventoCliente::left_click);
            //queue.enqueue(event);
        }
      
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
