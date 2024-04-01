use ncurses::{
    curs_set, init_pair, noecho, start_color, COLOR_BLACK, COLOR_WHITE, CURSOR_VISIBILITY,
};

pub const REGULAR_PAIR: i16 = 0;
pub const HIGHLIGHT_PAIR: i16 = 1;

pub fn init_style() {
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);
}
