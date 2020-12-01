use std::collections::BTreeMap;
use raqote::*;
use euclid::{Point2D};

/// Enumeration with the Render Instructions
#[derive(Copy, Clone, Debug)]
pub enum RenderInstruction {
    /// Instruction to the Render that a point needs to be drawn on the next Clipping
    /// The point should be rendered on absolute coordinates (x,y)
	DrawPoint {
        point: Point2D<f32,f32>,
    },
    
    /// Instruction to the Render that a line needs to be drawn on the next Clipping
    /// The line should be rendered on absolute coordinates from (x1, y1) to (x2, y2)
	DrawLine {
        pointA: Point2D<f32,f32>,
        pointB: Point2D<f32,f32>,
    },

    /// Instruction to the Render that an arc needs to be drawn on the next Clipping
    /// The arc should be rendered with center on absolute coordinates (x, y), 'r'
    /// radius, 'sang' start angle and 'eang' end angle
    DrawArc {
        point: Point2D<f32,f32>,
        r: f32,
        sang: f32,
        eang: f32,
    },

    /// Instruction to the Render that a circle needs to be drawn on the next Clipping
    /// The circle should be rendered with center on absolute coordinates (x, y) and 'r'
    /// radius
    DrawCircle {
        point: Point2D<f32,f32>,
        r: f32,
    },
    /// Instruction to the Render that a rectangle needs to be drawn on the next Clipping
    /// The rectangle should be rendered on absolute coordinates (x, y) with 'l' length
    /// and 'w' width
	DrawRect {
        point: Point2D<f32,f32>,
        length: u32,
        width: u32,
    },

    /// Instruction to the Render that a triangle needs to be drawn on the next Clipping
    /// The triangle should be rendered between the absolute coordinates (x1, y1),
    /// (x2, y2) and (x3, y3)
    DrawTriangle {
        pointA: Point2D<f32,f32>,
        pointB: Point2D<f32,f32>,
        pointC: Point2D<f32,f32>,
    },

    /// Instruction to the Render that an image needs to be drawn on the next Clipping
    /// [Doubt] The image should be rendered with center on the absolute coordinates (x, y)
    /// and with 'w' width and 'l' length
    DrawImage {
        point: Point2D<f32,f32>,
    },

    /// Instruction to the Render that some text needs to be drawn on the next Clipping
    /// [Doubt] The text should be rendered according to the text_alignment
    DrawText {
        point: Point2D<f32,f32>,
    },
}
/// Assumptions:
///     - 2D Meshes are compounded by a list of triangles so the instructions are gonna be
///     multiple DrawTriangleAbs intructions
///     Reference: https://github.com/hecrj/iced/blob/master/graphics/src/triangle.rs
///     - Based on: https://en.wikipedia.org/wiki/Geometric_primitive
///     - And on:   https://www.freepascal.org/docs-html/current/rtl/graph/funcdrawing.html

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

/// Structure that represents the collection of Render Instructions to be 
/// rendered each frame
#[derive(Debug)]
pub struct RenderInstructionCollection<'a> {
    pub instructions: &'a BTreeMap<u32, Vec<RenderInstruction>>
}
// Assumptions for the map:
//  - Need to have a key-value pair of <u32, RenderInstruction>/<id, RenderInstruction>
// Requirements:
//  - Fast iterator, due to client requirements of rendering
// 
// BTreeMap is the choice because of our use case:
//     - You want a map sorted by its keys.
//
// References: 
//     - https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
//     - https://doc.rust-lang.org/std/collections/index.html

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

/*struct BoxLayout {
    min_x: unimplemented!(),
    max_x: unimplemented!(),
    min_y: unimplemented!(),
    max_y: unimplemented!()
}*/

struct SliverLayout {
}

pub trait Widget {

}


/// Implements the methods for the rendering primitives
pub trait Primitives {
    fn draw(&mut self, instruction: RenderInstruction);
    fn DrawPoint(&mut self, point: Point2D<f32,f32>);
    fn DrawLine(&mut self, pointA: Point2D<f32,f32>, pointB: Point2D<f32,f32>);
    fn DrawArc(&mut self, point: Point2D<f32,f32>, r: f32, sang: f32, eang: f32);
    fn DrawCircle(&mut self, point: Point2D<f32,f32>, r: f32);
    fn DrawRect(&mut self, point: Point2D<f32,f32>, l: u32, w: u32);
    fn DrawTriangle(&mut self, pointA: Point2D<f32,f32>, pointB: Point2D<f32,f32>, pointC: Point2D<f32,f32>);
    fn DrawImage(&mut self, point: Point2D<f32,f32>);
    fn DrawText(&mut self, point: Point2D<f32,f32>);
}

pub struct Raqote {
    pub pb: PathBuilder,
}

impl Primitives for Raqote {

    fn draw(&mut self, instruction: RenderInstruction) {
        match instruction {
            RenderInstruction::DrawPoint {point}                        =>  self.DrawPoint(point),
            RenderInstruction::DrawLine {pointA, pointB}                =>  self.DrawLine(pointA, pointB),
            RenderInstruction::DrawArc {point, r, sang, eang}           =>  self.DrawArc(point, r, sang, eang),
            RenderInstruction::DrawCircle {point, r}                    =>  self.DrawCircle(point, r),
            RenderInstruction::DrawRect {point, length, width}          =>  self.DrawRect(point, length, width),
            RenderInstruction::DrawTriangle {pointA, pointB, pointC}    =>  self.DrawTriangle(pointA, pointB, pointC),
            RenderInstruction::DrawImage {point}                        =>  self.DrawImage(point),
            RenderInstruction::DrawText {point}                         =>  self.DrawText(point),
        }
    }
    /// Instruction to the Render that a point needs to be drawn on the next Clipping
    /// The point should be rendered on absolute coordinates (x,y)
    fn DrawPoint(&mut self, point: Point2D<f32,f32>) {
        
    }
    /// Instruction to the Render that a line needs to be drawn on the next Clipping
    /// The line should be rendered on absolute coordinates from (x1, y1) to (x2, y2)
	fn DrawLine(&mut self, pointA: Point2D<f32,f32>, pointB: Point2D<f32,f32>) {
        self.pb.move_to(pointA.x, pointA.y);
        self.pb.line_to(pointB.x,pointB.y);
    }
    /// Instruction to the Render that an arc needs to be drawn on the next Clipping
    /// The arc should be rendered with center on absolute coordinates (x, y), 'r'
    /// radius, 'sang' start angle and 'eang' end angle
    fn DrawArc(&mut self, point: Point2D<f32,f32>, r: f32, sang: f32, eang: f32) {
        self.pb.arc(point.x, point.y, r, sang, eang)
    }
    /// Instruction to the Render that a circle needs to be drawn on the next Clipping
    /// The circle should be rendered with center on absolute coordinates (x, y) and 'r'
    /// radius
    fn DrawCircle(&mut self, point: Point2D<f32,f32>, r: f32) {

    }
    /// Instruction to the Render that a rectangle needs to be drawn on the next Clipping
    /// The rectangle should be rendered on absolute coordinates (x, y) with 'l' length
    /// and 'w' width
	fn DrawRect(&mut self, point: Point2D<f32,f32>, l: u32, w: u32) {
        self.pb.rect(point.x, point.y, l as f32, w as f32);
    }
    /// Instruction to the Render that a triangle needs to be drawn on the next Clipping
    /// The triangle should be rendered between the absolute coordinates (x1, y1),
    /// (x2, y2) and (x3, y3)
    fn DrawTriangle(&mut self, pointA: Point2D<f32,f32>, pointB: Point2D<f32,f32>, pointC: Point2D<f32,f32>) {
        self.pb.move_to(pointA.x, pointA.y);
        self.pb.line_to(pointB.x,pointB.y);
        self.pb.line_to(pointC.x,pointC.y);
        self.pb.line_to(pointA.x,pointA.y);
    }
    /// Instruction to the Render that an image needs to be drawn on the next Clipping
    /// [Doubt] The image should be rendered with center on the absolute coordinates (x, y)
    /// and with 'w' width and 'l' length
    fn DrawImage(&mut self, point: Point2D<f32,f32>) {
        
    }
    /// Instruction to the Render that some text needs to be drawn on the next Clipping
    /// [Doubt] The text should be rendered according to the text_alignment
    fn DrawText(&mut self, point: Point2D<f32,f32>) {

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

    #[test]
    fn test() {
        use raqote::*;
        use euclid::{Point2D};
        use super::*;
        let mut dt = DrawTarget::new(400, 400);
        let mut pb = PathBuilder::new();
        let mut raqote = Raqote {pb: PathBuilder::new()};

        let instruction = RenderInstruction::DrawLine { pointA: Point2D::new(10., 10.), pointB: Point2D::new(50., 50.)};

        raqote.draw(instruction);

        //pb.move_to(10., 10.);
        //pb.line_to(50.,50.);
        
        raqote.pb.close();
        let path = raqote.pb.finish();
        dt.stroke(
            &path,
            &Source::Solid(SolidSource {
                r: 0x80,
                g: 0x80,
                b: 0x80,
                a: 0x80,
            }),
            &StrokeStyle {
                cap: LineCap::Round,
                join: LineJoin::Round,
                width: 10.,
                miter_limit: 2.,
                dash_array: vec![10., 18.],
                dash_offset: 16.,
            },
            &DrawOptions::new()
        );
        dt.write_png("result.png");
    }
}

