extern crate minifb;
use minifb::{Key, Window, WindowOptions, MouseButton};

mod lib;
pub use lib::Renderer;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

pub struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T){
        self.queue.push(item)
    }

    pub fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }

    pub fn lenght(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    ///remove the first
    pub fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
}

struct Rendererxpto{
    
}

enum EventoCliente{
    left_click
}

impl Renderer for Rendererxpto{
    
    fn map_events<EventoCliente>(event: EventoCliente) -> lib::Event { 
        match event {
            left_click => lib::Event::Mouse(lib::Mouse::ButtonPressed(lib::MouseButton::Left))
        }
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
    
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut queue: Queue<lib::Event> = Queue::new();  
    let renderer = Rendererxpto{};
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }
        if window.get_mouse_down(minifb::MouseButton::Left) {
            let event = Rendererxpto::map_events(EventoCliente::left_click);
            queue.enqueue(event);
        }
        if queue.lenght() != 0{
            println!("OLAAAAAAA");
            queue.dequeue();
        }
        
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
