use hyber::display::Display;
use hyber::event::Event;
use hyber::event::Mouse::CursorMoved;
use hyber::renderer::{AbsoluteWidgetCollection, Message, RenderInstructionCollection, Renderer};
use hyber::util::{Color, IDMachine, Vector2D};
use hyber::widget::grid_view::GridViewWidget;
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
        label_ptr: Weak<RefCell<LabelWidget>>,
        num_ptr: Weak<RefCell<i64>>,
        event: Option<Event>,
    },
    Close {
        label_ptr: Weak<RefCell<LabelWidget>>,
        num_ptr: Weak<RefCell<i64>>,
        event: Option<Event>,
    },
    Resize {
        grid_ptr: Weak<RefCell<GridViewWidget>>,
        event: Option<Event>,
    },
}

// t uconsegues :) we are rooting for you
impl Message for MessageXPTO {
    fn update(&self) {
        match self {
            MessageXPTO::Open {
                label_ptr,
                num_ptr,
                event: _,
            } => {
                if let Some(label) = label_ptr.upgrade() {
                    if let Some(num) = num_ptr.upgrade() {
                        *num.borrow_mut() += 1;
                        if *num.borrow_mut() % 2i64 != 0 {
                            println!("OPEN {}!", num.borrow_mut());
                            label
                                .borrow_mut()
                                .set_original_size(Vector2D::new(100., 100.));
                        } else {
                            println!("Close {}!", num.borrow_mut());
                            label.borrow_mut().set_original_size(Vector2D::new(0., 0.));
                        }
                    }
                }
            }
            MessageXPTO::Close {
                label_ptr,
                num_ptr: _,
                event: _,
            } => {
                if let Some(label) = label_ptr.upgrade() {
                    println!("Close !");
                    label.borrow_mut().set_original_size(Vector2D::new(0., 0.));
                }
            }
            MessageXPTO::Resize { grid_ptr, event } => {
                if let Some(grid) = grid_ptr.upgrade() {
                    if let Some(Event::Mouse(CursorMoved { x, y })) = event {
                        grid.borrow_mut()
                            .set_original_size(Vector2D::new(*x as f64, *y as f64))
                    }
                }
            }
        }
    }

    fn set_event(&mut self, new_event: Event) {
        match self {
            MessageXPTO::Open {
                label_ptr: _,
                num_ptr: _,
                event,
            } => {
                *event = Some(new_event);
            }
            MessageXPTO::Close {
                label_ptr: _,
                num_ptr: _,
                event,
            } => {
                *event = Some(new_event);
            }
            MessageXPTO::Resize { grid_ptr: _, event } => {
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

    let counter = Rc::new(RefCell::new(0));

    let label_1 = Rc::new(RefCell::new(LabelWidget::new(
        String::from("Teste1!"),
        Vector2D::new(2000f64, 2000f64),
        33,
        Color::from_hex(0xffff8026),
        Color::from_hex(0xff004dff),
    )));

    let label_background = Rc::new(RefCell::new(LabelWidget::new(
        String::from("Panel"),
        Vector2D::new(2000000., 200000.),
        20,
        Color::from_hex(0xff008026),
        Color::from_hex(0xffffffff),
    )));

    let panel = Rc::new(RefCell::new(PanelWidget::new(
        Vector2D::new(20000f64, 300f64),
        true,
        Color::from_hex(0x36bd2b00),
        Some(Box::new(MessageXPTO::Open {
            label_ptr: Rc::downgrade(&label_background),
            num_ptr: Rc::downgrade(&counter),
            event: None,
        })),
        Some(Box::new(MessageXPTO::Close {
            label_ptr: Rc::downgrade(&label_background),
            num_ptr: Rc::downgrade(&counter),
            event: None,
        })),
    )));

    let grid = Rc::new(RefCell::new(GridViewWidget::new(
        Vector2D::new(WIDTH, HEIGHT),
        Axis::Vertical,
        3,
    )));

    let root = Rc::new(RefCell::new(RootWidget::new(
        display.get_size(),
        Color::new(0xff, 0xff, 0xff, 0xff),
        Layout::Box(Axis::Horizontal),
    )));

    label_background
        .borrow_mut()
        .add_as_child(Rc::downgrade(&label_1) as Weak<RefCell<dyn Widget>>);
    panel
        .borrow_mut()
        .add_as_child(Rc::downgrade(&label_background) as Weak<RefCell<dyn Widget>>);
    grid.borrow_mut()
        .add_as_child(Rc::downgrade(&panel) as Weak<RefCell<dyn Widget>>);
    root.borrow_mut()
        .add_as_child(Rc::downgrade(&grid) as Weak<RefCell<dyn Widget>>);
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
}
