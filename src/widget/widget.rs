use sdl2::pixels::Color;
use smart_default::SmartDefault;

use super::font::Font;

pub trait Widget {
    fn draw(&self) -> Renderable;
}

pub fn default_color() -> Color {
    Color::RGB(0, 0, 0)
}

#[derive(PartialEq, Eq, Clone, SmartDefault)]
pub struct Padding {
    pub padding_left: i32,
    pub padding_right: i32,
    pub padding_top: i32,
    pub padding_bottom: i32,
}

#[derive(PartialEq, Eq, Clone, SmartDefault)]
pub struct Margin {
    pub margin_left: i32,
    pub margin_right: i32,
    pub margin_top: i32,
    pub margin_bottom: i32,
}

#[derive(PartialEq, Eq, Clone, SmartDefault)]
pub struct Text {
    pub font_size: u32,

    #[default(_code = "default_color()")]
    pub font_color: Color,

    pub font: Font,
}

#[derive(PartialEq, Eq, Clone, SmartDefault)]
pub struct Outline {
    pub left_thickness: u32,
    #[default(_code = "default_color()")]
    pub left_color: Color,

    pub right_thickness: u32,
    #[default(_code = "default_color()")]
    pub right_color: Color,

    pub top_thickness: u32,
    #[default(_code = "default_color()")]
    pub top_color: Color,

    pub bottom_thickness: u32,
    #[default(_code = "default_color()")]
    pub bottom_color: Color,
}

#[derive(PartialEq, Eq, Clone, SmartDefault)]
pub struct Renderable {
    pub width: u32,
    pub height: u32,

    pub padding: Padding,
    pub margin: Margin,

    #[default(_code = "default_color()")]
    pub bg_color: Color,

    pub outline: Outline,

    pub text: Option<Text>,

    pub rounded_corner: u32,

    pub children: Vec<Renderable>,
}
