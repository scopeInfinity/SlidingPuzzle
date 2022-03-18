const ROWS:u32 = 5;
const COLS:u32 = 5;

const STATE_CELL_EMPTY:u32 = 0;

fn create_state() -> [[u32; COLS as usize]; ROWS as usize] {
    let mut state = [[0u32; COLS as usize]; ROWS as usize];
    for i in 0..ROWS {
        for j in 0..COLS {
            state[i as usize][j as usize] = i*COLS + j + 1;
        }
    }
    state[(ROWS-1) as usize][(COLS-1) as usize] = 0;
    return state;
}

fn print_state(state: [[u32; COLS as usize]; ROWS as usize]) {
    const CELL_WIDTH:u32 = 5;
    for j in 0..COLS*CELL_WIDTH+1 {
        print!("-");
    }
    println!();
    for i in 0..ROWS {
        print!("| ");
        for j in 0..COLS {
            let cell = state[i as usize][j as usize];
            if (cell == STATE_CELL_EMPTY) {
                print!("{:2} | ", ' ');
            } else {
                print!("{:2} | ", cell);
            }
        }
        println!();
        for j in 0..COLS*CELL_WIDTH+1 {
            print!("-");
        }
        println!();
    }
}

fn main() {
    let state = create_state();
    print_state(state);
}
