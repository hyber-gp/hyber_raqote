use std::collections::BTreeMap;
use raqote::*;
use raqote::Color;
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use euclid::Point2D;

/// Enumeration with the Render Instructions
#[derive(Copy, Clone, Debug)]
pub enum RenderInstruction {
    /// Instruction to the Render that a point needs to be drawn on the next Clipping
    /// The point should be rendered on absolute coordinates (x,y) 
    /// Uses a Color struct using hexadecimal alpha and rgb for coloring
	DrawPoint {
        point: Point2D<f32,f32>,
        color: Color,
    },
    
    /// Instruction to the Render that a line needs to be drawn on the next Clipping
    /// The line should be rendered on absolute coordinates from (x1, y1) to (x2, y2)
    /// Uses a Color struct using hexadecimal alpha and rgb for coloring
	DrawLine {
        pointA: Point2D<f32,f32>,
        pointB: Point2D<f32,f32>,
        color: Color,
    },

    /// Instruction to the Render that an arc needs to be drawn on the next Clipping
    /// The arc should be rendered with center on absolute coordinates (x, y), 'r'
    /// radius, 'sang' start angle and 'eang' end angle.
    /// Uses a Color struct using hexadecimal alpha and rgb for coloring
    DrawArc {
        point: Point2D<f32,f32>,
        r: f32,
        sang: f32,
        eang: f32,
        color: Color,
    },

    /// Instruction to the Render that a circle needs to be drawn on the next Clipping
    /// The circle should be rendered with center on absolute coordinates (x, y) and 'r'
    /// radius
    /// Uses a Color struct using hexadecimal alpha and rgb for coloring
    DrawCircle {
        point: Point2D<f32,f32>,
        r: f32,
        color: Color,
    },
    /// Instruction to the Render that a rectangle needs to be drawn on the next Clipping
    /// The rectangle should be rendered on absolute coordinates (x, y) with 'l' length
    /// and 'w' width
    /// Uses a Color struct using hexadecimal alpha and rgb for coloring
    /// Uses a Color struct using hexadecimal alpha and rgb for coloring
	DrawRect {
        point: Point2D<f32,f32>,
        length: u32,
        width: u32,
        color: Color,
    },

    /// Instruction to the Render that a triangle needs to be drawn on the next Clipping
    /// The triangle should be rendered between the absolute coordinates (x1, y1),
    /// (x2, y2) and (x3, y3)
    /// Uses a Color struct using hexadecimal alpha and rgb for coloring
    DrawTriangle {
        pointA: Point2D<f32,f32>,
        pointB: Point2D<f32,f32>,
        pointC: Point2D<f32,f32>,
        color: Color,
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
    /// Drawing function that takes an instruction to parse and render
    fn draw(&mut self, instruction: RenderInstruction);

    /// Draws a single point at the coordinates given by point with the colors given  
    fn DrawPoint(&mut self, point: Point2D<f32,f32>, color: Color);

    /// Draws a line from pointA to pointB with the given color
    fn DrawLine(&mut self, pointA: Point2D<f32,f32>, pointB: Point2D<f32,f32>, color: Color);

    /// Draws an arc with a center in the given point, with radius r,
    /// sang and eang indicate the starting angle of the arc and the sweeping angle to cut the circle from
    /// essentially a starting angle and an end angle. Also paints it with the given color
    fn DrawArc(&mut self, point: Point2D<f32,f32>, r: f32, sang: f32, eang: f32, color: Color);

    /// Draws a circle at the given coordinates with radius r and paints it a certain color
    fn DrawCircle(&mut self, point: Point2D<f32,f32>, r: f32, color: Color);

    /// Draws a rectangle at the given coordinates with length l and width w and paints it a certain color
    fn DrawRect(&mut self, point: Point2D<f32,f32>, l: u32, w: u32, color: Color);

    /// Draws a triangle with the reference points pointA, pointB and pointC using the given color
    fn DrawTriangle(&mut self, pointA: Point2D<f32,f32>, pointB: Point2D<f32,f32>, pointC: Point2D<f32,f32>, color: Color);
    
    /// Draws an image from memory? with a certain size at the given coordinates
    fn DrawImage(&mut self, point: Point2D<f32,f32>);
    
    /// Draws text on screen given a certain font, text and coordinates
    fn DrawText(&mut self, point: Point2D<f32,f32>);
}

/// A struct to save the drawtarget of raqote and to use as a reference for the primitives trait
pub struct Raqote {
    pub dt: DrawTarget,
}

/// Implements the primitives for the raqote 2D context renderer
impl Primitives for Raqote {
    fn draw(&mut self, instruction: RenderInstruction) {
        match instruction {
            RenderInstruction::DrawPoint {point,color}                          =>  self.DrawPoint(point, color),
            RenderInstruction::DrawLine {pointA, pointB,color}                  =>  self.DrawLine(pointA, pointB, color),
            RenderInstruction::DrawArc {point, r, sang, eang, color}            =>  self.DrawArc(point, r, sang, eang, color),
            RenderInstruction::DrawCircle {point, r, color}                     =>  self.DrawCircle(point, r, color),
            RenderInstruction::DrawRect {point, length, width, color}           =>  self.DrawRect(point, length, width, color),
            RenderInstruction::DrawTriangle {pointA, pointB, pointC, color}     =>  self.DrawTriangle(pointA, pointB, pointC, color),
            RenderInstruction::DrawImage {point}                                =>  self.DrawImage(point),
            RenderInstruction::DrawText {point}                                 =>  self.DrawText(point),
        }
    }

    fn DrawPoint(&mut self, point: Point2D<f32,f32>, color: Color,) {
        // [Doubt] Isn't the point basically a tiny circle? 
    }

	fn DrawLine(&mut self, pointA: Point2D<f32,f32>, pointB: Point2D<f32,f32>, color: Color) {
        let mut pb = PathBuilder::new();
        pb.move_to(pointA.x, pointA.y);
        pb.line_to(pointB.x,pointB.y);
        pb.close();
        
        self.dt.stroke(
            &pb.finish(),
            &Source::Solid(SolidSource::from(color)),
            &StrokeStyle {
                cap: LineCap::Square,
                join: LineJoin::Bevel,
                width: 10.,
                miter_limit: 0.,
                dash_array: vec![1., 1.],
                dash_offset: 0.,
            },
            &DrawOptions::new()
        );
    }

    fn DrawArc(&mut self, point: Point2D<f32,f32>, r: f32, sang: f32, eang: f32, color: Color,) {
        let mut pb = PathBuilder::new();
        pb.move_to(point.x, point.y);
        pb.arc(point.x, point.y, r, sang, eang);

        pb.close();
        self.dt.fill(&pb.finish(), &Source::Solid(SolidSource::from(color)), &DrawOptions::new());
    }

    fn DrawCircle(&mut self, point: Point2D<f32,f32>, r: f32, color: Color,) {
        let mut pb = PathBuilder::new();

        pb.move_to(point.x, point.y);
        pb.arc(point.x, point.y, r, 0., 7.);

        pb.close();
        self.dt.fill(&pb.finish(), &Source::Solid(SolidSource::from(color)), &DrawOptions::new());

    }

	fn DrawRect(&mut self, point: Point2D<f32,f32>, l: u32, w: u32, color: Color,) {
        let mut pb = PathBuilder::new();

        pb.move_to(point.x, point.y);
        pb.rect(point.x, point.y, l as f32, w as f32);

        pb.close();
        self.dt.fill(&pb.finish(), &Source::Solid(SolidSource::from(color)), &DrawOptions::new());
    }

    fn DrawTriangle(&mut self, pointA: Point2D<f32,f32>, pointB: Point2D<f32,f32>, pointC: Point2D<f32,f32>, color: Color,) {
        let mut pb = PathBuilder::new();

        pb.move_to(pointA.x, pointA.y);
        pb.line_to(pointB.x,pointB.y);
        pb.line_to(pointC.x,pointC.y);

        pb.close();
        self.dt.fill(&pb.finish(), &Source::Solid(SolidSource::from(color)), &DrawOptions::new());
    }

    fn DrawImage(&mut self, point: Point2D<f32,f32>) {
        //self.dt.draw_image_with_size_at(width: f32, height: f32, x: f32, y: f32, image: &Image, options: &DrawOptions)
        // [todo] how to insert image pointer here?
    }

    fn DrawText(&mut self, point: Point2D<f32,f32>) {
        let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();

        /*self.dt.draw_text(&font, 36., &pos_string, Point::new(0., 100.),
                         &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0)),
                         &DrawOptions::new(),
                        );*/
        //[todo] compiler is complaining about the wrong struct being used in $font, needs further investigation 
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
    fn drawing_to_file_test() {
        use raqote::*;
        use euclid::{Point2D};
        use super::*;

        let mut raqote = Raqote {dt: DrawTarget::new(400, 400)};

        let line = RenderInstruction::DrawLine { 
            pointA: Point2D::new(100., 200.), 
            pointB: Point2D::new(100., 350.), 
            color: Color::new(0xff, 0xff, 0xff, 0xff)
        };
        let triangle = RenderInstruction::DrawTriangle { 
            pointA: Point2D::new(100., 100.), 
            pointB: Point2D::new(200., 200.), 
            pointC: Point2D::new(100., 200.), 
            color: Color::new(0xff, 0xff, 0x00, 0xff)
        };
        let rect = RenderInstruction::DrawRect { 
            point: Point2D::new(300., 100.), 
            width: 50, 
            length: 100,
            color: Color::new(0xff, 0xff, 0xf0, 0x00)
        };
        let arc = RenderInstruction::DrawArc { 
            point: Point2D::new(300., 300.), 
            r: 50., 
            sang: 0., 
            eang: 1.,
            color: Color::new(0xff, 0x00, 0x00, 0x00)
        };
        
        let circle = RenderInstruction::DrawCircle {
            point: Point2D::new(100., 200.),
            r: 100.,
            color: Color::new(0xFF, 0x00, 0xAA, 0xAA)
        };

        raqote.draw(line);
        raqote.draw(circle);
        raqote.draw(triangle);
        raqote.draw(rect);
        raqote.draw(arc);

        raqote.dt.write_png("result.png");
    }
}

