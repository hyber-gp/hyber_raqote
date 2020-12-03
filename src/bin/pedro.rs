use hyber;
use hyber::display::Display;
use hyber::renderer::RenderInstruction;
use hyber::renderer::Renderer;
use hyber_raqote;

use euclid::Point2D;
use raqote::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut display = hyber_raqote::DisplayMinifb::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        hyber::display::DisplayDescritor::default(),
    );
    let mut raqote = hyber_raqote::Raqote::new(WIDTH as i32, HEIGHT as i32);

    let line = RenderInstruction::DrawLine {
        point_a: Point2D::new(100., 200.),
        point_b: Point2D::new(100., 350.),
        color: Color::new(0xff, 0xff, 0xff, 0xff),
    };
    let triangle = RenderInstruction::DrawTriangle {
        point_a: Point2D::new(100., 100.),
        point_b: Point2D::new(200., 200.),
        point_c: Point2D::new(100., 200.),
        color: Color::new(0xff, 0xff, 0x00, 0xff),
    };
    let rect = RenderInstruction::DrawRect {
        point: Point2D::new(300., 100.),
        width: 50,
        length: 100,
        color: Color::new(0xff, 0xff, 0xf0, 0x00),
    };
    let arc = RenderInstruction::DrawArc {
        point: Point2D::new(300., 300.),
        r: 50.,
        s_ang: 0.,
        e_ang: 1.,
        color: Color::new(0xff, 0x00, 0x00, 0x00),
    };
    let circle = RenderInstruction::DrawCircle {
        point: Point2D::new(100., 200.),
        r: 100.,
        color: Color::new(0xFF, 0x00, 0xAA, 0xAA),
    };

    raqote.draw(line, &mut display);
    raqote.draw(circle, &mut display);
    raqote.draw(triangle, &mut display);
    raqote.draw(rect, &mut display);
    raqote.draw(arc, &mut display);

    raqote.dt.write_png("result.png");
}
