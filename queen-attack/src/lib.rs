#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

fn is_valid_chess_line(line: i32) -> bool {
    line >= 0 && line <= 7
}

fn is_same_line(queen1: &Queen, queen2: &Queen) -> bool {
    queen1.position.file == queen2.position.file || queen1.position.rank == queen2.position.rank
}

fn is_same_diagonal(queen1: &Queen, queen2: &Queen) -> bool {
    (queen1.position.file - queen2.position.file).abs()
        == (queen1.position.rank - queen2.position.rank).abs()
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (is_valid_chess_line(rank), is_valid_chess_line(file)) {
            (true, true) => Some(ChessPosition { rank, file }),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        is_same_line(self, other) || is_same_diagonal(self, other)
    }
}
