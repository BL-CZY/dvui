pub enum Value {
    Pixel(i32),
    Percent(f64),
}

impl Value {
    pub fn get_pixel(&self) -> i32 {
        if let Self::Pixel(v) = self {
            *v
        } else {
            0
        }
    }

    pub fn get_pixel_positive(&self) -> u32 {
        if let Self::Pixel(v) = self {
            if *v >= 0 {
                *v as u32
            } else {
                0
            }
        } else {
            0
        }
    }
}
