pub fn movement(board: &mut [[usize; 8]; 8]) {
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).expect("ERROR: Reading input failed.");
    let n = inp.split_whitespace();
    let mut x: usize;
    let mut coms: Vec<usize> = vec![];
    for i in n {
        x = i.parse::<usize>().unwrap();
        coms.push(x);
    }
    let first = coms[0];
    let second = coms[1];
    let third = coms[2];
    let fourth = coms[3];
    board[third][fourth] = board[first][second];
    board[first][second] = 0;
}
