use std::collections::HashMap;

struct Game {
    board: [[bool; 10]; 22],
    dropping: Tetrimino,
    dropping_point: Point,
    dropping_rotation: MinoRotation,
}

type Point = (isize, isize);
type MinoShape = Vec<Vec<bool>>;
type Board = [[bool; 10]; 22];

macro_rules! mino_shape {
    ($($x:expr),+ $(,)?) => (
        [ $($x),+ ].iter().map(|line| line.iter().map(|c| *c > 0).collect()).collect()
    )
}

impl Game {
    fn new() -> Game {
        Game {
            board: [[false; 10]; 22],
            dropping: Tetrimino::T,
            dropping_point: (4, 22 - 20),
            dropping_rotation: MinoRotation::Clockwise,
        }
    }

    fn board(&self) -> Board {
        let mut board = self.board.clone();
        let shapes = self.dropping.shapes();
        let dropping_shape = shapes.get(&self.dropping_rotation).unwrap();

        let (center_x, center_y) = self.dropping.center();
        let (dropping_x, dropping_y) = self.dropping_point;

        for (mass_y, line) in dropping_shape.iter().enumerate() {
            for (mass_x, exists) in line.iter().enumerate() {
                let x = (dropping_x as usize) + mass_x - (center_x as usize);
                let y = (dropping_y as usize) + mass_y - (center_y as usize);

                board[y][x] = *exists;
            }
        }

        board
    }
}

enum Tetrimino {
    T,
}

// clockwise angles starts at 12 o'clock position
#[derive(PartialEq, Eq, Hash)]
enum MinoRotation {
    Clockwise,
    Clockwise90,
    Clockwise180,
    Clockwise270,
}

impl Tetrimino {
    fn center(&self) -> Point {
        match self {
            Tetrimino::T => (1, 1)
        }
    }

    fn edge_length(&self) -> usize {
        match self {
            Tetrimino::T => 3,
        }
    }

    fn shapes(&self) -> HashMap<MinoRotation, MinoShape> {
        match self {
            Tetrimino::T => maplit::hashmap! {
                MinoRotation::Clockwise => mino_shape!(
                        [0, 1, 0],
                        [1, 1, 1],
                        [0, 0, 0],
                ),
                MinoRotation::Clockwise90 => mino_shape!(
                        [0, 1, 0],
                        [0, 1, 1],
                        [0, 1, 0],
                ),
                MinoRotation::Clockwise180 => mino_shape!(
                        [0, 0, 0],
                        [1, 1, 1],
                        [0, 1, 0],
                ),
                MinoRotation::Clockwise270 => mino_shape!(
                        [0, 1, 0],
                        [1, 1, 0],
                        [0, 1, 0],
                ),
            }
        }
    }

    fn spin_offsets(&self) -> HashMap<MinoRotation, Vec<Point>> {
        match self {
            Tetrimino::T => maplit::hashmap! {
                MinoRotation::Clockwise => vec!(
                        (0, 0),
                        (-1, 0),
                        (-1, -1),
                        (0, 2),
                        (-1, 2),
                ),
                MinoRotation::Clockwise90 => vec!(
                        (0, 0),
                        (1, 0),
                        (1, 1),
                        (0, -2),
                        (1, -2),
                ),
                MinoRotation::Clockwise180 => vec!(
                        (0, 0),
                        (1, 0),
                        (1, -1),
                        (0, 2),
                        (1, 2),
                ),
                MinoRotation::Clockwise270 => vec!(
                        (0, 0),
                        (-1, 0),
                        (-1, 1),
                        (0, -2),
                        (-1, -2),
                ),
            }
        }
    }
}

#[test]
// FIXME: separate
fn test_game() {
    let board = Game::new();

    assert_eq!(
        board.board,
        [
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
        ]
    );

    let board_with_dropping = Game::new().board();
    print_board(&board_with_dropping);

    assert_eq!(
        board_with_dropping,
        [
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, true, false, false, false, false, false],
            [false, false, false, true, true, true, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false],
        ]
    );

    let mino_shape: MinoShape = mino_shape!(
        [0, 1, 0, 1],
        [1, 1, 0, 0],
        [0, 0, 1, 0],
        [0, 1, 1, 0],
    );
    assert_eq!(
        mino_shape,
        vec!(
            vec!(false, true, false, true),
            vec!(true, true, false, false),
            vec!(false, false, true, false),
            vec!(false, true, true, false),
        )
    )
}

fn print_board(board: &Board) {
    board
        .iter()
        .map(|x| x.iter().map(|y| if *y { 1 } else { 0 }).collect::<Vec<_>>())
        .for_each(|line| println!("{:?}", line));
}