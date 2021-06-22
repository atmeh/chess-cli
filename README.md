# Chess

This is a small project I worked on to get familiar with Rust. I uploaded the code as a souvenir and also to make more space on my disk.

This is not a complete version of chess and is only a basic implementation of it.

# How it works

## Logic
The game works the same way you would play a game in real life, because you move any piece anywhere on the board with nothing restricting you. You could checkmate in the first move, but that makes the game boring. Pieces that are removed are permanetly removed. En passant, castling, and pawn promotion do not work.

## Instructions
The board is displayed each turn with the following letters representing the pieces:
* P is pawn
* N is knight
* B is bishop
* R is rook
* K is king
* Q is queen

You then input four numbers:
1. The first two numbers are the coordinates of the piece you want to move, for example `2 3`.
2. The second two numbers are the coordinates of where you want to piece to move, for example `2 4`.
The board is 8 by 8 like a normal chessboard. You want to make sure that you are using array-type logic, meaning that `1 8` should really be `0 7`

