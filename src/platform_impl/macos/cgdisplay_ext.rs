use core_graphics::base::CGError;
use core_graphics::display::CGDisplay;
use core_graphics::geometry::CGPoint;

/// Extension trait for `CGDisplay` to add custom functionality.
pub trait CGDisplayExt {
    /// Move the mouse cursor to the specified point in the global display coordinate space.
    ///
    /// This is a custom function that wraps the existing `nextshell_mouse_cursor_position` function.
    fn nextshell_mouse_cursor_position(point: CGPoint) -> Result<(), CGError> {
        CGDisplay::nextshell_mouse_cursor_position(point)
    }
}

// Implement the trait for CGDisplay
impl CGDisplayExt for CGDisplay {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nextshell_mouse_cursor_position() {
        // This is just a compilation test
        let _result = CGDisplay::nextshell_mouse_cursor_position(CGPoint::new(100.0, 100.0));
    }
}
