extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::Write;

const ROWS:usize = 5;
const COLS:usize = 5;

const STATE_CELL_EMPTY:u32 = 0;

trait Game {
    fn init(&mut self);
    fn get_empty_cell(&self) -> (usize, usize);
    fn move_up(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_down(&mut self);
    fn draw(&self, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>);
    fn step(&mut self);
}

struct SlidingPuzzle {
    state: [[u32; COLS]; ROWS],
}

impl Game for SlidingPuzzle {
    fn init(&mut self) {
        for i in 0..ROWS {
            for j in 0..COLS {
                self.state[i][j] = (i*COLS + j + 1) as u32;
            }
        }
        self.state[ROWS-1][COLS-1] = 0;
    }
    fn get_empty_cell(&self) -> (usize, usize) {
        for i in 0..ROWS {
            for j in 0..COLS {
                if self.state[i][j] == STATE_CELL_EMPTY {
                    return (i, j);
                }
            }
        }
        panic!("no empty cell found!");
    }
    fn move_up(&mut self) {
        let (empty_r, empty_c) = self.get_empty_cell();
        if empty_r+1 < ROWS {
            self.state[empty_r][empty_c] = self.state[empty_r+1][empty_c];
            self.state[empty_r+1][empty_c] = STATE_CELL_EMPTY;
        }
    }
    fn move_left(&mut self) {
        let (empty_r, empty_c) = self.get_empty_cell();
        if empty_c+1 < COLS {
            self.state[empty_r][empty_c] = self.state[empty_r][empty_c+1];
            self.state[empty_r][empty_c+1] = STATE_CELL_EMPTY;
        }
    }
    fn move_right(&mut self) {
        let (empty_r, empty_c) = self.get_empty_cell();
        if empty_c > 0 {
            self.state[empty_r][empty_c] = self.state[empty_r][empty_c-1];
            self.state[empty_r][empty_c-1] = STATE_CELL_EMPTY;
        }
    }
    fn move_down(&mut self) {
        let (empty_r, empty_c) = self.get_empty_cell();
        if empty_r > 0 {
            self.state[empty_r][empty_c] = self.state[empty_r-1][empty_c];
            self.state[empty_r-1][empty_c] = STATE_CELL_EMPTY;
        }
    }
    fn draw(&self, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
        // clear screen
        write!(
            stdout, "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
        ).unwrap();

        // header
        print!("Sliding Puzzle\r\n");
        print!("--------------\r\n\r\n");

        // print board
        const CELL_WIDTH:usize = 5;
        for _ in 0..COLS*CELL_WIDTH+1 {
            print!("-");
        }
        print!("\r\n");
        for i in 0..ROWS {
            print!("| ");
            for j in 0..COLS {
                let cell = self.state[i as usize][j as usize];
                if cell == STATE_CELL_EMPTY {
                    print!("{:2} | ", ' ');
                } else {
                    print!("{:2} | ", cell);
                }
            }
            print!("\r\n");
            for _ in 0..COLS*CELL_WIDTH+1 {
                print!("-");
            }
            print!("\r\n");
        }

        // footer
        print!("\r\n");
        print!("Usage:\r\n");
        print!("  - Use arrow keys to slide cells\r\n");
        print!("  - Use 'q' to quit\r\n");

        stdout.flush().unwrap();
    }

    fn step(&mut self) {

    }
}

fn main() {
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let stdin = std::io::stdin();

    let mut game = SlidingPuzzle{state: [[0u32; COLS as usize]; ROWS as usize]};
    game.init();
    game.draw(&mut stdout);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Up => game.move_up(),
            Key::Down => game.move_down(),
            Key::Left => game.move_left(),
            Key::Right => game.move_right(),
            Key::Char('q') => {
                return;
            },
            _ => (),
        }
        game.draw(&mut stdout);
    }
}
