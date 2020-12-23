use hyber;
use hyber::display::Display;
use hyber::renderer::Renderer;
use hyber_raqote;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut display = hyber_raqote::DisplayMinifb::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        hyber::display::DisplayDescritor {resizable: true,..hyber::display::DisplayDescritor::default()},
    );
    let mut renderer = hyber_raqote::Raqote::new(WIDTH as i32, HEIGHT as i32);
    let events = renderer.create_events_queue();
    let messages = renderer.create_message_queue();
    let mut buffer: Vec<u32> = vec![0; 640 * 360];
    display
        .display
        .limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    renderer.event_loop(events, messages, &mut display,&buffer);
    // Limit to max ~60 fps update rate
    /*while window.is_open() && !window.is_key_down(Key::Escape) {
    if window.get_mouse_down(minifb::MouseButton::Left) {
        let event = Rendererxpto::map_events(EventoCliente::left_click);
        queue.enqueue(event);
    }*/
    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
}
