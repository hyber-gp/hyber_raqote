//! Contains the implementation of a `IconWidget` using the [`hyber`]
//! (`crate`).
//!
//! The Icon, on the [`hyber`](`crate`), is a widget implemented according
//! to the [`Widget`] trait but with his own properties. This properties
//! need to be assigned by programmers.

use hyber::display::Display;
use hyber::renderer::{
    AbsoluteWidgetCollection, DrawImageOptions, RenderInstructionCollection, Renderer,
};
use hyber::util::{Color, IDMachine, Vector2D};
use hyber::widget::grid_view::GridViewWidget;
use hyber::widget::icon::IconWidget;
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

    // Initializes an `IconWidget`
    let icon = Rc::new(RefCell::new(IconWidget::new(
        String::from("rust.png"),
        Vector2D::new(200., 200.),
        DrawImageOptions::Resize {
            width: 200,
            height: 200,
        }, // DrawImageOptions
        // TODO: There is an issue regarding the background colour.
        // When the bg colour contains red, at least when using
        // the library raqote to render the icon widget,
        // the system panics with an overflown exception on the
        // the draw target function of that same library
        Color::from_hex(0xff004dff), // color
    )));

    // Initializes the `GridViewWidget` to hold the `IconWidget`
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

    // Adds a `IconWidget` as child of the `GridViewWidget`
    grid.borrow_mut()
        .add_as_child(Rc::downgrade(&icon) as Weak<RefCell<dyn Widget>>);
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
