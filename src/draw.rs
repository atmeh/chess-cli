pub fn draw(board: &[[usize; 8]; 8]) {
    for row in board {
        for square in row {
            match square {
                0 => print!("*"),
                1 => print!("P"),
                2 => print!("N"),
                3 => print!("B"),
                4 => print!("R"),
                5 => print!("Q"),
                6 => print!("K"),
                _ => panic!("ERROR: Invalid Integer in Board."),
            }
            print!(" ");
        }
        println!();
    }
}
