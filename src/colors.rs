pub enum Colors {
    gray1,
    gray2,
    gray3,
    gray4,
    cyan,
}

impl Colors {
    pub fn get_hex(&self) -> String {
        match self {
            Colors::gray1 => "#222222",
            Colors::gray2 => "#444444",
            Colors::gray3 => "#bbbbbb",
            Colors::gray4 => "#eeeeee",
            Colors::cyan  => "#005577",
        }
    }
}