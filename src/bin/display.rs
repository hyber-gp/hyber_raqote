use hyber;
use hyber::display::Display;
use hyber::renderer::DrawImageOptions;
use hyber::renderer::RenderInstruction;
use hyber::renderer::RenderInstructionCollection;
use hyber::renderer::Renderer;
use hyber::util::Color;
use hyber::util::IDMachine;
use hyber::util::Point;
use hyber::widget::*;
use hyber_raqote::MessageXPTO;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut display = hyber_raqote::DisplayMinifb::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        hyber::display::DisplayDescritor {
            resizable: true,
            ..hyber::display::DisplayDescritor::default()
        },
    );

    /*let line = RenderInstruction::DrawLine {
        point_a: Point { x: 100.0, y: 200.0 },
        point_b: Point { x: 100.0, y: 350.0 },
        color: Color {
            a: 0xff,
            r: 0xff,
            g: 0xff,
            b: 0xff,
        },
    };
    let triangle = RenderInstruction::DrawTriangle {
        point_a: Point { x: 100.0, y: 100.0 },
        point_b: Point { x: 200.0, y: 200.0 },
        point_c: Point { x: 100.0, y: 200.0 },
        color: Color {
            a: 0xff,
            r: 0xff,
            g: 0x00,
            b: 0xff,
        },
    };
    let rect = RenderInstruction::DrawRect {
        point: Point { x: 300.0, y: 100.0 },
        width: 50,
        height: 100,
        color: Color {
            a: 0xff,
            r: 0xff,
            g: 0xf0,
            b: 0xff,
        },
    };
    let arc = RenderInstruction::DrawArc {
        point: Point { x: 300.0, y: 300.0 },
        r: 50.,
        s_ang: 0.,
        e_ang: 1.,
        color: Color {
            a: 0xff,
            r: 0x00,
            g: 0x00,
            b: 0x00,
        },
    };
    let circle = RenderInstruction::DrawCircle {
        point: Point { x: 100.0, y: 200.0 },
        r: 100.,
        color: Color {
            a: 0xff,
            r: 0x00,
            g: 0xaa,
            b: 0xaa,
        },
    };
    let text = RenderInstruction::DrawText {
        point: Point { x: 250.0, y: 250.0 },
        string: String::from("Test 123"),
    };
    let image = RenderInstruction::DrawImage {
        point: Point { x: 200.0, y: 100.0 },
        path: String::from("result.png"),
        options: DrawImageOptions::Resize {
            height: 50.0,
            width: 50.0,
        },
    };*/

    let mut id_machine = IDMachine::new();

    let mut collection = RenderInstructionCollection::new();

    let mut root = RootWidget::<MessageXPTO>::new(display.get_size(), Axis::Vertical);

    let mut label = LabelWidget::<MessageXPTO>::new(
        String::from("Teste!"),
        (200, 350),
        Color {
            a: 0xff,
            r: 0xff,
            g: 0x00,
            b: 0x00,
        },
        Axis::Vertical,
    );

    let mut label_2 = LabelWidget::<MessageXPTO>::new(
        String::from("Teste2!"),
        (100, 100),
        Color {
            a: 0xff,
            r: 0x00,
            g: 0xff,
            b: 0x00,
        },
        Axis::Vertical,
    );

    root.add_as_child(Box::new(label));
    root.add_as_child(Box::new(label_2));

    root.build(
        0,
        0,
        display.get_size().0,
        display.get_size().1,
        &mut id_machine,
        &mut collection,
    );

    let mut renderer = hyber_raqote::Raqote::new(WIDTH as i32, HEIGHT as i32);
    let events = renderer.create_events_queue();
    let messages = renderer.create_message_queue();

    renderer.event_loop(events, messages, &mut display, &mut collection);
    // Limit to max ~60 fps update rate
    /*while window.is_open() && !window.is_key_down(Key::Escape) {
    if window.get_mouse_down(minifb::MouseButton::Left) {
        let event = Rendererxpto::map_events(EventoCliente::left_click);
        queue.enqueue(event);
    }*/
    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
}
