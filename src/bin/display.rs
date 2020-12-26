use hyber;
use hyber::display::Display;
use hyber::renderer::DrawImageOptions;
use hyber::renderer::RenderInstruction;
use hyber::renderer::RenderInstructionCollection;
use hyber::renderer::Renderer;
use hyber::util::Color;
use hyber::util::IDMachine;
use hyber::util::Vector2D;
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
    
    let mut id_machine = IDMachine::new();

    let mut collection = RenderInstructionCollection::new();

    let mut root = RootWidget::<MessageXPTO>::new(
        display.get_size(),
        Color::new(0xff, 0xff, 0xff, 0xff),
        Axis::Vertical,
    );

    let mut label = LabelWidget::<MessageXPTO>::new(
        String::from("Teste!"),
        Vector2D::new(200, 150),
        80,
        Color::from_hex(0xffffed00),
        Color::from_hex(0xff750787),
        Axis::Vertical,
    );

    let label_2 = LabelWidget::<MessageXPTO>::new(
        String::from("Teste2!"),
        Vector2D::new(100, 100),
        33,
        Color::from_hex(0xff008026),
        Color::from_hex(0xff004dff),
        Axis::Vertical,
    );

    let label_1_1 = LabelWidget::<MessageXPTO>::new(
        String::from("Teste1!"),
        Vector2D::new(2000, 40),
        50,
        Color::from_hex(0xffe40303),
        Color::new(0xff, 0xff, 0x00, 0xff),
        Axis::Vertical,
    );
    let label_1_2 = LabelWidget::<MessageXPTO>::new(
        String::from("Teste1!"),
        Vector2D::new(20, 150),
        6,
        Color::from_hex(0xffff8c00),
        Color::new(0xff, 0xff, 0xff, 0xff),
        Axis::Vertical,
    );

    label.add_as_child(Box::new(label_1_1));
    label.add_as_child(Box::new(label_1_2));
    root.add_as_child(Box::new(label));
    root.add_as_child(Box::new(label_2));

    root.build(
        Vector2D::new(0, 0),
        display.get_size(),
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
