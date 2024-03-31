use ncurses::*;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

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

#[derive(PartialEq)]
#[repr(u8)]
enum Direction {
    Up,
    Down,
    First,
    Last,
}

#[derive(Default)]
struct Action {}

impl Action {
    fn go(&self, dir: Direction, size: usize, index: &mut usize) {
        match dir {
            Direction::Down => {
                if size > 0 {
                    *index = (*index + 1) % size;
                }
            }
            Direction::Last => {
                if *index + 1 < size {
                    *index = size - 1;
                }
            }
            Direction::Up => {
                if size > 0 {
                    if *index > 0 {
                        *index = (*index - 1) % size;
                    } else {
                        *index = size - 1;
                    }
                }
            }
            Direction::First => {
                if *index > 0 {
                    *index = 0;
                }
            }
        }
    }

    fn delete(&self, src: &mut Vec<String>, curr: &mut usize) {
        if *curr < src.len() {
            src.remove(*curr);
            if *curr >= src.len() {
                self.go(Direction::Up, src.len(), curr);
            }
        }
    }

    fn drag(&self, dir: Direction, src: &mut [String], curr: &mut usize) {
        match dir {
            Direction::Down => {
                if *curr + 1 < src.len() {
                    src.swap(*curr, *curr + 1);
                    *curr += 1;
                }
            }
            Direction::Up => {
                if *curr > 0 {
                    src.swap(*curr, *curr - 1);
                    *curr -= 1;
                }
            }
            _ => {}
        }
    }

    fn transfer(&self, dst: &mut Vec<String>, src: &mut Vec<String>, curr: &mut usize) {
        if !src.is_empty() && *curr < src.len() {
            dst.push(src.remove(*curr));
            if *curr >= src.len() {
                self.go(Direction::Up, src.len(), curr);
            }
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
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        start_color();
        init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
        init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);
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
    let action = Action::default();
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
                (Status::Todo, 'j') => action.go(Direction::Down, todos.len(), &mut todo_curr),
                (Status::Done, 'j') => action.go(Direction::Down, dones.len(), &mut done_curr),
                (Status::Todo, 'J') => action.drag(Direction::Down, &mut todos, &mut todo_curr),
                (Status::Done, 'J') => action.drag(Direction::Down, &mut dones, &mut done_curr),
                (Status::Todo, 'g') => action.go(Direction::First, todos.len(), &mut todo_curr),
                (Status::Done, 'g') => action.go(Direction::First, dones.len(), &mut done_curr),
                (Status::Todo, 'G') => action.go(Direction::Last, todos.len(), &mut todo_curr),
                (Status::Done, 'G') => action.go(Direction::Last, dones.len(), &mut done_curr),
                (Status::Todo, 'k') => action.go(Direction::Up, todos.len(), &mut todo_curr),
                (Status::Done, 'k') => action.go(Direction::Up, dones.len(), &mut done_curr),
                (Status::Todo, 'K') => action.drag(Direction::Up, &mut todos, &mut todo_curr),
                (Status::Done, 'K') => action.drag(Direction::Up, &mut dones, &mut done_curr),
                (Status::Todo, '\n') => action.transfer(&mut dones, &mut todos, &mut todo_curr),
                (Status::Done, '\n') => action.transfer(&mut todos, &mut dones, &mut done_curr),
                (Status::Done, 'd') => action.delete(&mut dones, &mut done_curr),
                (_, _) => {}
            }
        }
        ui.end();
    }
    endwin();
}
