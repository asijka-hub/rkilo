use termion::*;
use std::io;
use std::env;
use std::fs;
use std::process::exit;
use termion::input::TermRead;
use termion::event::{Key, Event, MouseEvent};
use std::io::{Write, stdout, stdin, Cursor};
use termion::cursor::DetectCursorPos;
use termion::raw::IntoRawMode;

fn clear_screen() {
    println!("{}", clear::All);
}

fn print_help_msg() {
    println!("q,Q to quit");
}

struct Position {
    x: u16,
    y: u16
}

impl Position {
    fn from_tuple(coords: (u16, u16)) -> Position {
        Position {
            x: coords.0,
            y: coords.1
        }
    }
}

fn main() {
    let argn = env::args().len();
    let args:Vec<String> = env::args().collect();

    if argn != 2 {
        eprintln!("usage: ./rkilo <file-name>  // to edit file");
        eprintln!("usage: ./rkilo -h // for help");
        exit(1);
    }

    if args[1] == "-h" {
        print_help_msg();
        exit(0);
    };

    let input = fs::read_to_string(&args[1]).expect("could not open file");

    let _stdout = stdout().into_raw_mode().unwrap();
    let mut cursor_pos = Position::from_tuple(stdout().cursor_pos().unwrap());

    clear_screen();
    println!("{}", input);

    for c in stdin().events() {
        clear_screen();
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) |  Event::Key(Key::Char('Q')) => break,
            a => { print!("{:#?}", a);}
        }
        println!("{}", input);
    }
}
