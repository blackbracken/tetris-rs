use std::collections::VecDeque;
use std::convert::TryFrom;

use crate::tetris::game::{MinoBlock, Point, SpinDirection};
use crate::tetris::tetrimino::{MinoRotation, Tetrimino, WallKickOffset};

pub const FIELD_UNIT_WIDTH: usize = 10;
pub const FIELD_UNIT_HEIGHT: usize = 21;
pub const FIELD_VISIBLE_UNIT_HEIGHT: usize = 20;

pub type Field = [[MinoEntity; FIELD_UNIT_WIDTH]; FIELD_UNIT_HEIGHT];
pub type RemovedLines = Vec<usize>;

pub const SPAWN_POINT: Point = Point { x: 4, y: 1 };

#[derive(Copy, Clone)]
pub struct Board {
    pub confirmed_field: Field,
    pub dropping: Tetrimino,
    dropping_point: Point,
    dropping_rotation: MinoRotation,
}

impl Board {
    pub fn new(dropping: Tetrimino) -> Board {
        Board::new_with_field(dropping, [[MinoEntity::AIR; FIELD_UNIT_WIDTH]; FIELD_UNIT_HEIGHT])
    }

    fn new_with_field(dropping: Tetrimino, field: Field) -> Board {
        Board {
            confirmed_field: field,
            dropping,
            dropping_point: SPAWN_POINT,
            dropping_rotation: MinoRotation::default(),
        }
    }

    pub fn field(&self) -> Field {
        let mut field = self.confirmed_field.to_owned();
        let shapes = self.dropping.shapes();
        let shape = shapes.get(&self.dropping_rotation).unwrap();

        let center = &self.dropping.center();
        let dropping_at = &self.dropping_point;

        for (block_y, line) in shape.iter().enumerate() {
            for (block_x, &exists) in line.iter().enumerate() {
                let x = (dropping_at.x + (block_x as isize) - center.x) as usize;
                let y = (dropping_at.y + (block_y as isize) - center.y) as usize;

                if exists {
                    field[y][x] = self.dropping.block().into();
                }
            }
        }

        field
    }

    pub fn spawn(&mut self, dropping: Tetrimino) -> bool {
        self.dropping = dropping;
        self.dropping_point = SPAWN_POINT;
        self.dropping_rotation = MinoRotation::Clockwise;

        self.establishes_field()
    }

    pub fn try_move_x(&mut self, addition: isize) -> bool {
        let clone = &mut self.to_owned();

        let manipulation = |board: &mut Board| {
            board.dropping_point.x += addition;
        };

        manipulation(clone);

        if clone.establishes_field() {
            manipulation(self);
        }

        clone.establishes_field()
    }

    pub fn try_spin(&mut self, direction: SpinDirection) -> Option<Spin> {
        fn spin_with_offset(board: &mut Board, direction: &SpinDirection, offset: &WallKickOffset) {
            board.dropping_rotation = board.dropping_rotation.spin(direction);
            board.dropping_point.x += offset.x;
            board.dropping_point.y += offset.y;
        }

        let offset = (0..5).into_iter()
            .flat_map(|idx| {
                let offsets = self.dropping.wall_kick_offsets(&self.dropping_rotation, &direction);
                offsets.get(idx).map(|o| o.to_owned())
            })
            .find(|offset| {
                let mut clone = self.to_owned();
                spin_with_offset(&mut clone, &direction, offset);

                clone.establishes_field()
            });

        if let Some(offset) = offset {
            spin_with_offset(self, &direction, &offset);

            if self.satisfies_cond_for_t_spin() {
                Some(Spin::TSpin)
            } else {
                Some(Spin::Normal)
            }
        } else {
            None
        }
    }

    pub fn satisfies_cond_for_t_spin(&self) -> bool {
        let field = self.field();
        let unfilled_corners_count = [1, -1].iter()
            .flat_map(|&y| usize::try_from(self.dropping_point.y + y))
            .flat_map(|y| {
                [1, -1].iter()
                    .flat_map(|&x| usize::try_from(self.dropping_point.x + x))
                    .flat_map(|x| {
                        field
                            .get(y)
                            .and_then(|line| line.get(x))
                            .filter(|&entity| entity.is_air())
                    })
                    .collect::<Vec<_>>()
            })
            .count();
        self.dropping_mino_is_on_ground() && unfilled_corners_count <= 1
    }

    pub fn drop_one(&mut self) -> bool {
        self.dropping_point.y += 1;

        let could_drop = self.establishes_field();
        if !could_drop {
            self.dropping_point.y -= 1;
        }

        could_drop
    }

    pub fn hard_drop(&mut self) -> usize {
        let mut n = 0;
        while self.drop_one() {
            n += 1;
        }

        n
    }

    pub fn determine_dropping_mino(&mut self) {
        for p in self.dropping_mino_points() {
            self.confirmed_field[p.y as usize][p.x as usize] = self.dropping.block().into();
        }
    }

    pub fn calc_dropping_mino_prediction(&self) -> Vec<Point> {
        let mut clone = self.to_owned();
        clone.dropping_point.y += self.dropping_mino_height_from_ground() as isize;

        clone.dropping_mino_points()
    }

    pub fn remove_lines(&mut self) -> usize {
        let removed_lines = self.filled_lines();

        let mut field: VecDeque<Vec<MinoEntity>> = self.confirmed_field.iter()
            .enumerate()
            .filter(|(idx, _)| !removed_lines.contains(idx))
            .map(|(_, line)| Box::new(line).to_vec())
            .collect::<Vec<_>>()
            .into();
        for _ in 0..removed_lines.len() {
            field.push_front([MinoEntity::AIR; FIELD_UNIT_WIDTH].to_vec())
        }

        for y in 0..FIELD_UNIT_HEIGHT {
            for x in 0..FIELD_UNIT_WIDTH {
                self.confirmed_field[y][x] = field[y][x];
            }
        }

        removed_lines.len()
    }

    pub fn filled_lines(&self) -> RemovedLines {
        self.field().iter().enumerate()
            .filter(|(_, line)| line.iter().all(|entity| !entity.is_air()))
            .map(|(y, _)| y)
            .collect::<Vec<_>>()
    }

    pub fn dropping_mino_points(&self) -> Vec<Point> {
        let shapes = self.dropping.shapes();
        let shape = shapes.get(&self.dropping_rotation).unwrap();

        let center = &self.dropping.center();
        let dropping_at = &self.dropping_point;

        shape.iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .flat_map(|(x, &exists)| {
                        let x = dropping_at.x + (x as isize) - center.x;
                        let y = dropping_at.y + (y as isize) - center.y;

                        Some((x, y).into()).filter(|_| exists)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    pub fn dropping_mino_is_on_ground(&self) -> bool {
        self.dropping_mino_height_from_ground() == 0
    }

    fn dropping_mino_height_from_ground(&self) -> usize {
        let mut clone = self.to_owned();
        let mut height = 0;
        loop {
            clone.dropping_point.y += 1;

            if clone.establishes_field() {
                height += 1;
            } else {
                break;
            }
        }

        height
    }

    fn establishes_field(&self) -> bool {
        self.dropping_mino_points().iter()
            .all(|&point| {
                if let Ok(x) = usize::try_from(point.x) {
                    if let Ok(y) = usize::try_from(point.y) {
                        let entity = self.confirmed_field
                            .get(y)
                            .and_then(|line| line.get(x));

                        if let Some(entity) = entity {
                            return entity.block().is_none();
                        }
                    }
                }

                false
            })
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum MinoEntity {
    AQUA,
    YELLOW,
    PURPLE,
    BLUE,
    ORANGE,
    GREEN,
    RED,

    AIR,
}

impl MinoEntity {
    pub fn block(&self) -> Option<MinoBlock> {
        use MinoBlock::*;

        match self {
            MinoEntity::AQUA => Some(AQUA),
            MinoEntity::YELLOW => Some(YELLOW),
            MinoEntity::PURPLE => Some(PURPLE),
            MinoEntity::BLUE => Some(BLUE),
            MinoEntity::ORANGE => Some(ORANGE),
            MinoEntity::GREEN => Some(GREEN),
            MinoEntity::RED => Some(RED),
            MinoEntity::AIR => None,
        }
    }

    pub fn is_air(&self) -> bool {
        self == &MinoEntity::AIR
    }
}

pub enum Spin {
    Normal,
    TSpin,
}

#[cfg(test)]
//noinspection ALL
mod tests {
    use super::*;

    type ExistenceField = [[bool; FIELD_UNIT_WIDTH]; FIELD_UNIT_HEIGHT];

    trait LikeExistenceField {
        fn to_existences(&self) -> ExistenceField;

        fn similar<F: LikeExistenceField>(&self, to: F) -> bool {
            self.to_existences() == to.to_existences()
        }

        fn gen_field(&self) -> Field {
            let mut f = [[MinoEntity::AIR; FIELD_UNIT_WIDTH]; FIELD_UNIT_HEIGHT];
            for (y, line) in self.to_existences().iter().enumerate() {
                for (x, &exists) in line.iter().enumerate() {
                    if exists {
                        f[y][x] = MinoEntity::PURPLE;
                    }
                }
            }

            f
        }
    }

    impl LikeExistenceField for Field {
        fn to_existences(&self) -> ExistenceField {
            let mut f = ExistenceField::default();
            self.iter().enumerate().for_each(|(y, line)| {
                line.iter().enumerate().for_each(|(x, entity)| {
                    f[y][x] = entity.block().is_some();
                })
            });

            f
        }
    }

    impl LikeExistenceField for [[i32; FIELD_UNIT_WIDTH]; FIELD_UNIT_HEIGHT] {
        fn to_existences(&self) -> ExistenceField {
            let mut f = ExistenceField::default();
            self.iter().enumerate().for_each(|(y, line)| {
                line.iter().enumerate().for_each(|(x, &n)| {
                    f[y][x] = n > 0;
                })
            });

            f
        }
    }

    #[test]
    fn field_is_only_with_dropping_on_init() {
        let board = Board::new(Tetrimino::T);

        let expected = [
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
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn f() {
        let board = Board::new(Tetrimino::T);

        assert_eq!(board.dropping_mino_height_from_ground(), 18);
    }

    #[test]
    fn move_right_once() {
        let mut board = Board::new(Tetrimino::T);
        assert!(board.try_move_x(1));

        let expected = [
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
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn move_right_to_limit() {
        let mut board = Board::new(Tetrimino::T);
        assert!(board.try_move_x(4));
        assert!(!board.try_move_x(1));

        let expected = [
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
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn move_left_once() {
        let mut board = Board::new(Tetrimino::T);
        assert!(board.try_move_x(-1));

        let expected = [
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
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn move_left_to_limit() {
        let mut board = Board::new(Tetrimino::T);
        assert!(board.try_move_x(-3));
        assert!(!board.try_move_x(-1));

        let expected = [
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
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn soft_drop_t() {
        let mut board = Board::new(Tetrimino::T);

        assert_eq!(board.drop_one(), DroppingMinoStatus::InAir);

        let expected = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
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
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn hard_drop_t() {
        let mut board = Board::new(Tetrimino::T);

        assert_eq!(board.hard_drop(), Some(19));

        let expected = [
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
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn hard_drop_vertical_i() {
        let mut board = Board::new(Tetrimino::I);

        assert!(board.try_move_x(1));
        assert!(board.try_spin(SpinDirection::Right));
        assert_eq!(board.hard_drop(), Some(17));

        let expected = [
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
            [0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn dt_cannon() {
        let dt_field = [
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
            [0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
            [1, 1, 0, 1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1, 1, 1, 1, 1, 1],
            [1, 1, 0, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 0, 1, 1, 1, 1, 1, 1, 1],
        ];
        let mut board = Board::new_with_field(Tetrimino::T, dt_field.gen_field());

        // TODO: test
        assert!(true);
    }
}