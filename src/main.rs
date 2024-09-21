use peg::board::BoardBuilder;
use ui::draw_board;

pub mod peg;
pub mod ui;

fn main() {
    let board = BoardBuilder::new(5, 5)
        .build();

    draw_board(&board);
}
