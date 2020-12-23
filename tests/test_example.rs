use hyber;
use hyber::display::Display;
use hyber::renderer::Renderer;
use hyber_raqote;

/// To run tests you need to use the command
/// `cargo test`
#[cfg(test)]
mod tests {
    /// Usually, asserts are used to check wether a test passes.
    /// However, since you cannot assert graphical concepts, the tests need to be a little bit more creative. Good luck!
    /// Check documentation for more details.
    /// You can choose between a test file for each widget or simply one function for each widget in a single file.
    /// You can even choose to have a test file for widgets only, and another bench file for events, and another for multi-widget apps!
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
