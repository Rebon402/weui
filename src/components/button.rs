pub struct Button {
    text: String,
    variant: ButtonVariant,
}

pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
}

impl Button {
    pub fn new(text: String) -> Self {
        Button {
            text,
            variant: ButtonVariant::Primary,
        }
    }

    pub fn primary(mut self) -> Self {
        self.variant = ButtonVariant::Primary;
        self
    }

    pub fn secondary(mut self) -> Self {
        self.variant = ButtonVariant::Secondary;
        self
    }

    pub fn outline(mut self) -> Self {
        self.variant = ButtonVariant::Outline;
        self
    }
}