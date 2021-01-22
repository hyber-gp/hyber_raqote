use hyber::display::Display;
use hyber::event::Event;
use hyber::renderer::{AbsoluteWidgetCollection, Message, RenderInstructionCollection, Renderer};
use hyber::util::{Color, IDMachine, Vector2D};
use hyber::widget::button_view::ButtonViewWidget;
use hyber::widget::checkbox::CheckBoxWidget;
use hyber::widget::grid_view::GridViewWidget;
use hyber::widget::label::LabelWidget;
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
        label_ptr: Weak<RefCell<LabelWidget>>,
        num_ptr: Weak<RefCell<i64>>,
        event: Option<Event>,
    },
    Decrement {
        label_ptr: Weak<RefCell<LabelWidget>>,
        num_ptr: Weak<RefCell<i64>>,
        event: Option<Event>,
    },
    CheckBoxTrigger {
        btn_ptr: Weak<RefCell<ButtonViewWidget>>,
        checkbox_ptr: Weak<RefCell<CheckBoxWidget>>,
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
            MessageXPTO::CheckBoxTrigger {
                btn_ptr,
                checkbox_ptr,
                event: _,
            } => {
                if let Some(button) = btn_ptr.upgrade() {
                    if let Some(checkbox) = checkbox_ptr.upgrade() {
                        button
                            .borrow_mut()
                            .set_is_clickable(checkbox.borrow_mut().get_is_checked())
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
            MessageXPTO::CheckBoxTrigger {
                btn_ptr: _,
                checkbox_ptr: _,
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

    let counter = Rc::new(RefCell::new(0));

    let label_1 = Rc::new(RefCell::new(LabelWidget::new(
        String::from("Teste1!"),
        Vector2D::new(200f64, 200f64),
        33,
        Color::from_hex(0xffff8026),
        Color::from_hex(0xff004dff),
    )));

    let button = Rc::new(RefCell::new(ButtonViewWidget::new(
        Vector2D::new(200f64, 200f64),
        false,
        Color::from_hex(0x36bd2b00),
        Some(Box::new(MessageXPTO::Increment {
            label_ptr: Rc::downgrade(&label_1),
            num_ptr: Rc::downgrade(&counter),
            event: None,
        })),
        Some(Box::new(MessageXPTO::Decrement {
            label_ptr: Rc::downgrade(&label_1),
            num_ptr: Rc::downgrade(&counter),
            event: None,
        })),
    )));

    let checkbox = Rc::new(RefCell::new(CheckBoxWidget::new(
        Vector2D::new(20., 20.),
        Color::from_hex(0xFFFFFFFF),
        Color::from_hex(0xFF26B6EC),
        Color::from_hex(0xFF000000),
        None,
        false,
        2.,
        0.25,
    )));

    checkbox
        .borrow_mut()
        .set_message(Some(Box::new(MessageXPTO::CheckBoxTrigger {
            btn_ptr: Rc::downgrade(&button),
            checkbox_ptr: Rc::downgrade(&checkbox),
            event: None,
        })));

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
    button
        .borrow_mut()
        .add_as_child(Rc::downgrade(&label_1) as Weak<RefCell<dyn Widget>>);
    // definir relaçoes de parentesco
    //grid.borrow_mut().add_as_child(Rc::downgrade(&label_1) as Weak<RefCell<dyn Widget>>);
    grid.borrow_mut()
        .add_as_child(Rc::downgrade(&button) as Weak<RefCell<dyn Widget>>);
    grid.borrow_mut()
        .add_as_child(Rc::downgrade(&checkbox) as Weak<RefCell<dyn Widget>>);
    root.borrow_mut()
        .add_as_child(Rc::downgrade(&grid) as Weak<RefCell<dyn Widget>>);
    //root.borrow_mut().
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
