use hyber::display::Display;
use hyber::event::Event;
use hyber::event::Mouse::CursorMoved;
use hyber::renderer::{Message, RenderInstructionCollection, Renderer};
use hyber::util::{Color, IDMachine, Vector2D};
use hyber::widget::grid_view::GridViewWidget;
use hyber::widget::label::LabelWidget;
use hyber::widget::root::RootWidget;
use hyber::widget::tab::TabWidget;
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
    Resize {
        grid_ptr: Weak<RefCell<GridViewWidget>>,
        event: Option<Event>,
    },
    TabPress {
      //  tab_ptr: Weak<RefCell<LabelWidget>>,
        event: Option<Event>,
    },
    TabMoveLeft {
        tab_grid_ptr: Weak<RefCell<GridViewWidget>>,
        tab_ptr: Weak<RefCell<TabWidget>>,
        event: Option<Event>,
    },
    TabMoveRight {
        tab_grid_ptr: Weak<RefCell<GridViewWidget>>,
        tab_ptr: Weak<RefCell<TabWidget>>,
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
            MessageXPTO::Resize { grid_ptr, event } => {
                /*if let Some(grid) = grid_ptr.upgrade() {
                    if let Some(Event::Mouse(CursorMoved { x, y })) = event {
                        grid.borrow_mut().set_original_size(Vector2D::new(*x as f64, *y as f64))
                    }
                }*/
            }
            MessageXPTO::TabPress {
            //    tab_ptr,
                event: _,
            } => {
              println!("Tab was pressed!");
            }
            MessageXPTO::TabMoveLeft {
                tab_grid_ptr,
                tab_ptr,
                event: _,
            } => {
                println!("Tab was moved to the left!");
                if let Some(tab) = tab_ptr.upgrade(){
                    if let Some(tab_grid) = tab_grid_ptr.upgrade(){
                        let movedTabID = tab.borrow_mut().id();
                        let mut index = 0;
                        let mut index_to_be_moved = 999 ;
                        for value in tab_grid.borrow_mut().get_children().iter_mut() {
                            if let Some(child) = value.upgrade() {
                                let mut child= child.borrow_mut();
                                //Check if it is a diferent tab than the one moved
                                if child.id() != movedTabID{
                                    //Check position of other tabs to the left
                                    println!("X: {}",tab.borrow_mut().get_moved_cursor_pos().x);
                                    if  tab.borrow_mut().get_moved_cursor_pos().x < (child.position().x +child.size().x/2. ) {
                                        if index_to_be_moved == 999 {
                                            index_to_be_moved = index;
                                        }
                                    }
                                }//Once we get to the tab moved we should stop the search
                                else{
                                    break;
                                }
                            }
                            index = index + 1 ;
                        }
                        if index_to_be_moved != 999{
                            tab_grid.borrow_mut().get_children().swap(index_to_be_moved ,index );
                            tab_grid.borrow_mut().set_dirty(true);
                        }
                    }
                }
            }
            MessageXPTO::TabMoveRight {
                tab_grid_ptr,
                tab_ptr,
                event: _,
            } => {
                println!("Tab was moved to the right!");
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
            MessageXPTO::TabPress {
           //     tab_ptr: _,
                event,
            } => {
                *event = Some(new_event);
            }
            MessageXPTO::TabMoveLeft {
                tab_grid_ptr: _,
                tab_ptr: _,
                event,
            } => {
                *event = Some(new_event);
            }
            MessageXPTO::TabMoveRight {
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
        Axis::Vertical,
        3,
    )));

    let counter = Rc::new(RefCell::new(0));


    let tab_grid= Rc::new(RefCell::new(GridViewWidget::new(
        Vector2D::new(500., 200.),
        Axis::Vertical,
        2,
    )));


    let tab1 = Rc::new(RefCell::new(TabWidget::new(
        Vector2D::new(320., 200.),
        Color::from_hex(0xff509996),
        Some(Box::new(MessageXPTO::TabPress {
            //tab_grid_ptr: Rc::downgrade(&tab1),
            event: None,
        })),
        None,
        None,
    )));
    
    tab1.borrow_mut().setNewMessageMoveLeft(Some(Box::new(MessageXPTO::TabMoveLeft{
        tab_grid_ptr: Rc::downgrade(&tab_grid),
        tab_ptr: Rc::downgrade(&tab1),
        event: None,
    })));

    tab1.borrow_mut().setNewMessageMoveRight(Some(Box::new(MessageXPTO::TabMoveRight{
        tab_grid_ptr: Rc::downgrade(&tab_grid),
        tab_ptr: Rc::downgrade(&tab1),
        event: None,
    })));

    let label_1 = Rc::new(RefCell::new(LabelWidget::new(
        String::from("Tab 1"),
        Vector2D::new(320., 200.),
        55,
        Color::from_hex(0xff008026),
        Color::from_hex(0xff004dff),
    )));

    let tab2 = Rc::new(RefCell::new(TabWidget::new(
        Vector2D::new(320., 200.),
        Color::from_hex(0xffd15c20),
        Some(Box::new(MessageXPTO::TabPress {
            //tab_grid_ptr: Rc::downgrade(&tab1),
            event: None,
        })),
        None,
        None,
    )));
    
    tab2.borrow_mut().setNewMessageMoveLeft(Some(Box::new(MessageXPTO::TabMoveLeft{
        tab_grid_ptr: Rc::downgrade(&tab_grid),
        tab_ptr: Rc::downgrade(&tab2),
        event: None,
    })));

    tab2.borrow_mut().setNewMessageMoveRight(Some(Box::new(MessageXPTO::TabMoveRight{
        tab_grid_ptr: Rc::downgrade(&tab_grid),
        tab_ptr: Rc::downgrade(&tab2),
        event: None,
    })));


    let tab3 = Rc::new(RefCell::new(TabWidget::new(
        Vector2D::new(320., 200.),
        Color::from_hex(0xffd15390),
        Some(Box::new(MessageXPTO::TabPress {
            //tab_grid_ptr: Rc::downgrade(&tab1),
            event: None,
        })),
        None,
        None,
    )));
    
    tab3.borrow_mut().setNewMessageMoveLeft(Some(Box::new(MessageXPTO::TabMoveLeft{
        tab_grid_ptr: Rc::downgrade(&tab_grid),
        tab_ptr: Rc::downgrade(&tab3),
        event: None,
    })));

    tab3.borrow_mut().setNewMessageMoveRight(Some(Box::new(MessageXPTO::TabMoveRight{
        tab_grid_ptr: Rc::downgrade(&tab_grid),
        tab_ptr: Rc::downgrade(&tab3),
        event: None,
    })));

    let label_2 = Rc::new(RefCell::new(LabelWidget::new(
        String::from("Tab 2"),
        Vector2D::new(10., 200.),
        55,
        Color::from_hex(0xff008026),
        Color::from_hex(0xff004dff),
    )));



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
        }),
        Box::new(MessageXPTO::Resize {
            grid_ptr: Rc::downgrade(&grid),
            event: None,
        }),
    )));


    // definir relaçoes de parentesco

    //tab1.borrow_mut().add_as_child(Rc::downgrade(&label_1) as Weak<RefCell<dyn Widget>>);
   // tab2.borrow_mut().add_as_child(Rc::downgrade(&label_2) as Weak<RefCell<dyn Widget>>);
    tab_grid.borrow_mut()
    .add_as_child(Rc::downgrade(&tab1) as Weak<RefCell<dyn Widget>>);
    tab_grid.borrow_mut()
    .add_as_child(Rc::downgrade(&tab2) as Weak<RefCell<dyn Widget>>);
    tab_grid.borrow_mut()
    .add_as_child(Rc::downgrade(&tab3) as Weak<RefCell<dyn Widget>>);
    root.borrow_mut()
        .add_as_child(Rc::downgrade(&tab_grid) as Weak<RefCell<dyn Widget>>);


    
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
