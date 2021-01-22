//! Contains the implementation of a `TabWidget` using the [`hyber`]
//! (`crate`).
//!
//! The Tab View, on the [`hyber`](`crate`), is a widget implemented
//! according  to the [`Widget`] trait but with his own properties.
//! This properties need to be assigned by programmers.

use hyber::display::Display;
use hyber::event::Event;
use hyber::renderer::{AbsoluteWidgetCollection, Message, RenderInstructionCollection, Renderer};
use hyber::util::{Color, IDMachine, Vector2D};
use hyber::widget::grid_view::GridViewWidget;
use hyber::widget::label::LabelWidget;
use hyber::widget::root::RootWidget;
use hyber::widget::tab::TabWidget;
use hyber::widget::{Axis, Layout, Widget};

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

/// The predefined display's width
const WIDTH: f64 = 640.;
/// The predefined display's height
const HEIGHT: f64 = 360.;

/// Messages that the `TabWidget` needs to handle
#[derive(Clone)]
pub enum MessageXPTO {
    /// Message to handle a click on a Tab of the `TabWidget`
    TabPress {
        event: Option<Event>,
    },
    /// Message to handle the dragging of a Tab of the `TabWidget`
    /// by the user
    TabMove {
        /// Reference to the memory location of the `GridViewWidget`
        /// which contains the `TabWidget`
        tab_grid_ptr: Weak<RefCell<GridViewWidget>>,
        /// Reference to the memory location of the `TabWidget`
        tab_ptr: Weak<RefCell<TabWidget>>,
        /// Event that triggers this message
        event: Option<Event>,
    },
}

impl Message for MessageXPTO {
    fn update(&self) {
        match self {
            MessageXPTO::TabPress {
                event: _,
            } => {
                // Not used.
            }
            MessageXPTO::TabMove {
                tab_grid_ptr,
                tab_ptr,
                event: _,
            } => {
                // TODO: The following implementation changes the position of the tags when dragged
                // This is not ideal! 
                // The implementation should be done in Hyber! The problem is: it needs access to the tab's parent in order to change its position
                // and you cannot borrow the parent's pointer because it is already being used recursively on the OnEvent function.
                // Gets the memory reference of the `LabelWidget`
                if let Some(tab) = tab_ptr.upgrade() {
                    // Gets the memory reference of the `TabWidget`
                    if let Some(tab_grid) = tab_grid_ptr.upgrade() {
                        let moved_tab_id = tab.borrow_mut().id();
                        let mut index = 0;
                        let mut index_to_be_moved1 = 0;
                        let mut index_to_be_moved2 = 0;
                        let mut moved = false;
                        for value in tab_grid.borrow_mut().get_children().iter_mut() {
                            if let Some(child) = value.upgrade() {
                                let mut child = child.borrow_mut();
                                //Check if it is a diferent tab than the one moved
                                if child.id() != moved_tab_id {
                                    //Check if mouse moved inside other tabs
                                    if child
                                        .is_cursor_inside(tab.borrow_mut().get_moved_cursor_pos())
                                    {
                                        if !moved {
                                            index_to_be_moved1 = index;
                                            moved = true;
                                        }
                                    }
                                } else {
                                    index_to_be_moved2 = index;
                                }
                            }
                            index = index + 1;
                        }
                        if moved {
                            // swap tabs order if moved
                            tab_grid
                                .borrow_mut()
                                .get_children()
                                .swap(index_to_be_moved1, index_to_be_moved2);
                            tab_grid.borrow_mut().set_dirty(true);
                        }
                    }
                }
            }
        }
    }

    fn set_event(&mut self, new_event: Event) {
        match self {
            MessageXPTO::TabPress {
                event,
            } => {
                *event = Some(new_event);
            }
            MessageXPTO::TabMove {
                tab_grid_ptr: _,
                tab_ptr: _,
                event,
            } => {
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

    // Initializes the `GridViewWidget` to hold multiple `TabWidget`
    let tab_grid = Rc::new(RefCell::new(GridViewWidget::new(
        Vector2D::new(500., 200.),
        Axis::Vertical,
        2,
    )));

    // Initializes the `TabWidget` #01
    let tab1 = Rc::new(RefCell::new(TabWidget::new(
        Vector2D::new(320., 200.),
        Color::from_hex(0xff509996),
        Some(Box::new(MessageXPTO::TabPress {
            event: None,
        })),
        None,
    )));

    // Assigns the message `TabMove` to the `TabWidget` #01
    tab1.borrow_mut()
        .set_new_message_move(Some(Box::new(MessageXPTO::TabMove {
            tab_grid_ptr: Rc::downgrade(&tab_grid),
            tab_ptr: Rc::downgrade(&tab1),
            event: None,
        })));

    // Initializes the `LabelWidget` #01 to display the text within the `TabWidget` #01
    let label_1 = Rc::new(RefCell::new(LabelWidget::new(
        String::from("Tab 1"),
        Vector2D::new(320., 200.),
        55,
        Color::from_hex(0x00008026),
        Color::from_hex(0xff004dff),
    )));

    // Initializes the `TabWidget` #02
    let tab2 = Rc::new(RefCell::new(TabWidget::new(
        Vector2D::new(320., 200.),
        Color::from_hex(0xffd15c20),
        Some(Box::new(MessageXPTO::TabPress {
            event: None,
        })),
        None,
    )));

    // Assigns the message `TabMove` to the `TabWidget` #02
    tab2.borrow_mut()
        .set_new_message_move(Some(Box::new(MessageXPTO::TabMove {
            tab_grid_ptr: Rc::downgrade(&tab_grid),
            tab_ptr: Rc::downgrade(&tab2),
            event: None,
        })));

    // Initializes the `LabelWidget` #02 to display the text within the `TabWidget` #02
    let label_2 = Rc::new(RefCell::new(LabelWidget::new(
        String::from("Tab 2"),
        Vector2D::new(10., 200.),
        55,
        Color::from_hex(0x00008026),
        Color::from_hex(0xff004dff),
    )));

    // Initializes the `TabWidget` #03
    let tab3 = Rc::new(RefCell::new(TabWidget::new(
        Vector2D::new(320., 200.),
        Color::from_hex(0xffd15390),
        Some(Box::new(MessageXPTO::TabPress {
            event: None,
        })),
        None,
    )));

    // Assigns the message `TabMove` to the `TabWidget` #03
    tab3.borrow_mut()
        .set_new_message_move(Some(Box::new(MessageXPTO::TabMove {
            tab_grid_ptr: Rc::downgrade(&tab_grid),
            tab_ptr: Rc::downgrade(&tab3),
            event: None,
        })));

    // Initializes the `LabelWidget` #03 to display the text within the `TabWidget` #03
    let label_3 = Rc::new(RefCell::new(LabelWidget::new(
        String::from("Tab 3"),
        Vector2D::new(10., 200.),
        55,
        Color::from_hex(0x00008026),
        Color::from_hex(0xff004dff),
    )));

    // Initializes the `RootWidget` to handle the widgets tree
    let root = Rc::new(RefCell::new(RootWidget::new(
        display.get_size(),
        Color::new(0xff, 0xff, 0xff, 0xff),
        Layout::Box(Axis::Horizontal),
    )));

    // The next instructions build the widgets relative tree
    
    // Adds the `LabelWidget` #01 as child of the `TabWidget` #01
    tab1.borrow_mut()
        .add_as_child(Rc::downgrade(&label_1) as Weak<RefCell<dyn Widget>>);
    // Adds the `LabelWidget` #02 as child of the `TabWidget` #02
    tab2.borrow_mut()
        .add_as_child(Rc::downgrade(&label_2) as Weak<RefCell<dyn Widget>>);
    // Adds the `LabelWidget` #03 as child of the `TabWidget` #03
    tab3.borrow_mut()
        .add_as_child(Rc::downgrade(&label_3) as Weak<RefCell<dyn Widget>>);

    // Adds the `TabWidget` #01 as child of the `GridViewWidget`
    tab_grid
        .borrow_mut()
        .add_as_child(Rc::downgrade(&tab1) as Weak<RefCell<dyn Widget>>);
    // Adds the `TabWidget` #02 as child of the `GridViewWidget`
    tab_grid
        .borrow_mut()
        .add_as_child(Rc::downgrade(&tab2) as Weak<RefCell<dyn Widget>>);
    // Adds the `TabWidget` #03 as child of the `GridViewWidget`
    tab_grid
        .borrow_mut()
        .add_as_child(Rc::downgrade(&tab3) as Weak<RefCell<dyn Widget>>);

    // Adds the `GridViewWidget` as child of the `RootWidget`
    root.borrow_mut()
        .add_as_child(Rc::downgrade(&tab_grid) as Weak<RefCell<dyn Widget>>);

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
