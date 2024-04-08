use clearscreen::clear;
use crate::board::Board;

/// Game system, storing game board and number of turns taken
struct Game {
    board: Board,
    turn: u32,
} impl Game {
    fn new_game(rings: usize, towers: usize) -> Game {
        let board = Board::new(rings, towers);
        let turn = 1;
        Game { board, turn }
    }
}

/// Set up a turn-based version of the game
pub fn play_sequential(rings: usize, towers: usize) {
    let mut game: Game = Game::new_game(rings, towers);
    loop {
        let _ = clear();
        println!("Turn {}", game.turn);
        game.board.print();
        println!("(enter 'q' to quit)");
        println!("Move ring from tower (1-{}): ", towers);
        let mut from: String = String::new();
        let _ = std::io::stdin().read_line(&mut from);
        if from.trim() == "q" { return; }
        println!("to tower (1-{}): ", towers);
        let mut to: String = String::new();
        let _ = std::io::stdin().read_line(&mut to);
        if to.trim() == "q" { break; }
        let from_i: usize = from
            .trim()
            .parse()
            .expect("Unable to parse input");
        let to_i: usize = to
            .trim()
            .parse()
            .expect("Unable to parse input");
        if game.board.move_rings(from_i-1, to_i-1) { game.turn += 1; }
        if game.board.check_victory() { break; }
    }
    let _ = clear();
    game.board.print();
    println!("Congrats! You won in {} turns!", game.turn);
}
