use std::collections::HashMap;

struct Game {
    board: [[bool; 10]; 22],
    dropping: Tetrimino,
    dropping_point: Point,
    dropping_rotation: MinoRotation,
}

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 22;

pub type Point = (isize, isize);
pub type MinoShape = Vec<Vec<bool>>;
pub type Board = [[bool; BOARD_WIDTH]; BOARD_HEIGHT];

macro_rules! rect_vec {
    ($($x:expr),+ $(,)?) => (
        [ $($x),+ ].iter().map(|line| line.iter().map(|c| *c > 0).collect()).collect()
    )
}

impl Game {
    fn new() -> Game {
        Game {
            board: [[false; BOARD_WIDTH]; BOARD_HEIGHT],
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
                MinoRotation::Clockwise => rect_vec!(
                        [0, 1, 0],
                        [1, 1, 1],
                        [0, 0, 0],
                ),
                MinoRotation::Clockwise90 => rect_vec!(
                        [0, 1, 0],
                        [0, 1, 1],
                        [0, 1, 0],
                ),
                MinoRotation::Clockwise180 => rect_vec!(
                        [0, 0, 0],
                        [1, 1, 1],
                        [0, 1, 0],
                ),
                MinoRotation::Clockwise270 => rect_vec!(
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
fn board_is_only_with_dropping_on_init() {
    let game = Game::new();

    assert_eq_board(
        &game.board(),
        &rect_vec!(
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ),
    );
}

#[test]
fn rect_vec_returns_2d_vec() {
    let rect_vec: Vec<Vec<bool>> = rect_vec!(
        [0, 0, 0, 0],
        [1, 0, 1, 0],
        [0, 1, 0, 1],
        [1, 1, 1, 1],
    );

    assert_eq!(
        rect_vec,
        vec!(
            vec!(false, false, false, false),
            vec!(true, false, true, false),
            vec!(false, true, false, true),
            vec!(true, true, true, true),
        )
    );
}

// TODO: まともにする
fn print_board(board: &Board) {
    board
        .iter()
        .map(|x| x.iter().map(|y| if *y { 1 } else { 0 }).collect::<Vec<_>>())
        .for_each(|line| println!("{:?}", line));
}

// TODO: まともにする
fn assert_eq_board(left: &Board, right_vec: &Vec<Vec<bool>>) {
    let left_vec = left.iter().map(|line| line.to_vec()).collect::<Vec<_>>();

    assert_eq!(&left_vec, right_vec);
}