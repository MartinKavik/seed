// ------ ColorValue ------

#[allow(clippy::module_name_repetitions)]
pub type ColorValue = u8;

// ------ Color ------

#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    pub red: ColorValue,
    pub green: ColorValue,
    pub blue: ColorValue,
}

impl Color {
    pub fn to_css(self) -> String {
        format!("rgb({},{},{})", self.red, self.green, self.blue)
    }

    pub fn map_red(mut self, f: impl FnOnce(ColorValue) -> ColorValue) -> Self {
        self.red = f(self.red);
        self
    }

    pub fn map_green(mut self, f: impl FnOnce(ColorValue) -> ColorValue) -> Self {
        self.green = f(self.green);
        self
    }

    pub fn map_blue(mut self, f: impl FnOnce(ColorValue) -> ColorValue) -> Self {
        self.blue = f(self.blue);
        self
    }
}
