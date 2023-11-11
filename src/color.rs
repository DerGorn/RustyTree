#[derive(Clone)]
pub enum Color {
    RGBA(u8, u8, u8, u8),
    HSVA(u8, u8, u8, u8),
}
impl Color {
    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::RGBA(r, g, b, a)
    }

    pub fn new_hsva(h: u8, s: u8, v: u8, a: u8) -> Self {
        Self::HSVA(h, s, v, a)
    }

    pub fn from_str(color_name: &str) -> Self {
        match color_name {
            "black" => Self::new_rgba(0, 0, 0, 255),
            "white" => Self::new_rgba(255, 255, 255, 255),
            "red" => Self::new_rgba(255, 0, 0, 255),
            "blue" => Self::new_rgba(0, 0, 255, 255),
            "green" => Self::new_rgba(0, 255, 0, 255),
            "purple" => Self::new_rgba(170, 0, 140, 255),
            "whine_red" => Self::new_rgba(88, 24, 31, 255),
            x => panic!("Unknown color name '{}'", x),
        }
    }

    fn hsva_to_rgba(hsva: [u8; 4]) -> [u8; 4] {
        let [h, s, l, a] = hsva;
        let h_f = h as f64 / 255.0;
        let s_f = s as f64 / 100.0;
        let v_f = l as f64 / 100.0;

        let c = v_f * s_f;
        let h_dash = h_f * 6.0;
        let x = c * (1.0 - (h_dash % 2.0 - 1.0).abs());

        let m = v_f - c;
        let c = ((c + m) * 255.0) as u8;
        let x = ((x + m) * 255.0) as u8;
        let m = (m * 255.0) as u8;

        match h_dash {
            f if f < 1.0 => [c, x, m, a],
            f if f < 2.0 => [x, c, m, a],
            f if f < 3.0 => [m, c, x, a],
            f if f < 4.0 => [m, x, c, a],
            f if f < 5.0 => [x, m, c, a],
            f if f < 6.0 => [c, m, x, a],
            _ => panic!("Something went very wrong when converting from hsva to rgba. There is no possibility to end up here, but we managed. The only sollution is to end this. Everything goes dark and you die."),
        }
    }

    pub fn to_rgba(&self) -> Self {
        match self {
            Self::RGBA(..) => self.clone(),
            Self::HSVA(h, s, v, a) => {
                let [r, g, b, a] = Self::hsva_to_rgba([*h, *s, *v, *a]);
                Self::new_rgba(r, g, b, a)
            }
        }
    }

    pub fn to_slice(&self) -> [u8; 4] {
        match self {
            Self::RGBA(r, g, b, a) => [*r, *g, *b, *a],
            Self::HSVA(h, s, v, a) => [*h, *s, *v, *a],
        }
    }
}
