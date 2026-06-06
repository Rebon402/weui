use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlStyleElement};

static THEME_CSS: Lazy<String> = Lazy::new(|| include_str!("../../assets/theme.css").to_string());

pub fn inject_theme(document: &Document) {
    let style = document.create_element("style").unwrap().dyn_into::<HtmlStyleElement>().unwrap();
    style.set_text_content(Some(&THEME_CSS));
    let head = document.head().unwrap();
    head.append_child(&style).unwrap();
}

pub struct Theme {
    primary_color: String,
    background_color: String,
    text_color: String,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            primary_color: "#07C160".to_string(),
            background_color: "#FFFFFF".to_string(),
            text_color: "#333333".to_string(),
        }
    }
}

impl Theme {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn primary_color(mut self, color: &str) -> Self {
        self.primary_color = color.to_string();
        self
    }

    pub fn background_color(mut self, color: &str) -> Self {
        self.background_color = color.to_string();
        self
    }

    pub fn text_color(mut self, color: &str) -> Self {
        self.text_color = color.to_string();
        self
    }

    pub fn apply(&self, document: &Document) {
        let root = document.document_element().unwrap();
        let style = root.style();
        style.set_property("--weui-primary", &self.primary_color).unwrap();
        style.set_property("--weui-background", &self.background_color).unwrap();
        style.set_property("--weui-text", &self.text_color).unwrap();
    }
}

#[wasm_bindgen]
pub fn init_theme() -> Theme {
    Theme::new()
}