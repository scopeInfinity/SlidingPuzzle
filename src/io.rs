pub mod console;

use crate::game::SlidingPuzzle;

pub trait GameInputHandler {
    fn handle_io_and_print(
        self,
        output_handler: &mut dyn GameOutputHandler,
        game: &mut SlidingPuzzle,
    );
}

pub trait GameOutputHandler {
    fn draw(&mut self, game: &SlidingPuzzle, won: bool);
}
