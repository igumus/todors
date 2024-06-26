use ncurses::*;
use std::collections::HashSet;

mod ui;

use ui::action::*;
use ui::key_map::*;
use ui::layout::*;
use ui::mode::*;
use ui::status::*;
use ui::style;
use ui::vec2::*;

struct Ui {
    quit: bool,
    layouts: Vec<Layout>,
}

impl Ui {
    fn new() -> Self {
        initscr();
        keypad(stdscr(), true);
        style::init_style();
        Ui {
            quit: false,
            layouts: Vec::new(),
        }
    }

    fn should_quit(&self) -> bool {
        self.quit
    }

    fn do_quit(&mut self) {
        self.quit = !self.quit;
    }

    fn label_with_fix_width(&mut self, text: &str, pair: i16, width: i32) {
        let layout = self
            .layouts
            .last_mut()
            .expect("Trying to render labele outsize of any layout");
        let new_pos = layout.available_pos();
        mv(new_pos.y, new_pos.x);
        attron(COLOR_PAIR(pair));
        addstr(text).unwrap();
        attroff(COLOR_PAIR(pair));
        layout.add_widget(Vec2::new(width, 1));
    }

    fn begin(&mut self, kind: LayoutKind) {
        assert!(self.layouts.is_empty());
        erase();
        self.layouts.push(Layout::new(kind, Vec2::zero()));
    }

    fn end(&mut self) {
        self.layouts
            .pop()
            .expect("Unbalanced Ui::begin_layout and Ui::end_layout calls");
    }

    fn begin_layout(&mut self, kind: LayoutKind) {
        let layout = self
            .layouts
            .last()
            .expect("Can't create a layout outsize of Ui::begin and Ui::end");
        self.layouts.push(Layout::new(kind, layout.available_pos()));
    }

    fn end_layout(&mut self) {
        let layout = self
            .layouts
            .pop()
            .expect("Unbalanced Ui::begin_layout and Ui::end_layout calls");
        self.layouts
            .last_mut()
            .expect("Unbalanced Ui::begin_layout and Ui::end_layout calls")
            .add_widget(layout.size);
    }
}

fn main() {
    let mut ui = Ui::new();
    let mut w = 0;
    let mut h = 0;
    let mut notification: String = String::from("");

    let mut status = Status::Todo;
    let mut mode = Mode::Normal;
    let mut cursor = 0;
    let mut todo_curr: usize = 0;
    let mut done_curr: usize = 0;
    let mut v_todos: HashSet<String> = HashSet::new();
    let mut v_dones: HashSet<String> = HashSet::new();
    let mut todos = vec![
        "Make todo app".to_string(),
        "Make a cup of tea".to_string(),
        "Buy a bread".to_string(),
    ];
    let mut dones = vec!["Start the stream".to_string()];

    while !ui.should_quit() {
        getmaxyx(stdscr(), &mut h, &mut w);

        ui.begin(LayoutKind::Vert);
        {
            ui.begin_layout(LayoutKind::Horz);
            {
                ui.begin_layout(LayoutKind::Vert);
                ui.label_with_fix_width(
                    "TODO",
                    if status == Status::Todo {
                        style::HIGHLIGHT_PAIR
                    } else {
                        style::REGULAR_PAIR
                    },
                    w / 2,
                );

                for (index, todo) in todos.iter().enumerate() {
                    ui.label_with_fix_width(
                        &format!(" - [ ] {}", todo),
                        if mode == Mode::Normal && todo_curr == index && status == Status::Todo {
                            style::HIGHLIGHT_PAIR
                        } else if mode == Mode::Visual
                            && v_todos.contains(todo)
                            && status == Status::Todo
                        {
                            style::HIGHLIGHT_PAIR
                        } else {
                            style::REGULAR_PAIR
                        },
                        w / 2,
                    );
                }
                ui.end_layout();

                ui.begin_layout(LayoutKind::Vert);
                ui.label_with_fix_width(
                    "DONE",
                    if status == Status::Done {
                        style::HIGHLIGHT_PAIR
                    } else {
                        style::REGULAR_PAIR
                    },
                    w / 2,
                );
                for (index, done) in dones.iter().enumerate() {
                    ui.label_with_fix_width(
                        &format!(" - [x] {}", done),
                        if mode == Mode::Normal && done_curr == index && status == Status::Done {
                            style::HIGHLIGHT_PAIR
                        } else if mode == Mode::Visual
                            && v_dones.contains(done)
                            && status == Status::Done
                        {
                            style::HIGHLIGHT_PAIR
                        } else {
                            style::REGULAR_PAIR
                        },
                        w / 2,
                    );
                }
                ui.end_layout();
            }
            ui.end_layout();
            ui.begin_layout(LayoutKind::Horz);
            ui.label_with_fix_width(&mode.to_string(), style::REGULAR_PAIR, w / 10);
            ui.label_with_fix_width(
                &format!("{} ", notification),
                style::REGULAR_PAIR,
                w * 9 / 10,
            );
            ui.end_layout();

            refresh();
            let key = getch() as i32;
            notification.clear();
            match mode {
                Mode::Normal => match (status, key) {
                    (_, KEYMAP_QUIT) => ui.do_quit(),
                    (_, KEYMAP_TAB) => status = status.toggle(),
                    (_, KEYMAP_V) => mode = Mode::Visual,
                    (Status::Todo, KEYMAP_SHIFT_A) => {
                        mode = Mode::Insert;
                        let item = todos.get_mut(todo_curr).unwrap();
                        cursor = item.len() - 1;
                    }
                    (Status::Done, KEYMAP_SHIFT_A) => {
                        mode = Mode::Insert;
                        let item = dones.get_mut(done_curr).unwrap();
                        cursor = item.len() - 1;
                    }
                    (Status::Todo, KEYMAP_SHIFT_I) => {
                        mode = Mode::Insert;
                        cursor = 0;
                    }
                    (Status::Done, KEYMAP_SHIFT_I) => {
                        mode = Mode::Insert;
                        cursor = 0;
                    }
                    (Status::Todo, KEYMAP_O) => {
                        mode = Mode::Insert;
                        todo_curr += 1;
                        todos.insert(todo_curr, String::new());
                        cursor = 0;
                    }
                    (Status::Todo, KEYMAP_SHIFT_O) => {
                        mode = Mode::Insert;
                        todos.insert(todo_curr, String::new());
                        cursor = 0;
                    }
                    (Status::Todo, KEYMAP_J) => go(Direction::Down, todos.len(), &mut todo_curr),
                    (Status::Done, KEYMAP_J) => go(Direction::Down, dones.len(), &mut done_curr),
                    (Status::Todo, KEYMAP_SHIFT_J) => {
                        drag(Direction::Down, &mut todos, &mut todo_curr)
                    }
                    (Status::Done, KEYMAP_SHIFT_J) => {
                        drag(Direction::Down, &mut dones, &mut done_curr)
                    }
                    (Status::Todo, KEYMAP_G) => go(Direction::First, todos.len(), &mut todo_curr),
                    (Status::Done, KEYMAP_G) => go(Direction::First, dones.len(), &mut done_curr),
                    (Status::Todo, KEYMAP_SHIFT_G) => {
                        go(Direction::Last, todos.len(), &mut todo_curr)
                    }
                    (Status::Done, KEYMAP_SHIFT_G) => {
                        go(Direction::Last, dones.len(), &mut done_curr)
                    }
                    (Status::Todo, KEYMAP_K) => go(Direction::Up, todos.len(), &mut todo_curr),
                    (Status::Done, KEYMAP_K) => go(Direction::Up, dones.len(), &mut done_curr),
                    (Status::Todo, KEYMAP_SHIFT_K) => {
                        drag(Direction::Up, &mut todos, &mut todo_curr)
                    }
                    (Status::Done, KEYMAP_SHIFT_K) => {
                        drag(Direction::Up, &mut dones, &mut done_curr)
                    }
                    (Status::Todo, KEYMAP_NEWLINE) => {
                        transfer(&mut dones, &mut todos, &mut todo_curr)
                    }
                    (Status::Done, KEYMAP_NEWLINE) => {
                        transfer(&mut todos, &mut dones, &mut done_curr)
                    }
                    (Status::Done, KEYMAP_D) => {
                        delete(&mut dones, &mut done_curr);
                        notification.push_str("Item moved to TODO");
                    }
                    (_, _) => {}
                },
                Mode::Visual => match (status, key) {
                    (_, KEYMAP_QUIT) => ui.do_quit(),
                    (_, KEYMAP_TAB) => {
                        status = status.toggle();
                        mode = Mode::Normal;
                    }
                    (_, KEYMAP_V) => {
                        mode = Mode::Normal;
                        v_todos.clear();
                        v_dones.clear();
                    }
                    (Status::Todo, KEYMAP_J) => {
                        if let Some(item) = todos.get(todo_curr) {
                            if !v_todos.remove(item) {
                                v_todos.insert(item.clone());
                            }
                            go(Direction::Down, todos.len(), &mut todo_curr);
                        }
                    }
                    (Status::Done, KEYMAP_J) => {
                        if let Some(item) = dones.get(done_curr) {
                            if !v_dones.remove(item) {
                                v_dones.insert(item.clone());
                            }
                            go(Direction::Down, dones.len(), &mut done_curr);
                        }
                    }
                    (Status::Todo, KEYMAP_K) => {
                        if let Some(item) = todos.get(todo_curr) {
                            if !v_todos.remove(item) {
                                v_todos.insert(item.clone());
                            }
                            go(Direction::Up, todos.len(), &mut todo_curr);
                        }
                    }
                    (Status::Done, KEYMAP_K) => {
                        if let Some(item) = dones.get(done_curr) {
                            if !v_dones.remove(item) {
                                v_dones.insert(item.clone());
                            }
                            go(Direction::Up, dones.len(), &mut done_curr);
                        }
                    }
                    (Status::Todo, KEYMAP_NEWLINE) => {
                        if !v_todos.is_empty() {
                            todos = todos.into_iter().filter(|t| !v_todos.contains(t)).collect();
                            for t in v_todos.iter() {
                                dones.push(t.to_string());
                            }
                            v_todos.clear();
                        }
                    }
                    (Status::Done, KEYMAP_NEWLINE) => {
                        if !v_dones.is_empty() {
                            dones = dones.into_iter().filter(|t| !v_dones.contains(t)).collect();
                            for t in v_dones.iter() {
                                todos.push(t.to_string());
                            }
                            v_dones.clear();
                        }
                    }
                    (Status::Done, KEYMAP_D) => {
                        if !v_dones.is_empty() {
                            dones = dones.into_iter().filter(|t| !v_dones.contains(t)).collect();
                            v_dones.clear();
                        }
                    }
                    (_, _) => {}
                },
                Mode::Insert => match (status, key) {
                    (_, KEYMAP_ESC) => mode = Mode::Normal,
                    (_, KEYMAP_NEWLINE) => {
                        mode = Mode::Normal;
                        cursor = 0;
                    }
                    (_, KEYMAP_LEFT) => {
                        if cursor > 0 {
                            cursor -= 1;
                        }
                    }
                    (Status::Done, KEYMAP_DELETE) => {
                        todos.push(format!("key:{}", key));
                        let item = dones.get_mut(done_curr).unwrap();
                        if cursor < item.len() {
                            item.remove(cursor);
                        }
                    }
                    (Status::Done, KEYMAP_BACKSPACE) => {
                        if cursor > 0 {
                            let item = dones.get_mut(done_curr).unwrap();
                            item.remove(cursor);
                            cursor -= 1;
                        }
                    }
                    (Status::Done, key) => {
                        let item = dones.get_mut(done_curr).unwrap();
                        match key {
                            32..=126 => {
                                item.insert(cursor, char::from_u32(key as u32).unwrap());
                                cursor += 1;
                            }
                            _ => {}
                        }
                    }
                    (Status::Todo, KEYMAP_DELETE) => {
                        let item = todos.get_mut(todo_curr).unwrap();
                        if cursor < item.len() {
                            item.remove(cursor);
                        }
                    }
                    (Status::Todo, KEYMAP_BACKSPACE) => {
                        if cursor > 0 {
                            let item = todos.get_mut(todo_curr).unwrap();
                            item.remove(cursor);
                            cursor -= 1;
                        }
                    }
                    (Status::Todo, KEYMAP_RIGHT) => {
                        let item = todos.get_mut(todo_curr).unwrap();
                        if cursor < item.len() {
                            cursor += 1;
                        }
                    }
                    (Status::Todo, key) => {
                        let item = todos.get_mut(todo_curr).unwrap();
                        match key {
                            32..=126 => {
                                item.insert(cursor, char::from_u32(key as u32).unwrap());
                                cursor += 1;
                            }
                            _ => {}
                        }
                    }
                },
            }
        }
        ui.end();
    }
    endwin();
}
