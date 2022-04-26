use rand::Rng;

const ROWS: usize = 3;
const COLS: usize = 3;

const STATE_CELL_EMPTY: u32 = 0;

pub struct SlidingPuzzle {
    state: [[u32; COLS]; ROWS],
    empty_cell: (usize, usize),
}

impl SlidingPuzzle {
    pub fn new() -> SlidingPuzzle {
        let mut game = SlidingPuzzle {
            state: [[0u32; COLS as usize]; ROWS as usize],
            empty_cell: (0, 0),
        };

        for i in 0..ROWS {
            for j in 0..COLS {
                game.state[i][j] = (i * COLS + j + 1) as u32;
            }
        }
        game.state[ROWS - 1][COLS - 1] = 0;
        game.empty_cell = (ROWS - 1, COLS - 1);

        let shuffle_count = ROWS * COLS * ROWS * COLS;
        game.shuffle(shuffle_count);

        game
    }
    pub fn get_size(&self) -> (usize, usize) {
        (ROWS, COLS)
    }
    pub fn get_state(&self, r: usize, c: usize) -> (u32, bool) {
        (self.state[r][c], (self.state[r][c] == STATE_CELL_EMPTY))
    }
    fn shuffle(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let num: u32 = rng.gen();
            match num % 4 {
                0 => self.move_up(),
                1 => self.move_left(),
                2 => self.move_right(),
                3 => self.move_down(),
                _ => (),
            }
        }
    }
    pub fn move_up(&mut self) {
        let (empty_r, empty_c) = self.empty_cell;
        if empty_r + 1 < ROWS {
            self.state[empty_r][empty_c] = self.state[empty_r + 1][empty_c];
            self.state[empty_r + 1][empty_c] = STATE_CELL_EMPTY;
            self.empty_cell.0 += 1;
        }
    }
    pub fn move_left(&mut self) {
        let (empty_r, empty_c) = self.empty_cell;
        if empty_c + 1 < COLS {
            self.state[empty_r][empty_c] = self.state[empty_r][empty_c + 1];
            self.state[empty_r][empty_c + 1] = STATE_CELL_EMPTY;
            self.empty_cell.1 += 1;
        }
    }
    pub fn move_right(&mut self) {
        let (empty_r, empty_c) = self.empty_cell;
        if empty_c > 0 {
            self.state[empty_r][empty_c] = self.state[empty_r][empty_c - 1];
            self.state[empty_r][empty_c - 1] = STATE_CELL_EMPTY;
            self.empty_cell.1 -= 1;
        }
    }
    pub fn move_down(&mut self) {
        let (empty_r, empty_c) = self.empty_cell;
        if empty_r > 0 {
            self.state[empty_r][empty_c] = self.state[empty_r - 1][empty_c];
            self.state[empty_r - 1][empty_c] = STATE_CELL_EMPTY;
            self.empty_cell.0 -= 1;
        }
    }
    pub fn is_complete(&self) -> bool {
        for i in 0..ROWS {
            for j in 0..COLS {
                if self.state[i][j] != STATE_CELL_EMPTY
                    && self.state[i][j] != (i * COLS + j + 1) as u32
                {
                    return false;
                }
            }
        }
        true
    }
}
