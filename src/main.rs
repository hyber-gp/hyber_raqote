use hyber::{RenderInstruction, RenderInstructionCollection, Queue};
use std::collections::BTreeMap;

fn main() {

    let mut render_instructions = BTreeMap::new();

    let mut instructions_queue = Queue::new();

    instructions_queue.enqueue(RenderInstruction::DrawRect);
    instructions_queue.enqueue(RenderInstruction::DrawText);

    render_instructions.insert(1, instructions_queue);

    let mut _collection = RenderInstructionCollection{ instructions: render_instructions };

    println!("Collection of Render Instructions:");
    println!();

    for (key, mut value) in _collection.instructions {
        println!("For KEY: {:?}", key);

        while !value.is_empty() {
            println!("Instruction: {:?}", value.dequeue());
        }
    }
}