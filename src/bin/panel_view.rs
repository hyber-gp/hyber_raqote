use hyber::display::Display;
use hyber::event::Event;
use hyber::event::Mouse::CursorMoved;
use hyber::renderer::{AbsoluteWidgetCollection, Message, RenderInstructionCollection, Renderer};
use hyber::util::{Color, IDMachine, Vector2D};
use hyber::widget::label::LabelWidget;
use hyber::widget::panel::PanelWidget;
use hyber::widget::root::RootWidget;
use hyber::widget::{Axis, Layout, Widget};

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

const WIDTH: f64 = 640.;
const HEIGHT: f64 = 360.;

#[derive(Clone)]
pub enum MessageXPTO {
    Open {
        panel_ptr: Weak<RefCell<PanelWidget>>,
        event: Option<Event>,
    },
    Close {
        panel_ptr: Weak<RefCell<PanelWidget>>,
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
            MessageXPTO::Open { panel_ptr, event } => {
                if let Some(panel) = panel_ptr.upgrade() {
                    if let Some(Event::Keyboard(_)) = event {
                        println!("Open\n");
                        panel
                            .borrow_mut()
                            .set_original_size(Vector2D::new(500., 200.))
                    }
                }
            }
            MessageXPTO::Close { panel_ptr, event } => {
                if let Some(panel) = panel_ptr.upgrade() {
                    if let Some(Event::Keyboard(_)) = event {
                        println!("Close\n");
                        panel
                            .borrow_mut()
                            .set_original_size(Vector2D::new(0., -100.))
                    }
                }
            }
            MessageXPTO::Resize { label_ptr, event } => {
                if let Some(panel) = label_ptr.upgrade() {
                    if let Some(Event::Mouse(CursorMoved { x, y })) = event {
                        panel
                            .borrow_mut()
                            .set_original_size(Vector2D::new(*x as f64, *y as f64))
                    }
                }
            }
        }
    }

    fn set_event(&mut self, new_event: Event) {
        match self {
            MessageXPTO::Open {
                panel_ptr: _,
                event,
            } => {
                *event = Some(new_event);
            }
            MessageXPTO::Close {
                panel_ptr: _,
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

    let panel = Rc::new(RefCell::new(PanelWidget::new(
        String::from("Panel"),
        Vector2D::new(0.0, -100.0),
        40,
        Color::from_hex(0xff000000),
        Color::from_hex(0xffff0000),
    )));

    let label_background = Rc::new(RefCell::new(LabelWidget::new(
        String::from("Label"),
        Vector2D::new(2000000., 200000.),
        20,
        Color::from_hex(0xff008026),
        Color::from_hex(0xffffffff),
    )));

    let root = Rc::new(RefCell::new(RootWidget::new(
        display.get_size(),
        Color::new(0xfa, 0xaa, 0xaa, 0xaa),
        Layout::Box(Axis::Horizontal),
    )));

    let mut renderer = hyber_raqote::Raqote::new(WIDTH as i32, HEIGHT as i32);
    let events = renderer.create_events_queue();
    let messages = renderer.create_message_queue();

    panel
        .borrow_mut()
        .add_as_child(Rc::downgrade(&label_background) as Weak<RefCell<dyn Widget>>);
    root.borrow_mut()
        .add_as_child(Rc::downgrade(&panel) as Weak<RefCell<dyn Widget>>);

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
}
