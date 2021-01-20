use hyber::display::Display;
use hyber::event::Event;
use hyber::event::Mouse::CursorMoved;
use hyber::renderer::{Message, RenderInstructionCollection, Renderer};
use hyber::util::{Color, IDMachine, Vector2D};
use hyber::widget::grid_view::GridViewWidget;
use hyber::widget::label::LabelWidget;
use hyber::widget::progress_bar::ProgressBarWidget;
use hyber::widget::root::RootWidget;
use hyber::widget::{Axis, Layout, Widget};

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

const WIDTH: f64 = 640.;
const HEIGHT: f64 = 360.;

#[derive(Clone)]
pub enum MessageXPTO {
    Progress {
        label_ptr: Weak<RefCell<LabelWidget>>,
        progress_ptr: Weak<RefCell<ProgressBarWidget>>,
        num_ptr: Weak<RefCell<f64>>,
        inc: f64,
        event: Option<Event>,
    },
}

// t uconsegues :) we are rooting for you
impl Message for MessageXPTO {
    fn update(&self) {
        match self {
            MessageXPTO::Progress {
                label_ptr,
                progress_ptr,
                num_ptr,
                inc,
                event: _,
            } => {
                if let Some(label) = label_ptr.upgrade() {
                    if let Some(progress) = progress_ptr.upgrade() {
                        if let Some(num) = num_ptr.upgrade() {
                            *num.borrow_mut() += inc;
                            
                            if *num.borrow() > 100.0 {
                                *num.borrow_mut() = 100.0;
                            }
                            
                            label
                            .borrow_mut()
                            .set_text(String::from(format!("Progress: {:.1}%", *num.borrow())));
                            progress
                            .borrow_mut()
                            .set_progress(*num.borrow());
                        }
                    }
                }
            }
        }
    }

    fn set_event(&mut self, new_event: Event) {
        match self {
            MessageXPTO::Progress {
                label_ptr: _,
                progress_ptr: _,
                num_ptr: _,
                inc: _,
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

    let mut collection = RenderInstructionCollection::new();

    let grid = Rc::new(RefCell::new(GridViewWidget::new(
        Vector2D::new(WIDTH, HEIGHT),
        Axis::Horizontal,
        3,
    )));

    let label_1 = Rc::new(RefCell::new(LabelWidget::new(
        String::from("Progress: 0%"),
        Vector2D::new(100., 100.),
        33,
        Color::from_hex(0xffffffff),
        Color::from_hex(0xff004dff),
    )));

    let progressbar_1 = Rc::new(RefCell::new(ProgressBarWidget::new(
        Vector2D::new(500., 50.),
        33,
        0f64,
        Color::from_hex(0xff008026),
        Color::from_hex(0xff004dff),
    )));

    let counter = Rc::new(RefCell::new(0.0));

    let root = Rc::new(RefCell::new(RootWidget::new(
        display.get_size(),
        Color::new(0xff, 0xff, 0xff, 0xff),
        Layout::Box(Axis::Horizontal),
    )));

    // definir relaçoes de parentesco
    grid.borrow_mut()
        .add_as_child(Rc::downgrade(&label_1) as Weak<RefCell<dyn Widget>>);
    grid.borrow_mut()
        .add_as_child(Rc::downgrade(&progressbar_1) as Weak<RefCell<dyn Widget>>);
    root.borrow_mut()
        .add_as_child(Rc::downgrade(&grid) as Weak<RefCell<dyn Widget>>);
    let mut renderer = hyber_raqote::Raqote::new(WIDTH as i32, HEIGHT as i32);
    let events = renderer.create_events_queue();
    let mut messages = renderer.create_message_queue();

    messages.enqueue(
        Box::new(MessageXPTO::Progress{
            label_ptr: Rc::downgrade(&label_1),
            progress_ptr: Rc::downgrade(&progressbar_1),
            num_ptr: Rc::downgrade(&counter),
            inc: 5.2,
            event: None,
        })
    );

    messages.enqueue(
        Box::new(MessageXPTO::Progress{
            label_ptr: Rc::downgrade(&label_1),
            progress_ptr: Rc::downgrade(&progressbar_1),
            num_ptr: Rc::downgrade(&counter),
            inc: 15.6,
            event: None,
        })
    );
        
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
