pub mod components;
pub mod theme;

pub use components::button::{Button, ButtonVariant};
pub use components::cell::Cell;
pub use components::icon::Icon;
pub use components::toast::Toast;
pub use components::dialog::Dialog;
pub use components::actionsheet::ActionSheet;
pub use components::input::Input;
pub use components::checkbox::Checkbox;
pub use theme::{Theme, inject_theme, init_theme};

use wasm_bindgen::prelude::*;
use web_sys::Document;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn init(document: &Document) {
    inject_theme(document);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_creation() {
        let btn = Button::new("Test".to_string());
        assert_eq!(btn.text, "Test");
    }

    #[test]
    fn test_cell_creation() {
        let cell = Cell::new("Content".to_string());
        assert_eq!(cell.content, "Content");
    }

    #[test]
    fn test_theme_default() {
        let theme = Theme::new();
        assert_eq!(theme.primary_color, "#07C160");
    }
}