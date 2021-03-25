use std::collections::HashMap;

use ggez::filesystem::exists;

use crate::tetris::tetrimino::{MinoRotation, Tetrimino};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 22;

pub type MinoShape = Vec<Vec<bool>>;
pub type Board = [[bool; BOARD_WIDTH]; BOARD_HEIGHT];

#[derive(Copy, Clone)]
pub struct Game {
    confirmed_board: Board,
    dropping: Tetrimino,
    dropping_point: Point,
    dropping_rotation: MinoRotation,
}

impl Game {
    fn new() -> Game {
        Game {
            confirmed_board: [[false; BOARD_WIDTH]; BOARD_HEIGHT],
            dropping: Tetrimino::T,
            dropping_point: (4, (BOARD_HEIGHT - 20) as isize).into(),
            dropping_rotation: MinoRotation::Clockwise,
        }
    }

    fn board(&self) -> Board {
        let mut board = self.confirmed_board.clone();
        let shapes = self.dropping.shapes();
        let dropping_shape = shapes.get(&self.dropping_rotation).unwrap();

        let center = &self.dropping.center();
        let dropping_at = &self.dropping_point;

        for (mass_y, line) in dropping_shape.iter().enumerate() {
            for (mass_x, exists) in line.iter().enumerate() {
                let x = (dropping_at.x + (mass_x as isize) - center.x) as usize;
                let y = (dropping_at.y + (mass_y as isize) - center.y) as usize;

                board[y][x] = *exists;
            }
        }

        board
    }

    fn calc_dropping_mino_points(&self) -> Vec<Point> {
        let shapes = self.dropping.shapes();
        let shape = shapes.get(&self.dropping_rotation).unwrap();

        let center = &self.dropping.center();
        let dropping_at = &self.dropping_point;

        shape.iter()
            .enumerate()
            .flat_map(|(mass_y, line)| {
                line.iter()
                    .enumerate()
                    .flat_map(|(mass_x, &exists)| {
                        let x = dropping_at.x + (mass_x as isize) - center.x;
                        let y = dropping_at.y + (mass_y as isize) - center.y;

                        Some((x, y).into()).filter(|_| exists)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    pub fn try_move_left(&mut self) -> bool {
        self.try_move_x(-1)
    }

    pub fn try_move_right(&mut self) -> bool {
        self.try_move_x(1)
    }

    fn try_move_x(&mut self, addition: isize) -> bool {
        let clone = &mut self.clone();

        let manipulation = |game: &mut Game| {
            game.dropping_point.x += addition;
        };

        manipulation(clone);

        if clone.establishes_board() {
            manipulation(self);
        }

        clone.establishes_board()
    }

    pub fn try_spin_left(&mut self) -> bool {
        self.try_spin(RotateDirection::Left)
    }

    pub fn try_spin_right(&mut self) -> bool {
        self.try_spin(RotateDirection::Right)
    }

    fn try_spin(&mut self, direction: RotateDirection) -> bool {
        let rotation = match direction {
            RotateDirection::Left => self.dropping_rotation.left(),
            RotateDirection::Right => self.dropping_rotation.right(),
        };
        let offsets = self.dropping.spin_offsets();
        let offsets = offsets.get(&rotation).unwrap();

        let manipulation = |game: &mut Game, offset: &Point| {
            game.dropping_rotation = rotation;
            game.dropping_point.x += offset.x;
            game.dropping_point.y += offset.y;
        };

        offsets.into_iter()
            .find(|&offset| {
                let clone = &mut self.clone();
                manipulation(clone, offset);

                clone.establishes_board()
            })
            .map(|offset| manipulation(self, offset))
            .is_some()
    }

    fn establishes_board(&self) -> bool {
        self.calc_dropping_mino_points().iter()
            .all(|&point| {
                if !(0..(BOARD_HEIGHT as isize)).contains(&point.y)
                    || !(0..(BOARD_WIDTH as isize)).contains(&point.x) {
                    return false;
                }

                !self.confirmed_board[point.y as usize][point.x as usize]
            })
    }
}

#[derive(Copy, Clone)]
pub struct Point { x: isize, y: isize }

impl Into<Point> for (isize, isize) {
    fn into(self) -> Point {
        Point { x: self.0, y: self.1 }
    }
}

enum Movement {
    MoveLeft,
    MoveRight,
    DropSoftly,
    DropHardly,
    SpinLeft,
    SpinRight,
}

enum RotateDirection {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn move_right_once() {
        let game = &mut Game::new();

        assert!(game.try_move_right());

        assert_eq_board(
            &game.board(),
            &rect_vec!(
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
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
        )
    }

    #[test]
    fn move_right_to_limit() {
        let game = &mut Game::new();

        for _ in 0..4 {
            assert!(game.try_move_right());
        }
        assert!(!game.try_move_right());

        assert_eq_board(
            &game.board(),
            &rect_vec!(
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
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
        )
    }

    #[test]
    fn move_left_once() {
        let game = &mut Game::new();

        assert!(game.try_move_left());

        assert_eq_board(
            &game.board(),
            &rect_vec!(
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                [0, 0, 1, 1, 1, 0, 0, 0, 0, 0],
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
        )
    }

    #[test]
    fn move_left_to_limit() {
        let game = &mut Game::new();

        for _ in 0..3 {
            assert!(game.try_move_left());
        }
        assert!(!game.try_move_left());

        assert_eq_board(
            &game.board(),
            &rect_vec!(
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                [1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
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
        )
    }

    // TODO: まともにする
    fn print_board(board: &Board) {
        board
            .iter()
            .map(|&x| x.iter().map(|y| if *y { 1 } else { 0 }).collect::<Vec<_>>())
            .for_each(|line| println!("{:?}", line));
    }

    // TODO: まともにする
    fn assert_eq_board(left: &Board, right_vec: &Vec<Vec<bool>>) {
        let left_vec = left.iter().map(|&line| line.to_vec()).collect::<Vec<_>>();

        assert_eq!(&left_vec, right_vec);
    }
}