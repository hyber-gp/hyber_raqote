use hyber::display::Display;
use hyber::event::Event;
use hyber::event::Mouse::CursorMoved;
use hyber::renderer::{
    AbsoluteWidgetCollection, DrawImageOptions, Message, RenderInstructionCollection, Renderer,
};
use hyber::util::{Color, IDMachine, Vector2D};
use hyber::widget::grid_view::GridViewWidget;
use hyber::widget::icon::IconWidget;
use hyber::widget::root::RootWidget;
use hyber::widget::{Axis, Layout, Widget};

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

const WIDTH: f64 = 640.;
const HEIGHT: f64 = 360.;

#[derive(Clone)]
pub enum MessageXPTO {
    Increment {
        icon_ptr: Weak<RefCell<IconWidget>>,
        num_ptr: Weak<RefCell<i64>>,
        event: Option<Event>,
    },
    Decrement {
        icon_ptr: Weak<RefCell<IconWidget>>,
        num_ptr: Weak<RefCell<i64>>,
        event: Option<Event>,
    },
    Resize {
        grid_ptr: Weak<RefCell<GridViewWidget>>,
        event: Option<Event>,
    },
}

impl Message for MessageXPTO {
    fn update(&self) {
        match self {
            MessageXPTO::Increment {
                icon_ptr,
                num_ptr,
                event: _,
            } => {
                if let Some(_icon) = icon_ptr.upgrade() {
                    if let Some(num) = num_ptr.upgrade() {
                        *num.borrow_mut() += 1;
                    }
                }
            }
            MessageXPTO::Decrement {
                icon_ptr,
                num_ptr,
                event: _,
            } => {
                if let Some(_icon) = icon_ptr.upgrade() {
                    if let Some(num) = num_ptr.upgrade() {
                        *num.borrow_mut() -= 1;
                    }
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
            MessageXPTO::Increment {
                icon_ptr: _,
                num_ptr: _,
                event,
            } => {
                *event = Some(new_event);
            }
            MessageXPTO::Decrement {
                icon_ptr: _,
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

    let icon = Rc::new(RefCell::new(IconWidget::new(
        String::from("rust.png"),  // path
        Vector2D::new(200., 200.), // size of the box layout
        DrawImageOptions::Resize {
            width: 200,
            height: 200,
        }, // DrawImageOptions
        Color::from_hex(0xff004dff), // color
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
        Box::new(MessageXPTO::Increment {
            icon_ptr: Rc::downgrade(&icon),
            num_ptr: Rc::downgrade(&counter),
            event: None,
        }),
        Box::new(MessageXPTO::Decrement {
            icon_ptr: Rc::downgrade(&icon),
            num_ptr: Rc::downgrade(&counter),
            event: None,
        }),
    )));

    // definir relaçoes de parentesco
    grid.borrow_mut()
        .add_as_child(Rc::downgrade(&icon) as Weak<RefCell<dyn Widget>>);
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
