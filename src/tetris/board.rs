use std::convert::TryFrom;

use crate::tetris::game::{MinoBlock, Point, SpinDirection};
use crate::tetris::tetrimino::{MinoRotation, Tetrimino};

pub const FIELD_UNIT_WIDTH: usize = 10;
pub const FIELD_UNIT_HEIGHT: usize = 22;
pub const FIELD_VISIBLE_UNIT_HEIGHT: usize = 20;

pub type Field = [[MinoEntity; FIELD_UNIT_WIDTH]; FIELD_UNIT_HEIGHT];

pub const SPAWN_POINT: Point = Point { x: 4, y: 2 };

#[derive(Copy, Clone)]
pub struct Board {
    confirmed_field: Field,
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
        let mut field = self.confirmed_field.clone();
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

    pub fn dropping_mino_status(&self) -> DroppingMinoStatus {
        let mut clone = self.clone();
        clone.dropping_point.y += 1;

        if clone.establishes_field() {
            DroppingMinoStatus::InAir
        } else {
            DroppingMinoStatus::OnGround
        }
    }

    pub fn try_move_x(&mut self, addition: isize) -> bool {
        let clone = &mut self.clone();

        let manipulation = |board: &mut Board| {
            board.dropping_point.x += addition;
        };

        manipulation(clone);

        if clone.establishes_field() {
            manipulation(self);
        }

        clone.establishes_field()
    }

    pub fn try_spin(&mut self, direction: SpinDirection) -> bool {
        let rotation = match direction {
            SpinDirection::Left => self.dropping_rotation.left(),
            SpinDirection::Right => self.dropping_rotation.right(),
        };
        let kicks = self.dropping.wall_kicks();
        let kicks = kicks.get(&rotation).unwrap();

        let manipulation = |board: &mut Board, kick: &Point| {
            let mul = match direction {
                SpinDirection::Left => -1,
                SpinDirection::Right => 1,
            };
            board.dropping_rotation = rotation;
            board.dropping_point.x += kick.x * mul;
            board.dropping_point.y += kick.y * mul;
        };

        kicks.into_iter()
            .find(|&kick| {
                let clone = &mut self.clone();
                manipulation(clone, kick);

                clone.establishes_field()
            })
            .map(|kick| manipulation(self, kick))
            .is_some()
    }

    pub fn drop_softly(&mut self) -> DroppingMinoStatus {
        let status = self.dropping_mino_status();
        match status {
            DroppingMinoStatus::InAir => {
                self.dropping_point.y += 1;
            }
            DroppingMinoStatus::OnGround => ()
        }

        status
    }

    pub fn drop_hardly(&mut self) -> Option<usize> {
        let mut n = 0;
        loop {
            match self.drop_softly() {
                DroppingMinoStatus::InAir => { n += 1; }
                DroppingMinoStatus::OnGround => { return Some(n); }
            }
        }
    }

    pub fn determine_dropping_mino(&mut self) {
        for p in self.calc_dropping_mino_points() {
            self.confirmed_field[p.y as usize][p.x as usize] = self.dropping.block().into();
        }
    }

    pub fn calc_dropping_mino_prediction(&self) -> Vec<Point> {
        let mut clone = self.clone();

        loop {
            clone.dropping_point.y += 1;

            if !clone.establishes_field() {
                clone.dropping_point.y -= 1;
                break;
            }
        }

        clone.calc_dropping_mino_points()
    }

    fn establishes_field(&self) -> bool {
        self.calc_dropping_mino_points().iter()
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
}

#[derive(Debug, Eq, PartialEq)]
pub enum DroppingMinoStatus {
    InAir,
    OnGround,
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
    fn drop_t_softly() {
        let mut board = Board::new(Tetrimino::T);

        assert_eq!(board.drop_softly(), DroppingMinoStatus::InAir);

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
    fn drop_t_hardly() {
        let mut board = Board::new(Tetrimino::T);

        assert_eq!(board.drop_hardly(), Some(19));

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
    fn drop_vertical_i_hardly() {
        let mut board = Board::new(Tetrimino::I);

        assert!(board.try_move_x(1));
        assert!(board.try_spin(SpinDirection::Right));
        assert_eq!(board.drop_hardly(), Some(17));

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