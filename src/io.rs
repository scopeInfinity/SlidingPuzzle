use crate::game::SlidingPuzzle;

use std::io::Write;
use termion::event::Key;
use termion::input::TermRead;

use termion::raw::IntoRawMode;

pub struct GameInputHandler {
    stdin: std::io::Stdin,
}

pub struct GameOutputHandler {
    stdout: termion::raw::RawTerminal<std::io::Stdout>,
}

impl GameInputHandler {
    pub fn new(stdin: std::io::Stdin) -> GameInputHandler {
        GameInputHandler { stdin }
    }
    pub fn handle_io_and_print(
        self,
        output_handler: &mut GameOutputHandler,
        game: &mut SlidingPuzzle,
    ) {
        output_handler.draw(game, game.is_complete());
        for c in self.stdin.keys() {
            match c.unwrap() {
                Key::Up => game.move_up(),
                Key::Down => game.move_down(),
                Key::Left => game.move_left(),
                Key::Right => game.move_right(),
                Key::Char('q') => {
                    print!("Exiting\r\n");
                    return;
                }
                _ => (),
            }
            output_handler.draw(game, game.is_complete());
        }
    }
}

impl GameOutputHandler {
    pub fn new(stdout: std::io::Stdout) -> GameOutputHandler {
        GameOutputHandler {
            stdout: stdout.into_raw_mode().unwrap(),
        }
    }
    pub fn draw(&mut self, game: &SlidingPuzzle, won: bool) {
        // clear screen
        write!(
            self.stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
        )
        .unwrap();

        // header
        print!("Sliding Puzzle [Experimental]\r\n");
        print!("-----------------------------\r\n\r\n");

        // print board
        let sym_top = "┌────┐";
        let sym_side = "│";
        let sym_bottom = "└────┘";

        print!("\r\n");
        let (rows, cols) = game.get_size();
        for i in 0..rows * 3 {
            let r = i / 3;
            for c in 0..cols {
                if i % 3 == 0 {
                    print!("{}", sym_top);
                } else if i % 3 == 1 {
                    let (state, is_empty) = game.get_state(r, c);
                    let cell = if is_empty {
                        String::from(" ")
                    } else if state <= 9 {
                        format!(" {}", state)
                    } else {
                        state.to_string()
                    };
                    print!("{}{:^4}{}", sym_side, cell, sym_side);
                } else {
                    print!("{}", sym_bottom);
                }
            }
            print!("\r\n");
        }

        if won {
            print!("Puzzle Solved! Good game.\r\n");
        }

        // footer
        print!("\r\n");
        print!("Usage:\r\n");
        print!("  - Use arrow keys to slide cells\r\n");
        print!("  - Use 'q' to quit\r\n");

        self.stdout.flush().unwrap();
    }
}
