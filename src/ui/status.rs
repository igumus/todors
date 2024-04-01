#[derive(PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Status {
    Todo,
    Done,
}

impl Status {
    pub fn toggle(&self) -> Self {
        match self {
            Status::Todo => Status::Done,
            Status::Done => Status::Todo,
        }
    }
}
