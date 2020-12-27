use hyber::display::Display;
use hyber::event::Event;
use hyber::event::Mouse::CursorMoved;
use hyber::renderer::DrawImageOptions;
use hyber::renderer::Message;
use hyber::renderer::RenderInstruction;
use hyber::renderer::RenderInstructionCollection;
use hyber::renderer::Renderer;
use hyber::util::Color;
use hyber::util::IDMachine;
use hyber::util::Vector2D;
use hyber::widget::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

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
        label_ptr: Weak<RefCell<LabelWidget>>,
        event: Option<Event>,
    },
}

// t uconsegues :) we are rooting for you
impl Message for MessageXPTO {
    fn update(&self) {
        match self {
            MessageXPTO::Increment {
                label_ptr,
                num_ptr,
                event,
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
                event,
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
            MessageXPTO::Resize { label_ptr, event } => {
                if let Some(label) = label_ptr.upgrade() {
                    if let Some(Event::Mouse(CursorMoved { x, y })) = event {
                        label.borrow_mut().set_size(Vector2D::new(*x, *y))
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
                label_ptr: _,
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
        WIDTH,
        HEIGHT,
        hyber::display::DisplayDescritor {
            resizable: true,
            ..hyber::display::DisplayDescritor::default()
        },
    );
    let mut id_machine = IDMachine::new();

    let mut collection = RenderInstructionCollection::new();

    let label = Rc::new(RefCell::new(LabelWidget::new(
        String::from("Teste!"),
        Vector2D::new(200, 150),
        80,
        Color::from_hex(0xffffed00),
        Color::from_hex(0xff750787),
        Axis::Vertical,
    )));

    let label_2 = Rc::new(RefCell::new(LabelWidget::new(
        String::from("Teste2!"),
        Vector2D::new(100, 100),
        33,
        Color::from_hex(0xff008026),
        Color::from_hex(0xff004dff),
        Axis::Vertical,
    )));

    let counter = Rc::new(RefCell::new(0));

    let root = Rc::new(RefCell::new(RootWidget::new(
        display.get_size(),
        Color::new(0xff, 0xff, 0xff, 0xff),
        Axis::Vertical,
        Box::new(MessageXPTO::Increment {
            label_ptr: Rc::downgrade(&label),
            num_ptr: Rc::downgrade(&counter),
            event: None,
        }),
        Box::new(MessageXPTO::Decrement {
            label_ptr: Rc::downgrade(&label),
            num_ptr: Rc::downgrade(&counter),
            event: None,
        }),
        Box::new(MessageXPTO::Resize {
            label_ptr: Rc::downgrade(&label),
            event: None,
        }),
    )));

    // definir relaçoes de parentesco
    root.borrow_mut()
        .add_as_child(Rc::downgrade(&label) as Weak<RefCell<dyn Widget>>);
    root.borrow_mut()
        .add_as_child(Rc::downgrade(&label_2) as Weak<RefCell<dyn Widget>>);

    // chamar build que é recursivo
    root.borrow_mut().build(
        Vector2D::new(0, 0),
        display.get_size(),
        &mut id_machine,
        &mut collection,
    );

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
        &mut collection,
    );
    // Limit to max ~60 fps update rate
    /*while window.is_open() && !window.is_key_down(Key::Escape) {
    if window.get_mouse_down(minifb::MouseButton::Left) {
        let event = Rendererxpto::map_events(EventoCliente::left_click);
        queue.enqueue(event);
    }*/
    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
}
