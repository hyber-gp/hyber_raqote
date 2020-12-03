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
use hyber::{RenderInstruction, RenderInstructionCollection};
use std::collections::BTreeMap;
use std::vec::Vec;
use euclid::{Point2D};
use raqote::Color;

// Method to simulate an iteration over the Render Instructions on the Collection
fn renderer (render: &BTreeMap<u32, Vec<RenderInstruction>>){
    // Create a Render Instruction Collection with the previous BTreeMap
    let _collection = RenderInstructionCollection{ instructions: render };

    println!();
    println!("Collection of Render Instructions:");
    println!();

    // Loop over the instructions collection
    // Simulates the process of rendering a new frame
    for (key, value) in _collection.instructions {

        println!();
        println!("For KEY: {:?}", key);
        println!();

        for x in value.iter(){
            println!("{:?}", x);
        }
    }

    println!();
}

// Method to simulate the creation of frame 1
fn frame_1(render: &mut BTreeMap<u32, Vec<RenderInstruction>>){
    // Initialize a new vector to hold instructions
    // The vector will contain the widget's instructions list
    // This widget will have ID = 1
    let widget_1_id = 1;
    let mut widget_1_instructions = Vec::new();
    // This widget will have ID = 2
    let widget_2_id = 2;
    let mut widget_2_instructions = Vec::new();
    // This widget will have ID = 3
    let widget_3_id = 3;
    let mut widget_3_instructions = Vec::new();
    // This widget will have ID = 4
    let widget_4_id = 4;
    let mut widget_4_instructions = Vec::new();

    // Add instructions to the widget's vectors
    widget_1_instructions.push(RenderInstruction::DrawRect{point: Point2D::new(0.0, 0.0), length: 0, width: 0, color: Color::new(0xff, 0xff, 0xff, 0xff)});
    widget_1_instructions.push(RenderInstruction::DrawText{point: Point2D::new(0.0, 0.0)});

    widget_2_instructions.push(RenderInstruction::DrawLine{pointA: Point2D::new(0.0, 0.0), pointB: Point2D::new(0.0, 0.0), color: Color::new(0xff, 0xff, 0xff, 0xff)});

    widget_3_instructions.push(RenderInstruction::DrawImage{point: Point2D::new(0.0, 0.0)});

    widget_4_instructions.push(RenderInstruction::DrawCircle{point: Point2D::new(0.0, 0.0), r: 0.0, color: Color::new(0xff, 0xff, 0xff, 0xff)});
    widget_4_instructions.push(RenderInstruction::DrawText{point: Point2D::new(0.0, 0.0)});

    // Insert those widget's instructions on the collection
    // This simulates the process of sending all the new instructions to the collection
    render.insert(widget_1_id, widget_1_instructions);
    render.insert(widget_2_id, widget_2_instructions);
    render.insert(widget_3_id, widget_3_instructions);
    render.insert(widget_4_id, widget_4_instructions);
}

// Method to simulate the creation of frame 2
fn frame_2(render: &mut BTreeMap<u32, Vec<RenderInstruction>>){
    // This widget will have ID = 2
    let widget_2_id = 2;
    let mut widget_2_instructions = Vec::new();

    // Simulate some change on some widget
    widget_2_instructions.push(RenderInstruction::DrawRect{point: Point2D::new(0.0, 0.0), length: 0, width: 0, color: Color::new(0xff, 0xff, 0xff, 0xff)});
    widget_2_instructions.push(RenderInstruction::DrawText{point: Point2D::new(0.0, 0.0)});

    // Insert those widget's instructions on the collection
    // This simulates the process of sending all the new instructions to the collection
    render.insert(widget_2_id, widget_2_instructions);
}

fn main() {
    // Create a BTreeMap to set a Render Instruction Collection
    let mut render_instructions = BTreeMap::new();

    frame_1(&mut render_instructions);

    // Loop over the instructions collection
    // Simulates the process of rendering a new frame
    renderer(&render_instructions);
    
    frame_2(&mut render_instructions);

    println!();
    println!("WIDGET 2 updated...");
    println!("New instructions added...");
    println!();

    // Loop over the instructions collection
    // Simulates the process of rendering a new frame
    renderer(&render_instructions);
}
