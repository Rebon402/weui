use leptos::*;

/// Core theme values for a WeUI-based application.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Theme {
    pub primary: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub text_placeholder: String,
    pub background: String,
    pub background_secondary: String,
    pub border: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: "#07C160".into(),
            success: "#07C160".into(),
            warning: "#FFC300".into(),
            error: "#FA5151".into(),
            text_primary: "#191919".into(),
            text_secondary: "#888888".into(),
            text_placeholder: "#B2B2B2".into(),
            background: "#FFFFFF".into(),
            background_secondary: "#F7F7F7".into(),
            border: "#E5E5E5".into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorScheme {
    Green,
    Blue,
    Orange,
    Red,
}

impl ColorScheme {
    pub fn primary(&self) -> &'static str {
        match self {
            Self::Green => "#07C160",
            Self::Blue => "#10AEFF",
            Self::Orange => "#FA9D3B",
            Self::Red => "#FA5151",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Breakpoint {
    Mobile,
    Tablet,
    Desktop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Spacing {
    None,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

impl Spacing {
    pub fn value(&self) -> &'static str {
        match self {
            Self::None => "0",
            Self::Xs => "4px",
            Self::Sm => "8px",
            Self::Md => "12px",
            Self::Lg => "16px",
            Self::Xl => "24px",
            Self::Xxl => "32px",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticColor {
    Primary,
    Success,
    Warning,
    Error,
    Info,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Size {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl Size {
    pub fn button_height(&self) -> &'static str {
        match self {
            Self::Xs => "28px",
            Self::Sm => "32px",
            Self::Md => "44px",
            Self::Lg => "52px",
            Self::Xl => "60px",
        }
    }
    pub fn font_size(&self) -> &'static str {
        match self {
            Self::Xs => "12px",
            Self::Sm => "14px",
            Self::Md => "16px",
            Self::Lg => "18px",
            Self::Xl => "20px",
        }
    }
    pub fn icon_size(&self) -> &'static str {
        match self {
            Self::Xs => "12px",
            Self::Sm => "14px",
            Self::Md => "16px",
            Self::Lg => "20px",
            Self::Xl => "24px",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Position {
    Start,
    Center,
    End,
    Between,
    Around,
    Evenly,
}

#[derive(Clone)]
pub struct ThemeContext {
    pub theme: RwSignal<Theme>,
    pub mode: RwSignal<ThemeMode>,
}

impl Default for ThemeContext {
    fn default() -> Self {
        Self {
            theme: RwSignal::new(Theme::default()),
            mode: RwSignal::new(ThemeMode::Light),
        }
    }
}

pub fn provide_theme() {
    provide_context(ThemeContext::default());
}

pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>()
        .expect("Theme context missing. Wrap with <ThemeProvider>")
}