extern crate termion;
mod game;
mod io;

use game::SlidingPuzzle;
use io::console::ConsoleInputHandler;
use io::console::ConsoleOutputHandler;
use io::GameInputHandler;

struct GameController {
    game: SlidingPuzzle,
    input_handler: ConsoleInputHandler,
    output_handler: ConsoleOutputHandler,
}
impl GameController {
    fn new() -> GameController {
        GameController {
            game: SlidingPuzzle::new(),
            input_handler: ConsoleInputHandler::new(std::io::stdin()),
            output_handler: ConsoleOutputHandler::new(std::io::stdout()),
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
