#[derive(Default, Clone, PartialEq, Eq)]
pub enum FontName {
    #[default]
    Sans,
    SansSerif,
    Mono,
    Other(String),
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Font {
    pub weight: u32,
    pub name: FontName,
}

