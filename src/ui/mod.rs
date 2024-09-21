use crate::peg::board::Board;

pub fn draw_board(board: &Board) {
    for r in 0..board.r() {
        draw_row_separator(board.c());
        print!("|");
        for c in 0..board.c() {
            print!(" x |")
        }
        println!();
    }
    draw_row_separator(board.c());
}


fn draw_row_separator(cols: u8) {
    print!("+");
    for c in 0..cols {
        print!("---+");
    }
    println!();
}
