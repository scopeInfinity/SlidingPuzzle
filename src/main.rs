extern crate termion;
mod game;
mod io;

use game::SlidingPuzzle;
use io::GameInputHandler;
use io::GameOutputHandler;

struct GameController {
    game: SlidingPuzzle,
    input_handler: GameInputHandler,
    output_handler: GameOutputHandler,
}
impl GameController {
    fn new() -> GameController {
        GameController {
            game: SlidingPuzzle::new(),
            input_handler: GameInputHandler::new(std::io::stdin()),
            output_handler: GameOutputHandler::new(std::io::stdout()),
        }
    }
    fn run(mut self) {
        self.input_handler
            .handle_io_and_print(&mut self.output_handler, &mut self.game);
    }
}
fn main() {
    GameController::new().run();
}
