# todors
Simple VIM-like terminal Todo App in [Rust](https://www.rust-lang.org/).

## Pre-Requisite

Note that you must to have the ncurses library installed and linkable. On Linux, this should be trivial. On OS X, consider installing ncurses using Homebrew. (Note that you have to force Homebrew to link the library to /usr/local/lib: brew link --force ncurses and set that path to LIBRARY_PATH environmental variable.)

## Quick Start
```console
$ cargo run
```

## Modes

|Mode|Description|
|---|----|
| Normal | Normal Mode |
| Visual | Visual Mode | 
| Insert | Insert Mode | 

## Controls

|Mode|Key|Description|
|----|----|----|
| Visual, Normal | <kbd>q</kbd> | Quits application |
| Visual, Normal | <kbd>j</kbd> | Goes one item down in active list | 
| Visual, Normal | <kbd>k</kbd> | Goes one item up in active list | 
| Normal | <kbd>g</kbd> | Goes first item in active list | 
| Visual, Normal | <kbd>d</kbd> | Deletes item in DONE list | 
| Visual, Normal | <kbd>ENTER</kbd> | Performs transfer from active to other list | 
| Normal | <kbd>J</kbd> | Drag item down in active list | 
| Normal | <kbd>G</kbd> | Goes last item in active list | 
| Normal | <kbd>K</kbd> | Drag item up in active list | 
| Normal | <kbd>o</kbd> | Adds new item on after current line in TODO panel |
| Normal | <kbd>O</kbd> | Adds new item on before current line in TODO panel |
| Normal | <kbd>I</kbd> | Moves cursor to start of line and enters Insert mode | 
| Normal | <kbd>A</kbd> | Moves cursor to end of line and enters Insert mode | 
| Insert | <kbd>ENTER</kbd> | Update active panel, goes into Normal mode | 
| Insert | <kbd>DEL</kbd> | Deletes one char at a time towards to end | 
| Insert | <kbd>BACKSPACE</kbd> | Deletes one char at a time towards to start | 
| Insert | <kbd>LEFT</kbd> | Moves cursor to left |  
| Insert | <kbd>RIGHT</kbd> | Moves cursor to right | 

## Purposes

- Exercize with Rust 
- Experiment with Immediate UI (especially TUI) idea.

## TODOs
- [ ] Add cursor highlight
- [ ] Add persistence

## References

- ImGUI: https://github.com/ocornut/imgui
- [Tsoding](https://github.com/tsoding) Stream's:
   * [Terminal To-Do App in Rust](https://www.youtube.com/watch?v=tR6p7ZC7RaU)
   * [Two Panel Interface in Rust](https://www.youtube.com/watch?v=Iveh2W3roJk)
   * [My Rust Skill Are Growing Stronger](https://www.youtube.com/watch?v=Uj0CrPM65Rc)
- Casey Muratori's Talk on Immediate-Mode GUI: https://www.youtube.com/watch?v=Z1qyvQsjK5Y