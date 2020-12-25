use hyber::{
    display::Display, display::DisplayDescritor, event, key_code, renderer::DrawImageOptions,
    renderer::RenderInstruction, renderer::RenderInstructionCollection, renderer::Renderer,
    util::Color, util::Point, util::Queue,
};

use minifb;
use raqote::{
    DrawOptions, DrawTarget, LineCap, LineJoin, PathBuilder, SolidSource, Source, StrokeStyle,
};
// use std::os::raw; for window handle

use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;

use image::open;

pub enum EventClient {
    ///All mouse events
    LeftClickPress,
    LeftClickRelease,
    RightClickPress,
    RightClickRelease,
    MiddleClickPress,
    MiddleClickRelease,
    MouseMove {
        x: f32,
        y: f32,
    },
    MouseEntered,
    MouseLeft,
    Scroll {
        x: f32,
        y: f32,
    },

    //All Keyboard events
    KeyPressed {
        key_code: hyber::key_code::KeyCode,
        modifiers: hyber::event::ModifiersState,
    },

    KeyReleased {
        key_code: hyber::key_code::KeyCode,
        modifiers: hyber::event::ModifiersState,
    },

    //Window
    WindowResize {
        width: u32,
        height: u32,
    },
}

pub enum MessageXPTO {
    Abc,
    Dfg,
}

#[derive(Default)]
pub struct MouseState {
    pub mouse_pos: (f32, f32),
    pub button_left: bool,
    pub button_middle: bool,
    pub button_right: bool,
    pub mouse_on_window: bool,
}

pub struct WindowState {
    pub window_size: (usize, usize),
}

pub struct DisplayMinifb {
    pub display: minifb::Window,
    pub mouse_state: MouseState,
    pub window_state: WindowState,
}

impl Display for DisplayMinifb {
    type Buffer = Vec<u32>;

    fn new(title: &str, width: usize, height: usize, display_descriptor: DisplayDescritor) -> Self {
        match minifb::Window::new(
            title,
            width,
            height,
            minifb::WindowOptions {
                borderless: display_descriptor.border,
                title: display_descriptor.titled,
                resize: display_descriptor.resizable,
                topmost: display_descriptor.topmost,
                scale: minifb::Scale::X1,
                scale_mode: minifb::ScaleMode::UpperLeft,
                transparency: false,
            },
        ) {
            Ok(display) => DisplayMinifb {
                display: display,
                mouse_state: MouseState::default(),
                window_state: WindowState {
                    window_size: (width, height),
                },
            },
            Err(_) => panic!(),
        }
    }

    fn set_title(&mut self, title: &str) {
        self.display.set_title(title);
    }
    fn update(&mut self) {
        self.display.update();
    }

    fn update_with_buffer(&mut self, buffer: &Self::Buffer, width: usize, height: usize) {
        match self.display.update_with_buffer(buffer, width, height) {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

    fn is_open(&self) -> bool {
        self.display.is_open()
    }

    fn set_position(&mut self, x: usize, y: usize) {
        self.display.set_position(x as isize, y as isize);
    }
    fn border(&mut self, border: bool) {
        unimplemented!();
    }

    fn resizable(&mut self, resizable: bool) {
        unimplemented!();
    }

    fn topmost(&mut self, topmost: bool) {
        self.display.topmost(topmost);
    }

    fn minimizable(&mut self, minimizable: bool) {
        unimplemented!();
    }

    fn set_background_color(&mut self, red: usize, green: usize, blue: usize) {
        self.display.set_background_color(red, green, blue);
    }
    fn get_size(&self) -> (usize, usize) {
        self.display.get_size()
    }

    fn is_active(&mut self) -> bool {
        self.display.is_active()
    }
}

/// A struct to save the drawtarget of raqote and to use as a reference for the primitives trait
pub struct Raqote {
    pub dt: DrawTarget,
}

impl Raqote {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            dt: DrawTarget::new(width, height),
        }
    }
    fn draw_point(&mut self, point: &Point, color: &Color) {
        // [Doubt] Isn't the point basically a tiny circle?
    }

    fn draw_line(&mut self, point_a: &Point, point_b: &Point, color: &Color) {
        let mut pb = PathBuilder::new();
        pb.move_to(point_a.x, point_a.y);
        pb.line_to(point_b.x, point_b.y);
        pb.close();
        self.dt.stroke(
            &pb.finish(),
            &Source::Solid(SolidSource::from_unpremultiplied_argb(
                color.a, color.r, color.g, color.b,
            )),
            &StrokeStyle {
                cap: LineCap::Square,
                join: LineJoin::Bevel,
                width: 10.,
                miter_limit: 0.,
                dash_array: vec![1., 1.],
                dash_offset: 0.,
            },
            &DrawOptions::new(),
        );
    }

    fn draw_arc(&mut self, point: &Point, r: f32, s_ang: f32, e_ang: f32, color: &Color) {
        let mut pb = PathBuilder::new();
        pb.move_to(point.x, point.y);
        pb.arc(point.x, point.y, r, s_ang, e_ang);

        pb.close();
        self.dt.fill(
            &pb.finish(),
            &Source::Solid(SolidSource::from_unpremultiplied_argb(
                color.a, color.r, color.g, color.b,
            )),
            &DrawOptions::new(),
        );
    }

    fn draw_circle(&mut self, point: &Point, r: f32, color: &Color) {
        let mut pb = PathBuilder::new();

        pb.move_to(point.x, point.y);
        pb.arc(point.x, point.y, r, 0., 7.);

        pb.close();
        self.dt.fill(
            &pb.finish(),
            &Source::Solid(SolidSource::from_unpremultiplied_argb(
                color.a, color.r, color.g, color.b,
            )),
            &DrawOptions::new(),
        );
    }

    fn draw_rectangle(&mut self, point: &Point, l: u32, w: u32, color: &Color) {
        let mut pb = PathBuilder::new();

        pb.move_to(point.x, point.y);
        pb.rect(point.x, point.y, l as f32, w as f32);

        pb.close();
        self.dt.fill(
            &pb.finish(),
            &Source::Solid(SolidSource::from_unpremultiplied_argb(
                color.a, color.r, color.g, color.b,
            )),
            &DrawOptions::new(),
        );
    }

    fn draw_triangle(&mut self, point_a: &Point, point_b: &Point, point_c: &Point, color: &Color) {
        let mut pb = PathBuilder::new();

        pb.move_to(point_a.x, point_a.y);
        pb.line_to(point_b.x, point_b.y);
        pb.line_to(point_c.x, point_c.y);

        pb.close();
        self.dt.fill(
            &pb.finish(),
            &Source::Solid(SolidSource::from_unpremultiplied_argb(
                color.a, color.r, color.g, color.b,
            )),
            &DrawOptions::new(),
        );
    }

    fn draw_image(&mut self, point: &Point, path: &str, options: &DrawImageOptions) {
        let rgba = open(path).unwrap().into_rgba8();
        let img: Vec<u32> = rgba
            .pixels()
            .map(|p| {
                ((p[3] as u32) << 24) | ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | (p[2] as u32)
            })
            .collect();
        match options {
            DrawImageOptions::OriginalSize => self.dt.draw_image_at(
                point.x,
                point.y,
                &raqote::Image {
                    width: rgba.width() as i32,
                    height: rgba.height() as i32,
                    data: &img,
                },
                &DrawOptions::new(),
            ),
            DrawImageOptions::Resize { width, height } => self.dt.draw_image_with_size_at(
                *width,
                *height,
                point.x,
                point.y,
                &raqote::Image {
                    width: rgba.width() as i32,
                    height: rgba.height() as i32,
                    data: &img,
                },
                &DrawOptions::new(),
            ),
            DrawImageOptions::ResizeMultiplyer { mult } => self.dt.draw_image_with_size_at(
                rgba.width() as f32 * mult,
                rgba.height() as f32 * mult,
                point.x,
                point.y,
                &raqote::Image {
                    width: rgba.width() as i32,
                    height: rgba.height() as i32,
                    data: &img,
                },
                &DrawOptions::new(),
            ),
        }
    }

    fn draw_text(&mut self, point: &Point, string: &str) {
        let font = SystemSource::new()
            .select_best_match(&[FamilyName::SansSerif], &Properties::new())
            .unwrap()
            .load()
            .unwrap();

        self.dt.draw_text(
            &font,
            36.,
            string,
            raqote::Point::new(point.x, point.y),
            &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0)),
            &DrawOptions::new(),
        );
    }

    pub fn draw(&mut self, instruction: &RenderInstruction) {
        match instruction {
            RenderInstruction::DrawPoint { point, color } => self.draw_point(point, color),
            RenderInstruction::DrawLine {
                point_a,
                point_b,
                color,
            } => self.draw_line(point_a, point_b, color),
            RenderInstruction::DrawArc {
                point,
                r,
                s_ang,
                e_ang,
                color,
            } => self.draw_arc(point, *r, *s_ang, *e_ang, color),
            RenderInstruction::DrawCircle { point, r, color } => self.draw_circle(point, *r, color),
            RenderInstruction::DrawRect {
                point,
                length,
                width,
                color,
            } => self.draw_rectangle(point, *length, *width, color),
            RenderInstruction::DrawTriangle {
                point_a,
                point_b,
                point_c,
                color,
            } => self.draw_triangle(point_a, point_b, point_c, color),
            RenderInstruction::DrawImage {
                point,
                path,
                options,
            } => self.draw_image(point, path, options),
            RenderInstruction::DrawText { point, string } => self.draw_text(point, string),
        }
    }
}
impl Renderer<DisplayMinifb, EventClient> for Raqote {
    type Message = MessageXPTO;
    fn map_events(event_client: EventClient) -> event::Event {
        match event_client {
            EventClient::LeftClickPress => {
                event::Event::Mouse(event::Mouse::ButtonPressed(event::MouseButton::Left))
            }
            EventClient::LeftClickRelease => {
                event::Event::Mouse(event::Mouse::ButtonReleased(event::MouseButton::Left))
            }
            EventClient::RightClickPress => {
                event::Event::Mouse(event::Mouse::ButtonPressed(event::MouseButton::Right))
            }
            EventClient::RightClickRelease => {
                event::Event::Mouse(event::Mouse::ButtonReleased(event::MouseButton::Right))
            }
            EventClient::MiddleClickPress => {
                event::Event::Mouse(event::Mouse::ButtonPressed(event::MouseButton::Middle))
            }
            EventClient::MiddleClickRelease => {
                event::Event::Mouse(event::Mouse::ButtonReleased(event::MouseButton::Middle))
            }
            EventClient::MouseMove { x: new_x, y: new_y } => {
                event::Event::Mouse(event::Mouse::CursorMoved { x: new_x, y: new_y })
            }
            EventClient::MouseEntered => event::Event::Mouse(event::Mouse::CursorEntered),
            EventClient::MouseLeft => event::Event::Mouse(event::Mouse::CursorLeft),
            EventClient::Scroll { x: new_x, y: new_y } => {
                event::Event::Mouse(event::Mouse::WheelScrolled {
                    delta: event::ScrollDelta::Pixels { x: new_x, y: new_y },
                })
            }
            EventClient::KeyPressed {
                key_code: code,
                modifiers: mods,
            } => event::Event::Keyboard(event::Keyboard::KeyPressed {
                key_code: code,
                modifiers: mods,
            }),
            EventClient::KeyReleased {
                key_code: code,
                modifiers: mods,
            } => event::Event::Keyboard(event::Keyboard::KeyReleased {
                key_code: code,
                modifiers: mods,
            }),
            EventClient::WindowResize {
                width: new_width,
                height: new_height,
            } => event::Event::Window(event::Window::Resized {
                width: new_width,
                height: new_height,
            }),
        }
    }

    fn detect_display_events(queue: &mut Queue<event::Event>, display: &mut DisplayMinifb) {
        if display.is_open() && !display.display.is_key_down(minifb::Key::Escape) {
            //Mouse
            let left_button_down = display.display.get_mouse_down(minifb::MouseButton::Left);
            let right_button_down = display.display.get_mouse_down(minifb::MouseButton::Right);
            let middle_button_down = display.display.get_mouse_down(minifb::MouseButton::Middle);
            let mouse_position = display.display.get_mouse_pos(minifb::MouseMode::Pass);
            //Window
            let window_size = display.display.get_size();

            if window_size != display.window_state.window_size {
                queue.enqueue(Self::map_events(EventClient::WindowResize {
                    width: window_size.0 as u32,
                    height: window_size.1 as u32,
                }));
                display.window_state.window_size = window_size;
            }

            if mouse_position != Some(display.mouse_state.mouse_pos) {
                let mut x: f32 = 0f32;
                let mut y: f32 = 0f32;
                mouse_position.map(|mouse| {
                    x = mouse.0;
                    y = mouse.1;
                });
                let window_width: f32 = display.display.get_size().0 as f32;
                let window_weight: f32 = display.display.get_size().1 as f32;
                if x < 0f32 || x > window_width || y < 0f32 || y > window_weight {
                    //mouse out of window
                    if display.mouse_state.mouse_on_window {
                        display.mouse_state.mouse_on_window = false;
                        queue.enqueue(Self::map_events(EventClient::MouseLeft));
                    }
                }
                if x > 0f32 && x < window_width && y > 0f32 && y < window_weight {
                    //mouse inside window
                    queue.enqueue(Self::map_events(EventClient::MouseMove { x: x, y: y }));
                    if !display.mouse_state.mouse_on_window {
                        display.mouse_state.mouse_on_window = true;
                        queue.enqueue(Self::map_events(EventClient::MouseEntered));
                    }
                }
                display.mouse_state.mouse_pos = (x, y);
            }

            display.display.get_scroll_wheel().map(|scroll| {
                queue.enqueue(Self::map_events(EventClient::Scroll {
                    x: scroll.0,
                    y: scroll.1,
                }));
            });

            if left_button_down != display.mouse_state.button_left {
                if left_button_down {
                    queue.enqueue(Self::map_events(EventClient::LeftClickPress));
                } else {
                    queue.enqueue(Self::map_events(EventClient::LeftClickRelease));
                }
                display.mouse_state.button_left = left_button_down;
            }
            if right_button_down != display.mouse_state.button_right {
                if right_button_down {
                    queue.enqueue(Self::map_events(EventClient::RightClickPress));
                } else {
                    queue.enqueue(Self::map_events(EventClient::RightClickRelease));
                }
                display.mouse_state.button_right = right_button_down;
            }
            if middle_button_down != display.mouse_state.button_middle {
                if middle_button_down {
                    queue.enqueue(Self::map_events(EventClient::MiddleClickPress));
                } else {
                    queue.enqueue(Self::map_events(EventClient::MiddleClickRelease));
                }
                display.mouse_state.button_middle = middle_button_down;
            }

            //Keyboard
            //Check for key modifiers
            let mut shift = false;
            let mut control = false;
            let mut alt = false;
            let mut logo = false;

            if display.display.is_key_down(minifb::Key::LeftShift)
                || display.display.is_key_down(minifb::Key::RightShift)
            {
                shift = true;
            }

            if display.display.is_key_down(minifb::Key::LeftCtrl)
                || display.display.is_key_down(minifb::Key::RightCtrl)
            {
                control = true;
            }

            if display.display.is_key_down(minifb::Key::LeftAlt)
                || display.display.is_key_down(minifb::Key::RightAlt)
            {
                alt = true;
            }
            //TODO: Windows Key
            if display.display.is_key_down(minifb::Key::LeftCtrl)
                || display.display.is_key_down(minifb::Key::RightCtrl)
            {
                logo = true;
            }

            //-----Get Key Press--
            display
                .display
                .get_keys_pressed(minifb::KeyRepeat::No)
                .map(|keys| {
                    for t in keys {
                        match t {
                            minifb::Key::Key0 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Key0,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Key1 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Key1,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Key2 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Key2,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Key3 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Key3,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Key4 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Key4,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Key5 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Key5,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Key6 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Key6,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Key7 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Key7,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Key8 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Key8,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Key9 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Key9,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::A => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::A,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::B => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::B,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::C => {
                                if control {
                                    //Copy Event
                                    queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                        key_code: key_code::KeyCode::Copy,
                                        modifiers: event::ModifiersState {
                                            shift,
                                            control,
                                            alt,
                                            logo,
                                        },
                                    }))
                                } else {
                                    queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                        key_code: key_code::KeyCode::C,
                                        modifiers: event::ModifiersState {
                                            shift,
                                            control,
                                            alt,
                                            logo,
                                        },
                                    }))
                                }
                            }

                            minifb::Key::D => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::D,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::E => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::E,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::F => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::F,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::G => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::G,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::H => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::H,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::I => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::I,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::J => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::J,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::K => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::K,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::L => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::L,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::M => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::M,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::N => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::N,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::O => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::O,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::P => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::P,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Q => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Q,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::R => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::R,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::S => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::S,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::T => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::T,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::U => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::U,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::V => {
                                if control {
                                    //Paste event
                                    queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                        key_code: key_code::KeyCode::Paste,
                                        modifiers: event::ModifiersState {
                                            shift,
                                            control,
                                            alt,
                                            logo,
                                        },
                                    }))
                                } else {
                                    queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                        key_code: key_code::KeyCode::V,
                                        modifiers: event::ModifiersState {
                                            shift,
                                            control,
                                            alt,
                                            logo,
                                        },
                                    }))
                                }
                            }

                            minifb::Key::W => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::W,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::X => {
                                if control {
                                    //Cut event
                                    queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                        key_code: key_code::KeyCode::Cut,
                                        modifiers: event::ModifiersState {
                                            shift,
                                            control,
                                            alt,
                                            logo,
                                        },
                                    }))
                                } else {
                                    queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                        key_code: key_code::KeyCode::X,
                                        modifiers: event::ModifiersState {
                                            shift,
                                            control,
                                            alt,
                                            logo,
                                        },
                                    }))
                                }
                            }
                            minifb::Key::Y => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Y,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Z => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Z,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::F1 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::F1,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::F2 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::F2,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::F3 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::F3,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::F4 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::F4,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::F5 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::F5,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::F6 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::F6,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::F7 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::F7,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::F8 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::F8,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::F9 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::F9,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::F10 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::F10,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::F11 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::F11,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::F12 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::F12,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::F13 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::F13,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::F14 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::F14,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::F15 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::F15,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Down => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Down,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Left => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Left,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Right => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Right,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Up => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Up,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Apostrophe => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Apostrophe,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            // minifb::Key::Backquote => queue.enqueue(Self::map_events(EventClient::KeyPressed{key_code:key_code::KeyCode::Backquote, modifiers:event::ModifiersState{shift,control,alt,logo}})),
                            minifb::Key::Backslash => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Backslash,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Comma => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Comma,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Equal => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Equals,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::LeftBracket => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::LBracket,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Minus => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Minus,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Period => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Period,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::RightBracket => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::RBracket,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Semicolon => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Semicolon,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Slash => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Slash,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Backspace => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Backspace,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Delete => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Delete,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::End => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::End,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Enter => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Enter,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Escape => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Escape,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Home => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Home,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Insert => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Insert,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            //TODO: Change mapping MENU
                            minifb::Key::Menu => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::AbntC1,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::PageDown => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::PageDown,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::PageUp => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::PageUp,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Pause => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Pause,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Space => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Space,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::Tab => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Tab,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::NumLock => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Numlock,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            //TODO: Missing CapsLock code hyber
                            //minifb::Key::CapsLock => queue.enqueue(Self::map_events(EventClient::KeyPressed{key_code:key_code::KeyCode::CapsLock, modifiers:event::ModifiersState{shift,control,alt,logo}})),
                            minifb::Key::ScrollLock => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Scroll,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::LeftShift => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::LShift,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::RightShift => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::RShift,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::LeftCtrl => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::LControl,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::RightCtrl => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::RControl,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::NumPad0 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Numpad0,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::NumPad1 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Numpad1,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::NumPad2 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Numpad2,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::NumPad3 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Numpad3,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::NumPad4 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Numpad4,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::NumPad5 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Numpad5,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::NumPad6 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Numpad6,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::NumPad7 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Numpad7,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::NumPad8 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Numpad8,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::NumPad9 => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Numpad9,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::NumPadDot => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::Period,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::NumPadSlash => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::NumpadDivide,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::NumPadAsterisk => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::NumpadMultiply,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::NumPadMinus => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::NumpadSubtract,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::NumPadPlus => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::NumpadAdd,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::NumPadEnter => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::NumpadEnter,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::LeftAlt => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::LAlt,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            minifb::Key::RightAlt => {
                                queue.enqueue(Self::map_events(EventClient::KeyPressed {
                                    key_code: key_code::KeyCode::RAlt,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                            _ => (),
                        }
                    }
                });

            //-----Get Key released -
            display.display.get_keys_released().map(|keys| {
                for t in keys {
                    match t {
                        minifb::Key::Key0 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Key0,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Key1 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Key1,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Key2 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Key2,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Key3 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Key3,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Key4 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Key4,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Key5 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Key5,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Key6 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Key6,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Key7 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Key7,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Key8 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Key8,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Key9 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Key9,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::A => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::A,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::B => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::B,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::C => {
                            if control {
                                //Copy Event
                                queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                    key_code: key_code::KeyCode::Copy,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            } else {
                                queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                    key_code: key_code::KeyCode::C,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                        }

                        minifb::Key::D => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::D,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::E => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::E,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::F => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::F,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::G => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::G,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::H => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::H,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::I => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::I,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::J => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::J,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::K => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::K,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::L => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::L,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::M => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::M,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::N => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::N,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::O => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::O,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::P => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::P,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Q => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Q,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::R => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::R,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::S => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::S,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::T => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::T,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::U => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::U,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::V => {
                            if control {
                                //Paste event
                                queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                    key_code: key_code::KeyCode::Paste,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            } else {
                                queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                    key_code: key_code::KeyCode::V,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                        }

                        minifb::Key::W => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::W,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::X => {
                            if control {
                                //Cut event
                                queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                    key_code: key_code::KeyCode::Cut,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            } else {
                                queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                    key_code: key_code::KeyCode::X,
                                    modifiers: event::ModifiersState {
                                        shift,
                                        control,
                                        alt,
                                        logo,
                                    },
                                }))
                            }
                        }
                        minifb::Key::Y => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Y,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Z => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Z,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::F1 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::F1,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::F2 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::F2,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::F3 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::F3,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::F4 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::F4,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::F5 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::F5,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::F6 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::F6,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::F7 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::F7,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::F8 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::F8,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::F9 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::F9,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::F10 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::F10,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::F11 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::F11,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::F12 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::F12,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::F13 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::F13,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::F14 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::F14,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::F15 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::F15,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Down => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Down,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Left => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Left,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Right => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Right,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Up => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Up,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Apostrophe => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Apostrophe,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        // minifb::Key::Backquote => queue.enqueue(Self::map_events(EventClient::KeyReleased{key_code:key_code::KeyCode::Backquote, modifiers:event::ModifiersState{shift,control,alt,logo}})),
                        minifb::Key::Backslash => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Backslash,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Comma => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Comma,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Equal => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Equals,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::LeftBracket => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::LBracket,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Minus => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Minus,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Period => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Period,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::RightBracket => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::RBracket,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Semicolon => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Semicolon,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Slash => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Slash,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Backspace => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Backspace,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Delete => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Delete,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::End => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::End,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Enter => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Enter,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Escape => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Escape,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Home => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Home,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Insert => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Insert,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        //TODO: Change mapping MENU
                        minifb::Key::Menu => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::AbntC1,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::PageDown => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::PageDown,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::PageUp => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::PageUp,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Pause => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Pause,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Space => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Space,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::Tab => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Tab,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::NumLock => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Numlock,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        //TODO: Missing CapsLock code hyber
                        //minifb::Key::CapsLock => queue.enqueue(Self::map_events(EventClient::KeyReleased{key_code:key_code::KeyCode::CapsLock, modifiers:event::ModifiersState{shift,control,alt,logo}})),
                        minifb::Key::ScrollLock => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Scroll,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::LeftShift => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::LShift,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::RightShift => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::RShift,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::LeftCtrl => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::LControl,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::RightCtrl => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::RControl,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::NumPad0 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Numpad0,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::NumPad1 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Numpad1,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::NumPad2 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Numpad2,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::NumPad3 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Numpad3,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::NumPad4 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Numpad4,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::NumPad5 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Numpad5,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::NumPad6 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Numpad6,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::NumPad7 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Numpad7,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::NumPad8 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Numpad8,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::NumPad9 => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Numpad9,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::NumPadDot => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::Period,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::NumPadSlash => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::NumpadDivide,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::NumPadAsterisk => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::NumpadMultiply,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::NumPadMinus => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::NumpadSubtract,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::NumPadPlus => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::NumpadAdd,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::NumPadEnter => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::NumpadEnter,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::LeftAlt => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::LAlt,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        minifb::Key::RightAlt => {
                            queue.enqueue(Self::map_events(EventClient::KeyReleased {
                                key_code: key_code::KeyCode::RAlt,
                                modifiers: event::ModifiersState {
                                    shift,
                                    control,
                                    alt,
                                    logo,
                                },
                            }))
                        }
                        _ => (),
                    }
                }
            });
        }
        // TODO: TEMPORARY CODE, SHOULD BE AMMENDED
        //let buffer2: Vec<u32> = vec![0; display.display.get_size().0 * display.display.get_size().1];
        //display.update_with_buffer(buffer, 640 as usize, 360 as usize);
    }

    fn draw_collection(
        &mut self,
        collection: &RenderInstructionCollection,
        display: &mut DisplayMinifb,
    ) {
        self.dt.clear(SolidSource::from_unpremultiplied_argb(
            0xff, 0x00, 0x00, 0x00,
        ));
        let size = display.get_size();
        if size.0 as i32 != self.dt.width() || size.1 as i32 != self.dt.height() {
            self.dt = DrawTarget::new(size.0 as i32, size.1 as i32);
        }
        for (key, instructions) in collection.pairs.iter() {
            for instruction in instructions {
                self.draw(instruction);
            }
        }

        display
            .display
            .update_with_buffer(self.dt.get_data(), size.0, size.1)
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
