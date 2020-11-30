use std::collections::BTreeMap;
use raqote::*;

/// Enumeration with the Render Instructions @joaosantos
pub enum RenderInstruction {
    /// Instruction to the Render that a point needs to be drawn on the next Clipping
    /// The point should be rendered on absolute coordinates (x,y)
	DrawPoint,
    /// Instruction to the Render that a line needs to be drawn on the next Clipping
    /// The line should be rendered on absolute coordinates from (x1, y1) to (x2, y2)
	DrawLine,
    /// Instruction to the Render that an arc needs to be drawn on the next Clipping
    /// The arc should be rendered with center on absolute coordinates (x, y), 'r'
    /// radius, 'sang' start angle and 'eang' end angle
    DrawArc,
    /// Instruction to the Render that a circle needs to be drawn on the next Clipping
    /// The circle should be rendered with center on absolute coordinates (x, y) and 'r'
    /// radius
    DrawCircle,
    /// Instruction to the Render that a rectangle needs to be drawn on the next Clipping
    /// The rectangle should be rendered on absolute coordinates (x, y) with 'l' length
    /// and 'w' width
	DrawRect,
    /// Instruction to the Render that a triangle needs to be drawn on the next Clipping
    /// The triangle should be rendered between the absolute coordinates (x1, y1),
    /// (x2, y2) and (x3, y3)
    DrawTriangle,
    /// Instruction to the Render that an image needs to be drawn on the next Clipping
    /// [Doubt] The image should be rendered with center on the absolute coordinates (x, y)
    /// and with 'w' width and 'l' length
    DrawImage,
    /// Instruction to the Render that some text needs to be drawn on the next Clipping
    /// [Doubt] The text should be rendered according to the text_alignment
    DrawText,
}
/// Assumptions:
///     - 2D Meshes are compounded by a list of triangles so the instructions are gonna be
///     multiple DrawTriangleAbs intructions
///     Reference: https://github.com/hecrj/iced/blob/master/graphics/src/triangle.rs
///     - Based on: https://en.wikipedia.org/wiki/Geometric_primitive
///     - And on:   https://www.freepascal.org/docs-html/current/rtl/graph/funcdrawing.html

// Structure of an Instruction to be on the Render Instrucstions Collection
pub struct Instruction {
    pub id: u32,
    pub instruction: RenderInstruction,
}

/// Implements the method for a new Instruction
impl Instruction {
    pub fn new(id: u32, instruction: RenderInstruction) -> Instruction {
        Instruction {
            id,
            instruction,
        }
    }
}

// Example:
//
// Criar:
//     (-> BTreeMap<K, V>)
//     - makes a new empty BTreeMap.
//
//     let mut map = BTreeMap::new();
//
// Limpar:
//     - clears the map, removing all elements
//
//     map.clear();
//
// Get Value:
//     (-> Option<&V>)
//     - returns a reference to the value corresponding to the key
//
//     map.get(&1);
//
// Get Key-Value:
//     (-> Option<(&K, &V)>)
//     - returns the key-value pair corresponding to the supplied key
//
//     map.get_key_value(&1);
//
// Get Mutable Value:
//     (-> Option<&mut V>)
//     - returns a mutable reference to the value corresponding to the key.
//
//     map.get_mut(&1);
//
// Contains Key:
//     (bool)
//     - returns true if the map contains a value for the specified key.
//
//     map.contains_key(&1);
//
// First Key-Value:
//     (-> Option<(&K, &V)>)
//     - to obtain the first key-value pair in the map
//
//     map.first_key_value();
//
// Insertion:
//     (-> Option<V>)
//     - inserts a key-value pair into the map
//
//     map.insert(1, RenderInstruction::DrawPoint);
//
// Remove:
//     (-> Option<V>)
//     - removes a key from the map, returning the value at the key
//     - if the key was previously in the map
//
//     map.remove(&1);
//
// Remove Entry:
//     (-> Option<(K, V)>)
//     - removes a key from the map, returning the stored key and value
//     - if the key was previously in the map
//
//     map.remove_entry(&1);

//let mut instruction = BTreeMap::new();

/// Assumptions for the map:
///  - Need to have a key-value pair of <u32, RenderInstruction>/<id, RenderInstruction>
/// Requirements:
///  - Fast iterator, due to client requirements of rendering
/// 
/// BTreeMap is the choice because of our use case:
///     - You want a map sorted by its keys.
///
/// References: 
///     - https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.remove
///     - https://doc.rust-lang.org/std/collections/index.html#use-a-btreemap-when


pub enum Event {
    //TODO: Rato, teclado, window
}

impl Event {
    // TODO: funcoes
    fn mouse_click() {
        unimplemented!();
    }

}

pub trait Display {
    
}

struct BoxLayout {
    min_x: unimplemented!(),
    max_x: unimplemented!(),
    min_y: unimplemented!(),
    max_y: unimplemented!()
}

struct SliverLayout {
}

pub trait Widget {

}

pub struct Point {
    x: f32,
    y: f32,
}
/// Implements the method for a new point
impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point {
            x,
            y,
        }
    }
}

/// Implements the methods for the rendering primitives
pub trait Primitives {
    fn draw(&mut self, instruction: RenderInstruction, buffer: Buffer);
    fn DrawPoint(&mut self, point: Point, buffer: Buffer) -> Buffer;
    fn DrawLine(&mut self, pointA: Point, pointB: Point, buffer: Buffer) -> Buffer;
    fn DrawArc(&mut self, point: Point, r: f32, sang: f32, eang: f32, buffer: Buffer) -> Buffer;
    fn DrawCircle(&mut self, point: Point, r: f32, buffer: Buffer) -> Buffer;
    fn DrawRect(&mut self, point: Point, l: u32, w: u32, buffer: Buffer) -> Buffer;
    fn DrawTriangle(&mut self, pointA: Point, pointB: Point, pointC: Point, buffer: Buffer) -> Buffer;
    fn DrawImage(&mut self, point: Point,buffer: Buffer) -> Buffer;
    fn DrawText(&mut self, point: Point,buffer: Buffer) -> Buffer;
}

pub struct Raqote {
    pub pb: PathBuilder,
}

impl Primitives for Raqote {

    fn draw(&mut self, instruction: RenderInstruction, buffer: Buffer) -> Buffer {
        match instruction {
            DrawPoint   =>  self.DrawPoint(Point {x: 0., y: 10.}, buffer),
            DrawLine    =>  self.DrawLine(Point {x: 0., y: 10.}, Point {x:50., y:100.}, buffer),
            DrawArc     =>  self.DrawArc(),
            DrawCircle  =>  self.DrawCircle(),
            DrawRect    =>  self.DrawRect(),
            DrawTriangle=>  self.DrawTriangle(),
            DrawImage   =>  self.DrawImage(),
            DrawText    =>  self.DrawText(),
        }
    }
    /// Instruction to the Render that a point needs to be drawn on the next Clipping
    /// The point should be rendered on absolute coordinates (x,y)
    fn DrawPoint(&mut self, point: Point, buffer: Buffer) -> Buffer {
        
    }
    /// Instruction to the Render that a line needs to be drawn on the next Clipping
    /// The line should be rendered on absolute coordinates from (x1, y1) to (x2, y2)
	fn DrawLine(&mut self, pointA: Point, pointB: Point, buffer: Buffer) -> Buffer {
        self.pb.move_to(pointA.x, pointA.y);
        self.pb.line_to(pointB.x,pointB.y);
        
    }
    /// Instruction to the Render that an arc needs to be drawn on the next Clipping
    /// The arc should be rendered with center on absolute coordinates (x, y), 'r'
    /// radius, 'sang' start angle and 'eang' end angle
    fn DrawArc(&mut self, point: Point, r: f32, sang: f32, eang: f32, buffer: Buffer) -> Buffer {
        self.pb.arc(point.x, point.y, r, sang, eang)
    }
    /// Instruction to the Render that a circle needs to be drawn on the next Clipping
    /// The circle should be rendered with center on absolute coordinates (x, y) and 'r'
    /// radius
    fn DrawCircle(&mut self, point: Point, r: u32, buffer: Buffer) -> Buffer {

    }
    /// Instruction to the Render that a rectangle needs to be drawn on the next Clipping
    /// The rectangle should be rendered on absolute coordinates (x, y) with 'l' length
    /// and 'w' width
	fn DrawRect(&mut self, point: Point, l: u32, w: u32, buffer: Buffer) -> Buffer {
        self.pb.rect(point.x, point.y, l as f32, w as f32);
    }
    /// Instruction to the Render that a triangle needs to be drawn on the next Clipping
    /// The triangle should be rendered between the absolute coordinates (x1, y1),
    /// (x2, y2) and (x3, y3)
    fn DrawTriangle(&mut self, pointA: Point, pointB: Point, pointC: Point, buffer: Buffer) -> Buffer {
        self.pb.move_to(pointA.x, pointA.y);
        self.pb.line_to(pointB.x,pointB.y);
        self.pb.line_to(pointC.x,pointC.y);
        self.pb.line_to(pointA.x,pointA.y);
    }
    /// Instruction to the Render that an image needs to be drawn on the next Clipping
    /// [Doubt] The image should be rendered with center on the absolute coordinates (x, y)
    /// and with 'w' width and 'l' length
    fn DrawImage(&mut self, point: Point,buffer: Buffer) -> Buffer {
        
    }
    /// Instruction to the Render that some text needs to be drawn on the next Clipping
    /// [Doubt] The text should be rendered according to the text_alignment
    fn DrawText(&mut self, point: Point,buffer: Buffer) -> Buffer {

    }
}

pub trait Renderer {
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

