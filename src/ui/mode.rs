#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Visual,
    Insert,
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::Normal => "NORMAL".to_owned(),
            Mode::Visual => "VISUAL".to_owned(),
            Mode::Insert => "INSERT".to_owned(),
        }
    }
}
