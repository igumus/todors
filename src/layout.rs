use crate::vec2::*;
use std::cmp;
pub enum LayoutKind {
    Horz,
    Vert,
}

pub struct Layout {
    pub kind: LayoutKind,
    pub pos: Vec2,
    pub size: Vec2,
}

impl Layout {
    pub fn new(kind: LayoutKind, pos: Vec2) -> Self {
        Layout {
            kind,
            pos,
            size: Vec2::zero(),
        }
    }

    pub fn available_pos(&self) -> Vec2 {
        match self.kind {
            LayoutKind::Horz => self.pos + self.size * Vec2::new(1, 0),
            LayoutKind::Vert => self.pos + self.size * Vec2::new(0, 1),
        }
    }

    pub fn add_widget(&mut self, size: Vec2) {
        match self.kind {
            LayoutKind::Horz => {
                self.size.x += size.x;
                self.size.y = cmp::max(self.size.y, size.y);
            }
            LayoutKind::Vert => {
                self.size.x = cmp::max(self.size.x, size.x);
                self.size.y += size.y;
            }
        }
    }
}
