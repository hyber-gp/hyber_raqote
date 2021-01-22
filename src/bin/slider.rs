//! Contains the implementation of a `SliderWidget` using the [`hyber`]
//! (`crate`).
//!
//! The Slider, on the [`hyber`](`crate`), is a widget implemented according 
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
use hyber::widget::slider::SliderWidget;
use hyber::widget::{Axis, Layout, Widget};

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

/// The predefined display's width
const WIDTH: f64 = 640.;
/// The predefined display's height
const HEIGHT: f64 = 360.;

/// Messages that the `SliderWidget` needs to handle
#[derive(Clone)]
pub enum MessageXPTO {
    Slide{
        /// Reference to the memory location of the `LabelWidget`
        /// responsible to displays some text according to the current
        /// `SliderWidget` button value
        label_ptr: Weak<RefCell<LabelWidget>>,
        /// Reference to the memory location of the `SliderWidget`
        slider_ptr: Weak<RefCell<SliderWidget>>,
        /// Event that triggers this message
        event: Option<Event>,
    },
}

impl Message for MessageXPTO {
    fn update(&self) {
        match self {
            // Handles an `Slide` `Message`
            MessageXPTO::Slide{ label_ptr, slider_ptr, event:_,} =>{
                // Gets the memory reference of the `LabelWidget`
                if let Some(label) = label_ptr.upgrade(){
                    // Gets the memory reference of the `SliderWidget`
                    if let Some(slider) = slider_ptr.upgrade(){
                        // Updates the text on the `LabelWidget`
                        // According to the current value on the `SliderWidget` button
                        label.borrow_mut().set_text(slider.borrow_mut().get_slider_value().to_string());
                    }
                }
            }
        }
    }

    fn set_event(&mut self, new_event: Event) {
        match self {
            MessageXPTO::Slide {
                label_ptr: _,
                slider_ptr: _,
                event,
            } => *event = Some(new_event),
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

    // Initializes the `LabelWidget` to display the text according to the
    // current value of the `SliderWidget` button
    let label = Rc::new(RefCell::new(LabelWidget::new(
        String::from("Teste1!"),
        Vector2D::new(200f64, 200f64),
        100,
        Color::from_hex(0xff008026),
        Color::from_hex(0xff004dff),
    )));

    // Initializes the `SliderWidget`
    let slider = Rc::new(RefCell::new(SliderWidget::new(
        Vector2D::new(100., 4.),
        Color::from_hex(0xFF26B6EC),
        Color::from_hex(0xFF000000),
        Vector2D::new(8., 8.),
        (0, 100),
        10,
        100,
        None,
    )));

    // Assigns the message `Slide` to the `SliderWidget`
    slider.borrow_mut().set_message(
        Some(Box::new(MessageXPTO::Slide{
            label_ptr: Rc::downgrade(&label),
            slider_ptr: Rc::downgrade(&slider),
            event: None,
        })));

    // Initializes the `GridViewWidget` to hold the `SliderWidget`
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
    
    // Adds the `LabelWidget` as child of the `GridViewWidget`
    grid.borrow_mut()
        .add_as_child(Rc::downgrade(&label) as Weak<RefCell<dyn Widget>>);
    // Adds the `SliderWidget` as child of the `GridViewWidget`
    grid.borrow_mut()
        .add_as_child(Rc::downgrade(&slider) as Weak<RefCell<dyn Widget>>);
    
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
