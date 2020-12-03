use hyber;
use hyber::renderer::RenderInstruction;
use hyber::renderer::RenderInstructionCollection;

use euclid::Point2D;
use raqote::Color;
use std::collections::BTreeMap;

// Method to simulate an iteration over the Render Instructions on the Collection
fn renderer(render: &BTreeMap<u32, Vec<RenderInstruction<Point2D<f32, f32>, Color>>>) {
    // Create a Render Instruction Collection with the previous BTreeMap
    let _collection = RenderInstructionCollection {
        instructions: render,
    };

    println!();
    println!("Collection of Render Instructions:");
    println!();

    // Loop over the instructions collection
    // Simulates the process of rendering a new frame
    for (key, value) in _collection.instructions {
        println!();
        println!("For KEY: {:?}", key);
        println!();

        for x in value.iter() {
            println!("{:?}", x);
        }
    }

    println!();
}

// Method to simulate the creation of frame 1
fn frame_1(render: &mut BTreeMap<u32, Vec<RenderInstruction<Point2D<f32, f32>, Color>>>) {
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
    widget_1_instructions.push(RenderInstruction::DrawRect {
        point: Point2D::new(0.0, 0.0),
        length: 0,
        width: 0,
        color: Color::new(0xff, 0xff, 0xff, 0xff),
    });
    widget_1_instructions.push(RenderInstruction::DrawText {
        point: Point2D::new(0.0, 0.0),
    });

    widget_2_instructions.push(RenderInstruction::DrawLine {
        point_a: Point2D::new(0.0, 0.0),
        point_b: Point2D::new(0.0, 0.0),
        color: Color::new(0xff, 0xff, 0xff, 0xff),
    });

    widget_3_instructions.push(RenderInstruction::DrawImage {
        point: Point2D::new(0.0, 0.0),
    });

    widget_4_instructions.push(RenderInstruction::DrawCircle {
        point: Point2D::new(0.0, 0.0),
        r: 0.0,
        color: Color::new(0xff, 0xff, 0xff, 0xff),
    });
    widget_4_instructions.push(RenderInstruction::DrawText {
        point: Point2D::new(0.0, 0.0),
    });

    // Insert those widget's instructions on the collection
    // This simulates the process of sending all the new instructions to the collection
    render.insert(widget_1_id, widget_1_instructions);
    render.insert(widget_2_id, widget_2_instructions);
    render.insert(widget_3_id, widget_3_instructions);
    render.insert(widget_4_id, widget_4_instructions);
}

// Method to simulate the creation of frame 2
fn frame_2(render: &mut BTreeMap<u32, Vec<RenderInstruction<Point2D<f32, f32>, Color>>>) {
    // This widget will have ID = 2
    let widget_2_id = 2;
    let mut widget_2_instructions = Vec::new();

    // Simulate some change on some widget
    widget_2_instructions.push(RenderInstruction::DrawRect {
        point: Point2D::new(0.0, 0.0),
        length: 0,
        width: 0,
        color: Color::new(0xff, 0xff, 0xff, 0xff),
    });
    widget_2_instructions.push(RenderInstruction::DrawText {
        point: Point2D::new(0.0, 0.0),
    });

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
