extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::Write;
use rand::Rng;

const ROWS:usize = 3;
const COLS:usize = 3;

const STATE_CELL_EMPTY:u32 = 0;

struct GameInputHandler {
    stdin: std::io::Stdin,
}

struct GameOutputHandler {
    stdout: termion::raw::RawTerminal<std::io::Stdout>,
}

impl GameInputHandler {
    fn new(stdin: std::io::Stdin) -> GameInputHandler {
        GameInputHandler{stdin: stdin}
    }
    fn handle_io_and_print(self, output_handler: &mut GameOutputHandler, game: &mut SlidingPuzzle) {
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
                },
                _ => (),
            }
            output_handler.draw(game, game.is_complete());
        }
    }
}

impl GameOutputHandler {
    fn new(stdout: termion::raw::RawTerminal<std::io::Stdout>) -> GameOutputHandler {
        GameOutputHandler{stdout: stdout}
    }
    fn draw(&mut self, game: &SlidingPuzzle, won: bool) {

        // clear screen
        write!(
            self.stdout, "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
        ).unwrap();

        // header
        print!("Sliding Puzzle [Experimental]\r\n");
        print!("-----------------------------\r\n\r\n");

        // print board
        let sym_top = "┌────┐";
        let sym_side = "│";
        let sym_bottom = "└────┘";

        print!("\r\n");
        let (rows, cols) = game.get_size();
        for i in 0..rows*3 {
            let r = i/3;
            for c in 0..cols {
                if i%3 == 0 {
                    print!("{}", sym_top);
                } else if i%3 == 1 {
                    let state = game.get_state(r, c);
                    let cell = if state == STATE_CELL_EMPTY {
                        String::from(" ")
                    } else if state <= 9 {
                        format!(" {}", state.to_string())
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

struct GameController {
    game: SlidingPuzzle,
    input_handler: GameInputHandler,
    output_handler: GameOutputHandler,
}
impl GameController {
    fn new() -> GameController {
        GameController{
            game: SlidingPuzzle::new(),
            input_handler: GameInputHandler::new(std::io::stdin()),
            output_handler: GameOutputHandler::new(std::io::stdout().into_raw_mode().unwrap()),
        }
    }
    fn run(mut self) {
        self.input_handler.handle_io_and_print(&mut self.output_handler, &mut self.game);
    }
}

trait Game {
    fn new() -> SlidingPuzzle;
    fn get_size(&self) -> (usize, usize);
    fn get_state(&self, r: usize, c: usize) -> u32;
    fn shuffle(&mut self, count: usize);
    fn move_up(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_down(&mut self);
    fn is_complete(&self) -> bool;
}

struct SlidingPuzzle {
    state: [[u32; COLS]; ROWS],
    empty_cell: (usize, usize),
}

impl Game for SlidingPuzzle {
    fn new() -> SlidingPuzzle {
        let mut game = SlidingPuzzle{
            state: [[0u32; COLS as usize]; ROWS as usize],
            empty_cell: (0, 0),
        };

        for i in 0..ROWS {
            for j in 0..COLS {
                game.state[i][j] = (i*COLS + j + 1) as u32;
            }
        }
        game.state[ROWS-1][COLS-1] = 0;
        game.empty_cell = (ROWS-1, COLS-1);

        let shuffle_count = ROWS*COLS*ROWS*COLS;
        game.shuffle(shuffle_count);

        game
    }
    fn get_size(&self) -> (usize, usize) {
        (ROWS, COLS)
    }
    fn get_state(&self, r: usize, c: usize) -> u32 {
        self.state[r][c]
    }
    fn shuffle(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let num: u32 = rng.gen();
            match num%4 {
                0 => self.move_up(),
                1 => self.move_left(),
                2 => self.move_right(),
                3 => self.move_down(),
                _ => (),
            }
        }
    }
    fn move_up(&mut self) {
        let (empty_r, empty_c) = self.empty_cell;
        if empty_r+1 < ROWS {
            self.state[empty_r][empty_c] = self.state[empty_r+1][empty_c];
            self.state[empty_r+1][empty_c] = STATE_CELL_EMPTY;
            self.empty_cell.0+=1;
        }
    }
    fn move_left(&mut self) {
        let (empty_r, empty_c) = self.empty_cell;
        if empty_c+1 < COLS {
            self.state[empty_r][empty_c] = self.state[empty_r][empty_c+1];
            self.state[empty_r][empty_c+1] = STATE_CELL_EMPTY;
            self.empty_cell.1+=1;
        }
    }
    fn move_right(&mut self) {
        let (empty_r, empty_c) = self.empty_cell;
        if empty_c > 0 {
            self.state[empty_r][empty_c] = self.state[empty_r][empty_c-1];
            self.state[empty_r][empty_c-1] = STATE_CELL_EMPTY;
            self.empty_cell.1-=1;
        }
    }
    fn move_down(&mut self) {
        let (empty_r, empty_c) = self.empty_cell;
        if empty_r > 0 {
            self.state[empty_r][empty_c] = self.state[empty_r-1][empty_c];
            self.state[empty_r-1][empty_c] = STATE_CELL_EMPTY;
            self.empty_cell.0-=1;
        }
    }

    fn is_complete(&self) -> bool {
        for i in 0..ROWS {
            for j in 0..COLS {
                if self.state[i][j]!=STATE_CELL_EMPTY && self.state[i][j] != (i*COLS + j + 1) as u32 {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    GameController::new().run();
}
