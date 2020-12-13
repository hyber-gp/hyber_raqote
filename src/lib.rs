use hyber::{
    display::Display, display::DisplayDescritor, event, renderer::RenderInstruction,
    renderer::Renderer, util::Queue,
};

use minifb;
use raqote::{
    Color, DrawOptions, DrawTarget, LineCap, LineJoin, PathBuilder, SolidSource, Source,
    StrokeStyle, Point
};
// use std::os::raw; for window handle

use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;

use euclid::Point2D;

use image::open;

pub enum EventClient {
    LeftClick,
    RightClick,
}

pub enum MessageXPTO {
    Abc,
    Dfg,
}

#[derive(Copy, Clone)]
pub enum DrawImageOptions {
    OriginalSize,
    Resize {
        width: f32,
        height: f32,
    },
    ResizeMultiplyer {
        mult: f32,
    }
}

pub struct DisplayMinifb {
    pub display: minifb::Window,
}

impl Display for DisplayMinifb {
    type Buffer = Vec<u32>;

    fn new(title: &str, width: usize, height: usize, display_descriptor: DisplayDescritor) -> Self {
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
            Err(e) => panic!("{:?}", e),
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

/// A struct to save the drawtarget of raqote and to use as a reference for the primitives trait
pub struct Raqote {
    pub dt: DrawTarget,
}


impl Raqote {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            dt: DrawTarget::new(width, height),
        }
    }
    fn draw_point(&mut self, point: Point2D<f32, f32>, color: Color) {
        // [Doubt] Isn't the point basically a tiny circle?
    }

    fn draw_line(&mut self, point_a: Point2D<f32, f32>, point_b: Point2D<f32, f32>, color: Color) {
        let mut pb = PathBuilder::new();
        pb.move_to(point_a.x, point_a.y);
        pb.line_to(point_b.x, point_b.y);
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
            &DrawOptions::new(),
        );
    }

    fn draw_arc(&mut self, point: Point2D<f32, f32>, r: f32, s_ang: f32, e_ang: f32, color: Color) {
        let mut pb = PathBuilder::new();
        pb.move_to(point.x, point.y);
        pb.arc(point.x, point.y, r, s_ang, e_ang);

        pb.close();
        self.dt.fill(
            &pb.finish(),
            &Source::Solid(SolidSource::from(color)),
            &DrawOptions::new(),
        );
    }

    fn draw_circle(&mut self, point: Point2D<f32, f32>, r: f32, color: Color) {
        let mut pb = PathBuilder::new();

        pb.move_to(point.x, point.y);
        pb.arc(point.x, point.y, r, 0., 7.);

        pb.close();
        self.dt.fill(
            &pb.finish(),
            &Source::Solid(SolidSource::from(color)),
            &DrawOptions::new(),
        );
    }

    fn draw_rectangle(&mut self, point: Point2D<f32, f32>, l: u32, w: u32, color: Color) {
        let mut pb = PathBuilder::new();

        pb.move_to(point.x, point.y);
        pb.rect(point.x, point.y, l as f32, w as f32);

        pb.close();
        self.dt.fill(
            &pb.finish(),
            &Source::Solid(SolidSource::from(color)),
            &DrawOptions::new(),
        );
    }

    fn draw_triangle(
        &mut self,
        point_a: Point2D<f32, f32>,
        point_b: Point2D<f32, f32>,
        point_c: Point2D<f32, f32>,
        color: Color,
    ) {
        let mut pb = PathBuilder::new();

        pb.move_to(point_a.x, point_a.y);
        pb.line_to(point_b.x, point_b.y);
        pb.line_to(point_c.x, point_c.y);

        pb.close();
        self.dt.fill(
            &pb.finish(),
            &Source::Solid(SolidSource::from(color)),
            &DrawOptions::new(),
        );
    }

    fn draw_image(&mut self, point: Point2D<f32, f32>, path: &str, options: DrawImageOptions) {
        let rgba = open(path).unwrap().into_rgba();
        let img: Vec<u32> = rgba.
            pixels()
            .map(|p| {
                ((p[3] as u32) << 24) | ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | (p[2] as u32)
            })
            .collect();
        match options {
            DrawImageOptions::OriginalSize => 
                self.dt.draw_image_at(point.x, point.y, &raqote::Image { width:rgba.width() as i32, height: rgba.height() as i32, data: &img}, &DrawOptions::new()),
            DrawImageOptions::Resize {width, height} => 
                self.dt.draw_image_with_size_at(width, height,point.x, point.y, &raqote::Image { width:rgba.width() as i32, height: rgba.height() as i32, data: &img}, &DrawOptions::new()),
            DrawImageOptions::ResizeMultiplyer {mult} => 
                self.dt.draw_image_with_size_at(rgba.width() as f32*mult,rgba.height() as f32*mult,point.x, point.y, &raqote::Image { width:rgba.width() as i32, height: rgba.height() as i32, data: &img}, &DrawOptions::new()),
        }
        
    }

    fn draw_text(&mut self, point: Point2D<f32, f32>, string: &str) {
        let font = SystemSource::new()
            .select_best_match(&[FamilyName::SansSerif], &Properties::new())
            .unwrap()
            .load()
            .unwrap();
        
        self.dt.draw_text(&font, 36., string, Point::new(point.x, point.y),
         &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0)),
         &DrawOptions::new(),
        );
    }
}

impl Renderer<DisplayMinifb, EventClient, Point2D<f32, f32>, Color, DrawImageOptions> for Raqote {
    type Message = MessageXPTO;
    fn map_events(event_client: EventClient) -> event::Event {
        match event_client {
            EventClient::LeftClick => {
                event::Event::Mouse(event::Mouse::ButtonPressed(event::MouseButton::Left))
            }
            EventClient::RightClick => {
                event::Event::Mouse(event::Mouse::ButtonPressed(event::MouseButton::Right))
            }
        }
    }

    fn detect_display_events(queue: &mut Queue<event::Event>, display: &mut DisplayMinifb) {
        if display.is_open() && !display.display.is_key_down(minifb::Key::Escape) {
            if display.display.get_mouse_down(minifb::MouseButton::Left) {
                let event = Self::map_events(EventClient::LeftClick);
                queue.enqueue(event);
            }
        }
        // TODO: TEMPORARY CODE, SHOULD BE AMMENDED
        let buffer: Vec<u32> = vec![0; 640 * 360];
        display.update_with_buffer(&buffer, 640, 360);
    }

    fn draw(
        &mut self,
        instruction: RenderInstruction<Point2D<f32, f32>, Color, DrawImageOptions>,
        display: &mut DisplayMinifb,
    ) {
        match instruction {
            RenderInstruction::DrawPoint { point, color } => self.draw_point(point, color),
            RenderInstruction::DrawLine {
                point_a,
                point_b,
                color,
            } => self.draw_line(point_a, point_b, color),
            RenderInstruction::DrawArc {
                point,
                r,
                s_ang,
                e_ang,
                color,
            } => self.draw_arc(point, r, s_ang, e_ang, color),
            RenderInstruction::DrawCircle { point, r, color } => self.draw_circle(point, r, color),
            RenderInstruction::DrawRect {
                point,
                length,
                width,
                color,
            } => self.draw_rectangle(point, length, width, color),
            RenderInstruction::DrawTriangle {
                point_a,
                point_b,
                point_c,
                color,
            } => self.draw_triangle(point_a, point_b, point_c, color),
            RenderInstruction::DrawImage { point, path, options } => self.draw_image(point, path, options),
            RenderInstruction::DrawText { point, string } => self.draw_text(point, string),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
