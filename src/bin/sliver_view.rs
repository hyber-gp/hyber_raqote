use hyber::display::Display;
use hyber::event::Event;
use hyber::event::Mouse::CursorMoved;
use hyber::renderer::{AbsoluteWidgetCollection, Message, RenderInstructionCollection, Renderer};
use hyber::util::{Color, IDMachine, Vector2D};
use hyber::widget::label::LabelWidget;
use hyber::widget::root::RootWidget;
use hyber::widget::sliver_view::SliverViewWidget;
use hyber::widget::{Axis, Layout, Widget};

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

const WIDTH: f64 = 640.;
const HEIGHT: f64 = 360.;

#[derive(Clone)]
pub enum MessageXPTO {
    Increment {
        label_ptr: Weak<RefCell<LabelWidget>>,
        num_ptr: Weak<RefCell<i64>>,
        event: Option<Event>,
    },
    Decrement {
        label_ptr: Weak<RefCell<LabelWidget>>,
        num_ptr: Weak<RefCell<i64>>,
        event: Option<Event>,
    },
    Resize {
        sliver_ptr: Weak<RefCell<SliverViewWidget>>,
        event: Option<Event>,
    },
}

impl Message for MessageXPTO {
    fn update(&self) {
        match self {
            MessageXPTO::Increment {
                label_ptr,
                num_ptr,
                event: _,
            } => {
                if let Some(label) = label_ptr.upgrade() {
                    if let Some(num) = num_ptr.upgrade() {
                        *num.borrow_mut() += 1;
                        label
                            .borrow_mut()
                            .set_text(String::from(format!("{}", *num.borrow())));
                    }
                }
            }
            MessageXPTO::Decrement {
                label_ptr,
                num_ptr,
                event: _,
            } => {
                if let Some(label) = label_ptr.upgrade() {
                    if let Some(num) = num_ptr.upgrade() {
                        *num.borrow_mut() -= 1;
                        label
                            .borrow_mut()
                            .set_text(String::from(format!("{}", *num.borrow())));
                    }
                }
            }
            MessageXPTO::Resize { sliver_ptr, event } => {
                if let Some(sliver) = sliver_ptr.upgrade() {
                    if let Some(Event::Mouse(CursorMoved { x, y })) = event {
                        sliver
                            .borrow_mut()
                            .set_original_size(Vector2D::new(*x as f64, *y as f64))
                    }
                }
            }
        }
    }

    fn set_event(&mut self, new_event: Event) {
        match self {
            MessageXPTO::Increment {
                label_ptr: _,
                num_ptr: _,
                event,
            } => {
                *event = Some(new_event);
            }
            MessageXPTO::Decrement {
                label_ptr: _,
                num_ptr: _,
                event,
            } => {
                *event = Some(new_event);
            }
            MessageXPTO::Resize {
                sliver_ptr: _,
                event,
            } => {
                *event = Some(new_event);
            }
        }
    }
}

fn main() {
    let mut display = hyber_raqote::DisplayMinifb::new(
        "Test - ESC to exit",
        WIDTH as usize,
        HEIGHT as usize,
        hyber::display::DisplayDescritor {
            resizable: true,
            ..hyber::display::DisplayDescritor::default()
        },
    );
    let mut id_machine = IDMachine::new();

    let collection = Rc::new(RefCell::new(RenderInstructionCollection::new()));

    let absolute_collection = Rc::new(RefCell::new(AbsoluteWidgetCollection::new()));

    let sliver = Rc::new(RefCell::new(SliverViewWidget::new(
        Vector2D::new(WIDTH, HEIGHT),
        Axis::Vertical,
    )));

    let mut label_vector = Vec::new();

    for i in 0..40 {
        label_vector.push(Rc::new(RefCell::new(LabelWidget::new(
            String::from(format!("label {}", i)),
            Vector2D::new(2000., 50.),
            20,
            Color::from_hex(0xffffed00),
            Color::from_hex(0xff750787),
        ))))
    }

    let label_1 = Rc::new(RefCell::new(LabelWidget::new(
        String::from("Teste1!"),
        Vector2D::new(2000., 50.),
        20,
        Color::from_hex(0xff008026),
        Color::from_hex(0xff004dff),
    )));

    let label_2 = Rc::new(RefCell::new(LabelWidget::new(
        String::from("Teste2!"),
        Vector2D::new(2000., 50.),
        20,
        Color::from_hex(0xff509996),
        Color::from_hex(0xff004d00),
    )));

    let root = Rc::new(RefCell::new(RootWidget::new(
        display.get_size(),
        Color::new(0xff, 0xff, 0xff, 0xff),
        Layout::Box(Axis::Horizontal),
    )));

    // definir relaçoes de parentesco
    sliver
        .borrow_mut()
        .add_as_child(Rc::downgrade(&label_1) as Weak<RefCell<dyn Widget>>);
    sliver
        .borrow_mut()
        .add_as_child(Rc::downgrade(&label_2) as Weak<RefCell<dyn Widget>>);
    for child in label_vector.iter() {
        sliver
            .borrow_mut()
            .add_as_child(Rc::downgrade(&child) as Weak<RefCell<dyn Widget>>);
    }
    root.borrow_mut()
        .add_as_child(Rc::downgrade(&sliver) as Weak<RefCell<dyn Widget>>);
    let mut renderer = hyber_raqote::Raqote::new(WIDTH as i32, HEIGHT as i32);
    let events = renderer.create_events_queue();
    let messages = renderer.create_message_queue();

    renderer.event_loop(
        events,
        messages,
        Rc::downgrade(&root) as Weak<RefCell<dyn Widget>>,
        &mut display,
        Vector2D::new(WIDTH, HEIGHT),
        &mut id_machine,
        Rc::downgrade(&collection),
        Rc::downgrade(&absolute_collection),
    );
    // Limit to max ~60 fps update rate
    /*while window.is_open() && !window.is_key_down(Key::Escape) {
    if window.get_mouse_down(minifb::MouseButton::Left) {
        let event = Rendererxpto::map_events(EventoCliente::left_click);
        queue.enqueue(event);
    }*/
    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
}
