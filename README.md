# todors
Simple VIM-like terminal Todo App in Rust.

## Pre-Requisite

Note that you must to have the ncurses library installed and linkable. On Linux, this should be trivial. On OS X, consider installing ncurses using Homebrew. (Note that you have to force Homebrew to link the library to /usr/local/lib: brew link --force ncurses and set that path to LIBRARY_PATH environmental variable.)

## Quick Start
```console
$ cargo run
```

## Controls

|Key|Description|
|---|----|
|<kbd>q</kbd>| Quits application|
|<kbd>j</kbd>| Goes one item down in active list|
|<kbd>g</kbd>| Goes first item in active list|
|<kbd>d</kbd>| Deletes item in DONE list |
|<kbd>G</kbd>| Goes last item in active list|
|<kbd>k</kbd>| Goes one item up in active list|
|<kbd>x</kbd>| Performs transfer from active to other list|

## References
- TBD