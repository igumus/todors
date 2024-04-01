use ncurses::*;

mod ui;

use ui::action::*;
use ui::layout::*;
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
    let mut status = Status::Todo;
    let mut w = 0;
    let mut h = 0;

    let mut todo_curr: usize = 0;
    let mut done_curr: usize = 0;
    let mut todos = vec![
        "Make todo app".to_string(),
        "Make a cup of tea".to_string(),
        "Buy a bread".to_string(),
    ];
    let mut dones = vec!["Start the stream".to_string()];

    while !ui.should_quit() {
        getmaxyx(stdscr(), &mut h, &mut w);

        ui.begin(LayoutKind::Horz);
        {
            ui.begin_layout(LayoutKind::Vert);
            {
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
                        if todo_curr == index && status == Status::Todo {
                            style::HIGHLIGHT_PAIR
                        } else {
                            style::REGULAR_PAIR
                        },
                        w / 2,
                    );
                }
            }
            ui.end_layout();
            ui.begin_layout(LayoutKind::Vert);
            {
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
                        if done_curr == index && status == Status::Done {
                            style::HIGHLIGHT_PAIR
                        } else {
                            style::REGULAR_PAIR
                        },
                        w / 2,
                    );
                }
            }
            ui.end_layout();

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
