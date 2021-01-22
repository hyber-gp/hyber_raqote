//! Contains the implementation of a `GridViewWidget` using the [`hyber`]
//! (`crate`).
//!
//! The Grid View, on the [`hyber`](`crate`), is a widget implemented according 
//! to the [`Widget`] trait but with his own properties. This properties
//! need to be assigned by programmers.

use hyber::display::Display;
use hyber::event::Event;
use hyber::event::Mouse::CursorMoved;
use hyber::renderer::{AbsoluteWidgetCollection, Message, RenderInstructionCollection, Renderer};
use hyber::util::{Color, IDMachine, Vector2D};
use hyber::widget::grid_view::GridViewWidget;
use hyber::widget::label::LabelWidget;
use hyber::widget::root::RootWidget;
use hyber::widget::{Axis, Layout, Widget};

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

/// The predefined display's width
const WIDTH: f64 = 640.;
/// The predefined display's height
const HEIGHT: f64 = 360.;

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

    // Initializes a vector to hold `LabelWidget`
    let mut label_vector = Vec::new();

    // Loop to initialize eight `LabelWidget`
    for i in 0..8 {
        // Initializes and inserts a `LabelWidget` on the previous vector
        label_vector.push(Rc::new(RefCell::new(LabelWidget::new(
            String::from(format!("label {}", i+2)),
            Vector2D::new(2000., 2000.),
            20,
            Color::from_hex(0xffffed00),
            Color::from_hex(0xff750787),
        ))))
    }

    // Initializes a `LabelWidget`
    let label_1 = Rc::new(RefCell::new(LabelWidget::new(
        String::from("Teste1!"),
        Vector2D::new(2000., 2000.),
        33,
        Color::from_hex(0xff008026),
        Color::from_hex(0xff004dff),
    )));

    // Initializes a `LabelWidget`
    let label_2 = Rc::new(RefCell::new(LabelWidget::new(
        String::from("Teste2!"),
        Vector2D::new(2000., 2000.),
        33,
        Color::from_hex(0xff509996),
        Color::from_hex(0xff004dff),
    )));

    // Initializes the `GridViewWidget` to hold the multiple `LabelWidget`
    let grid = Rc::new(RefCell::new(GridViewWidget::new(
        Vector2D::new(WIDTH, HEIGHT),
        Axis::Vertical,
        3,
    )));

    // Initializes the `RootWidget` to handle the widgets tree
    let root = Rc::new(RefCell::new(RootWidget::new(
        display.get_size(),
        Color::new(0xff, 0xff, 0xff, 0xff),
        Layout::Box(Axis::Horizontal),
    )));

    // The next instructions build the widgets relative tree

    // Adds a `LabelWidget` as child of the `GridViewWidget`
    grid.borrow_mut()
        .add_as_child(Rc::downgrade(&label_1) as Weak<RefCell<dyn Widget>>);
    // Adds a `LabelWidget` as child of the `GridViewWidget`
    grid.borrow_mut()
        .add_as_child(Rc::downgrade(&label_2) as Weak<RefCell<dyn Widget>>);
    // Iterate over the vector of `LabelWidget`
    for child in label_vector.iter() {
        // Adds a `LabelWidget` as child of the `GridViewWidget`
        grid.borrow_mut()
            .add_as_child(Rc::downgrade(&child) as Weak<RefCell<dyn Widget>>);
    }

    // Adds the `GridViewWidget` as child of the `RootWidget`
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
