use ncurses::*;

mod action;
mod style;

use action::*;
use style::*;

#[derive(PartialEq, Clone, Copy)]
#[repr(u8)]
enum Status {
    Todo,
    Done,
}

impl Status {
    fn toggle(&self) -> Self {
        match self {
            Status::Todo => Status::Done,
            Status::Done => Status::Todo,
        }
    }
}

struct Ui {
    quit: bool,
    x: i32,
    y: i32,
    active_list: Option<u8>,
}

impl Ui {
    fn new() -> Self {
        initscr();
        init_style();
        Ui {
            quit: false,
            x: 0,
            y: 0,
            active_list: None,
        }
    }

    fn should_quit(&self) -> bool {
        self.quit
    }

    fn do_quit(&mut self) {
        self.quit = !self.quit;
    }

    fn label(&mut self, text: &str, pair: i16) {
        mv(self.y, self.x);
        attron(COLOR_PAIR(pair));
        addstr(text).unwrap();
        attroff(COLOR_PAIR(pair));
        self.y += 1;
    }

    fn list_element(&mut self, text: &str, curr: &usize, index: &usize) {
        let pair = if *curr == *index {
            HIGHLIGHT_PAIR
        } else {
            REGULAR_PAIR
        };
        self.label(text, pair);
    }

    fn begin(&mut self) {
        erase();
        self.x = 0;
        self.y = 0;
    }

    fn begin_list(&mut self, id: u8) {
        assert!(
            self.active_list.is_none(),
            "List#{} is already active. Nested lists are not allowed",
            self.active_list.unwrap()
        );
        self.active_list = Some(id);
    }

    fn end_list(&mut self) {
        self.active_list = None;
    }

    fn end(&self) {}
}

fn main() {
    let mut ui = Ui::new();
    let mut status = Status::Todo;

    let mut todo_curr: usize = 0;
    let mut done_curr: usize = 0;
    let mut todos = vec![
        "Make todo app".to_string(),
        "Make a cup of tea".to_string(),
        "Buy a bread".to_string(),
    ];
    let mut dones = vec!["Start the stream".to_string()];

    while !ui.should_quit() {
        ui.begin();
        {
            ui.begin_list(status as u8);
            {
                match status {
                    Status::Todo => {
                        ui.label("[TODO] DONE", REGULAR_PAIR);
                        for (index, todo) in todos.iter().enumerate() {
                            ui.list_element(&format!(" - [ ] {}", todo), &todo_curr, &index);
                        }
                    }
                    Status::Done => {
                        ui.label("TODO [DONE]", REGULAR_PAIR);
                        for (index, done) in dones.iter().enumerate() {
                            ui.list_element(&format!(" - [x] {}", done), &done_curr, &index);
                        }
                    }
                }
            }
            ui.end_list();

            refresh();
            let key = getch() as u8 as char;
            match (status, key) {
                (_, 'q') => ui.do_quit(),
                (_, '\t') => status = status.toggle(),
                (Status::Todo, 'j') => go(Direction::Down, todos.len(), &mut todo_curr),
                (Status::Done, 'j') => go(Direction::Down, dones.len(), &mut done_curr),
                (Status::Todo, 'J') => drag(Direction::Down, &mut todos, &mut todo_curr),
                (Status::Done, 'J') => drag(Direction::Down, &mut dones, &mut done_curr),
                (Status::Todo, 'g') => go(Direction::First, todos.len(), &mut todo_curr),
                (Status::Done, 'g') => go(Direction::First, dones.len(), &mut done_curr),
                (Status::Todo, 'G') => go(Direction::Last, todos.len(), &mut todo_curr),
                (Status::Done, 'G') => go(Direction::Last, dones.len(), &mut done_curr),
                (Status::Todo, 'k') => go(Direction::Up, todos.len(), &mut todo_curr),
                (Status::Done, 'k') => go(Direction::Up, dones.len(), &mut done_curr),
                (Status::Todo, 'K') => drag(Direction::Up, &mut todos, &mut todo_curr),
                (Status::Done, 'K') => drag(Direction::Up, &mut dones, &mut done_curr),
                (Status::Todo, '\n') => transfer(&mut dones, &mut todos, &mut todo_curr),
                (Status::Done, '\n') => transfer(&mut todos, &mut dones, &mut done_curr),
                (Status::Done, 'd') => delete(&mut dones, &mut done_curr),
                (_, _) => {}
            }
        }
        ui.end();
    }
    endwin();
}
