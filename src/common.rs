use strum_macros::{Display, EnumString};

#[derive(PartialEq, Clone)]
pub enum ButtonType {
    Close,
    Minimize,
    Maximize,
}

impl ButtonType {
    pub fn as_class(&self) -> &'static str {
        match self {
            ButtonType::Close => "bg-red-500 hover:bg-red-600 focus:ring-red-300",
            ButtonType::Minimize => "bg-yellow-500 hover:bg-yellow-600 focus:ring-yellow-300",
            ButtonType::Maximize => "bg-green-500 hover:bg-green-600 focus:ring-green-300",
        }
    }

    pub fn default_aria_label(&self) -> &'static str {
        match self {
            ButtonType::Close => "Close window",
            ButtonType::Minimize => "Minimize window",
            ButtonType::Maximize => "Maximize window",
        }
    }

    pub fn default_title(&self) -> &'static str {
        match self {
            ButtonType::Close => "Close window (Cmd+W)",
            ButtonType::Minimize => "Minimize window (Cmd+M)",
            ButtonType::Maximize => "Maximize window (Cmd+Ctrl+F)",
        }
    }
}

#[derive(PartialEq, Clone, Default, Debug, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
    Full,
}

impl Size {
    pub fn to_style(&self) -> &'static str {
        match self {
            Size::Small => "max-width: 28rem; width: auto;",
            Size::Medium => "max-width: 42rem; width: auto;",
            Size::Large => "max-width: 56rem; width: auto;",
            Size::Full => "width: 100%;",
        }
    }
}

#[derive(PartialEq, Clone, Default, Debug, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Variant {
    #[default]
    Default,
    Tabs,
    Ios,
}
