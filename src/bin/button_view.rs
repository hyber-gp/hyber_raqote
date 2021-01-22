//! Contains the implementation of a [`ButtonViewWidget`] using the [`hyber`]
//! (`crate`) as GUI, the [`minifb`](`crate`) as display and the [`raqote`](`crate`)
//! as renderer.
//!
//! Button, on the [`hyber`](`crate`), is a widget implemented according 
//! to the [`Widget`] trait but with his own properties. This properties
//! need to be assigned by programmers.

use hyber::display::Display;
use hyber::event::Event;
use hyber::event::Mouse::CursorMoved;
use hyber::renderer::{Message, RenderInstructionCollection, Renderer, AbsoluteWidgetCollection};
use hyber::util::{Color, IDMachine, Vector2D};
use hyber::widget::grid_view::GridViewWidget;
use hyber::widget::label::LabelWidget;
use hyber::widget::root::RootWidget;
use hyber::widget::button_view::ButtonViewWidget;
use hyber::widget::{Axis, Layout, Widget};

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

/// The predefined display's width
const WIDTH: f64 = 640.;
/// The predefined display's height
const HEIGHT: f64 = 360.;

/// Messages that the [`ButtonViewWidget`] needs to handle
#[derive(Clone)]
pub enum MessageXPTO {
    /// Message to increment the value of the counter
    /// displayed by a label within the [`ButtonViewWidget`]
    Increment {
        /// Reference to the memory location of the [`LabelWidget`]
        /// responsible to displays some text within the [`ButtonViewWidget`]
        label_ptr: Weak<RefCell<LabelWidget>>,
        /// Reference to the memory location of the counter that
        /// needs to be updated
        num_ptr: Weak<RefCell<i64>>,
        /// Event that triggers this message
        event: Option<Event>,
    },
    /// Message to decrement the value of the counter
    /// displayed by a label within the [`ButtonViewWidget`]
    Decrement {
        /// Reference to the memory location of the [`LabelWidget`]
        /// responsible to displays some text within the [`ButtonViewWidget`]
        label_ptr: Weak<RefCell<LabelWidget>>,
        /// Reference to the memory location of the counter that
        /// needs to be updated
        num_ptr: Weak<RefCell<i64>>,
        /// Event that triggers this message
        event: Option<Event>,
    },
    /// Message to resize the display window
    Resize {
        /// Reference to the memory location of the [`GridViewWidget`]
        /// that has the [`ButtonViewWidget`] as child and can be resized
        grid_ptr: Weak<RefCell<GridViewWidget>>,
        /// Event that triggers this message
        event: Option<Event>,
    },
}

impl Message for MessageXPTO {
    fn update(&self) {
        match self {
            // Handles an `Increment` `Message`
            MessageXPTO::Increment {
                label_ptr,
                num_ptr,
                event: _,
            } => {
                // To get the memory reference of the `LabelWidget`
                if let Some(label) = label_ptr.upgrade() {
                    // To get the memory reference of the counter
                    if let Some(num) = num_ptr.upgrade() {
                        // Updates the counter by plus one
                        *num.borrow_mut() += 1;
                        // Updates the `LabelWidget` with the new counter value
                        label
                            .borrow_mut()
                            .set_text(String::from(format!("{}", *num.borrow())));
                    }
                }
            }
            // Handles a `Decrement` `Message`
            MessageXPTO::Decrement {
                label_ptr,
                num_ptr,
                event: _,
            } => {
                // To get the memory reference of the `LabelWidget`
                if let Some(label) = label_ptr.upgrade() {
                    // To get the memory reference of the counter
                    if let Some(num) = num_ptr.upgrade() {
                        // Updates the counter by minus one
                        *num.borrow_mut() -= 1;
                        // Updates the `LabelWidget` with the new counter value
                        label
                            .borrow_mut()
                            .set_text(String::from(format!("{}", *num.borrow())));
                    }
                }
            }
            // Handles a `Resize` `Message`
            MessageXPTO::Resize { grid_ptr, event } => {
                // To get the memory reference of the `GridViewWidget`
                if let Some(grid) = grid_ptr.upgrade() {
                    // To get the specific `CursorMoved` event
                    if let Some(Event::Mouse(CursorMoved { x, y })) = event {
                        // Update the `GridViewWidget` size
                        grid.borrow_mut().set_original_size(Vector2D::new(*x as f64, *y as f64))
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
            MessageXPTO::Resize { grid_ptr: _, event } => {
                *event = Some(new_event);
            }
        }
    }
}

fn main() {
    // Sets up the display using the [`minifb`](`crate`)
    let mut display = hyber_raqote::DisplayMinifb::new(
        "Test - ESC to exit",
        WIDTH as usize,
        HEIGHT as usize,
        hyber::display::DisplayDescritor {
            resizable: true,
            ..hyber::display::DisplayDescritor::default()
        },
    );

    // Sets up the identifier to this machine
    let mut id_machine = IDMachine::new();

    // Sets up the collection to hold all the render instructions required
    let collection = Rc::new(RefCell::new(RenderInstructionCollection::new()));

    // Sets up the absolute widgets collection
    let absolute_collection = Rc::new(RefCell::new(AbsoluteWidgetCollection::new()));

    // Initializes the counter to be displayed by the [`LabelWidget`]
    let counter = Rc::new(RefCell::new(0));

    // Initializes the [`LabelWidget`] to display the text within the [`ButtonViewWidget`]
    let label_1 = Rc::new(RefCell::new(LabelWidget::new(
        String::from("Teste1!"),
        Vector2D::new(200f64, 200f64),
        33,
        Color::from_hex(0xffff8026),
        Color::from_hex(0xff004dff),
    )));

    // Initializes the [`ButtonViewWidget`]
    let button = Rc::new(RefCell::new(ButtonViewWidget::new(
        Vector2D::new(200f64,200f64),
        true,
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
        }))
        
    )));

    // Initializes the [`GridViewWidget`] to hold the [`ButtonViewWidget`]
    let grid = Rc::new(RefCell::new(GridViewWidget::new(
        Vector2D::new(WIDTH, HEIGHT),
        Axis::Vertical,
        3,
    )));

    // Initializes the [`RootWidget`] to handle the widgets tree
    let root = Rc::new(RefCell::new(RootWidget::new(
        display.get_size(),
        Color::new(0xff, 0xff, 0xff, 0xff),
        Layout::Box(Axis::Horizontal),
        Box::new(MessageXPTO::Increment {
            label_ptr: Rc::downgrade(&label_1),
            num_ptr: Rc::downgrade(&counter),
            event: None,
        }),
        Box::new(MessageXPTO::Decrement {
            label_ptr: Rc::downgrade(&label_1),
            num_ptr: Rc::downgrade(&counter),
            event: None,
        })
    )));

    // The next instructions build the widgets relative tree

    // Adds the [`LabelWidget`] as child of the [`ButtonViewWidget`]
    button.borrow_mut()
          .add_as_child(Rc::downgrade(&label_1) as Weak<RefCell<dyn Widget>>);
    // Adds the [`ButtonViewWidget`] as child of the [`GridViewWidget`]
    grid.borrow_mut()
        .add_as_child(Rc::downgrade(&button) as Weak<RefCell<dyn Widget>>);
    // Adds the [`GridViewWidget`] as child of the [`RootWidget`]
    root.borrow_mut()
        .add_as_child(Rc::downgrade(&grid) as Weak<RefCell<dyn Widget>>);

    // Initializes the renderer built with [`raqote`](`crate`)
    let mut renderer = hyber_raqote::Raqote::new(WIDTH as i32, HEIGHT as i32);
    let events = renderer.create_events_queue();
    let messages = renderer.create_message_queue();

    // Initializes the main renderer loop
    // This loop makes it possible to handle all the events
    // and user interactions, as well as manages the widgets
    // that are being displayed
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
