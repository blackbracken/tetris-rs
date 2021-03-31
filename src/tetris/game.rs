use std::collections::VecDeque;
use std::convert::TryFrom;

use rand::prelude::SliceRandom;

use crate::tetris::tetrimino::{MinoRotation, Tetrimino};

pub const FIELD_UNIT_WIDTH: usize = 10;
pub const FIELD_UNIT_HEIGHT: usize = 22;
pub const FIELD_VISIBLE_UNIT_HEIGHT: usize = 20;

const SPAWN_POINT: Point = Point {
    x: 4,
    y: ((FIELD_UNIT_HEIGHT - FIELD_VISIBLE_UNIT_HEIGHT + 1) as isize),
};

pub type Field = [[MinoEntity; FIELD_UNIT_WIDTH]; FIELD_UNIT_HEIGHT];

pub struct Game {
    pub board: Board,
    pub bag: MinoBag,
    pub hold_mino: Option<Tetrimino>,
    pub did_already_hold: bool,
}

impl Game {
    pub fn new() -> Game {
        let mut bag = MinoBag::new();
        let dropping = bag.pop();

        Game {
            board: Board::new(dropping),
            bag,
            hold_mino: None,
            did_already_hold: false,
        }
    }

    pub fn move_left(&mut self) -> bool {
        self.board.try_move_x(-1)
    }

    pub fn move_right(&mut self) -> bool {
        self.board.try_move_x(1)
    }

    pub fn spin_left(&mut self) -> bool {
        self.board.try_spin(SpinDirection::Left)
    }

    pub fn spin_right(&mut self) -> bool {
        self.board.try_spin(SpinDirection::Right)
    }

    pub fn drop_softly(&mut self) -> DropResult {
        match self.board.drop_softly() {
            DroppingMinoStatus::InAir => {
                DropResult::SoftDropped
            }
            DroppingMinoStatus::OnGround => {
                self.put_and_spawn(); // TODO: handle error
                DropResult::Put
            }
        }
    }

    pub fn drop_hardly(&mut self) -> Option<usize> {
        self.board.drop_hardly()
            .filter(|_| self.put_and_spawn() == SpawnResult::Success)
    }

    pub fn try_swap_hold(&mut self) {
        if !self.did_already_hold {
            self.did_already_hold = true;

            match self.hold_mino {
                Some(spawned) => {
                    let held = self.board.dropping;

                    self.hold_mino = Some(held);
                    self.spawn_mino(spawned);
                }
                None => {
                    let spawned = self.bag.queue.pop_front().unwrap();
                    let held = self.board.dropping;

                    self.hold_mino = Some(held);
                    self.spawn_mino(spawned);
                }
            }
        }
    }

    fn spawn_mino(&mut self, mino: Tetrimino) -> SpawnResult {
        self.board.dropping = mino;
        self.board.dropping_point = SPAWN_POINT;
        self.board.dropping_rotation = MinoRotation::default();

        if !self.board.establishes_field() {
            self.board.dropping_point.y -= 1;

            if !self.board.establishes_field() {
                return SpawnResult::Fail;
            }
        }

        SpawnResult::Success
    }

    fn spawn_mino_from_bag(&mut self) -> SpawnResult {
        self.did_already_hold = false;

        let mino = self.bag.pop();
        self.spawn_mino(mino)
    }

    fn put_and_spawn(&mut self) -> SpawnResult {
        if !self.board.establishes_field() {
            return SpawnResult::Fail;
        }

        self.board.determine_dropping_mino();
        self.spawn_mino_from_bag()
    }
}

pub enum DropResult {
    SoftDropped,
    Put,
    Failed,
}

#[derive(Copy, Clone)]
pub struct Board {
    confirmed_field: Field,
    dropping: Tetrimino,
    dropping_point: Point,
    dropping_rotation: MinoRotation,
}

impl Board {
    fn new(dropping: Tetrimino) -> Board {
        Board {
            confirmed_field: [[MinoEntity::AIR; FIELD_UNIT_WIDTH]; FIELD_UNIT_HEIGHT],
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

    pub fn dropping_mino_status(&self) -> DroppingMinoStatus {
        let mut clone = self.clone();
        clone.dropping_point.y += 1;

        if clone.establishes_field() {
            DroppingMinoStatus::InAir
        } else {
            DroppingMinoStatus::OnGround
        }
    }

    fn try_move_x(&mut self, addition: isize) -> bool {
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

    fn try_spin(&mut self, direction: SpinDirection) -> bool {
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

    fn drop_softly(&mut self) -> DroppingMinoStatus {
        let status = self.dropping_mino_status();
        match status {
            DroppingMinoStatus::InAir => {
                self.dropping_point.y += 1;
            }
            DroppingMinoStatus::OnGround => ()
        }

        status
    }

    fn drop_hardly(&mut self) -> Option<usize> {
        let mut n = 0;
        loop {
            match self.drop_softly() {
                DroppingMinoStatus::InAir => { n += 1; }
                DroppingMinoStatus::OnGround => { return Some(n); }
            }
        }
    }

    fn determine_dropping_mino(&mut self) {
        for p in self.calc_dropping_mino_points() {
            self.confirmed_field[p.y as usize][p.x as usize] = self.dropping.block().into();
        }
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
pub enum SpawnResult {
    Success,
    Fail,
}

#[derive(Debug, Eq, PartialEq)]
pub enum DroppingMinoStatus {
    InAir,
    OnGround,
}

pub struct MinoBag {
    queue: VecDeque<Tetrimino>
}

impl MinoBag {
    fn new() -> MinoBag {
        let mut queue = MinoBag::gen_shuffled_all_minos();
        let mut added = MinoBag::gen_shuffled_all_minos();
        queue.append(&mut added);

        MinoBag {
            queue: queue.into(),
        }
    }

    fn pop(&mut self) -> Tetrimino {
        let p = self.queue.pop_front().unwrap();

        if self.queue.len() < Tetrimino::all().len() {
            let added = MinoBag::gen_shuffled_all_minos();
            self.queue.append(&mut added.into());
        }

        p
    }

    pub fn peek(&self, amount: usize) -> Vec<Tetrimino> {
        if amount > Tetrimino::all().len() {
            panic!("the amount of minos must be equal to or lower than the amount of tetrimino types");
        }

        (0..amount)
            .map(|idx| self.queue.get(idx).unwrap().clone())
            .collect::<Vec<_>>()
    }


    fn gen_shuffled_all_minos() -> Vec<Tetrimino> {
        let mut rng = rand::thread_rng();

        let mut s = Tetrimino::all();
        s.shuffle(&mut rng);

        s
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum MinoBlock {
    AQUA,
    YELLOW,
    PURPLE,
    BLUE,
    ORANGE,
    GREEN,
    RED,
}

impl Into<MinoEntity> for MinoBlock {
    fn into(self) -> MinoEntity {
        use MinoEntity::*;

        match self {
            MinoBlock::AQUA => AQUA,
            MinoBlock::YELLOW => YELLOW,
            MinoBlock::PURPLE => PURPLE,
            MinoBlock::BLUE => BLUE,
            MinoBlock::ORANGE => ORANGE,
            MinoBlock::GREEN => GREEN,
            MinoBlock::RED => RED,
        }
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
}

enum SpinDirection {
    Left,
    Right,
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Into<Point> for (isize, isize) {
    fn into(self) -> Point {
        Point { x: self.0, y: self.1 }
    }
}

impl Into<Point> for (f32, f32) {
    fn into(self) -> Point {
        Point { x: self.0 as isize, y: self.1 as isize }
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

    impl LikeExistenceField for [[i32; FIELD_UNIT_WIDTH]; FIELD_VISIBLE_UNIT_HEIGHT] {
        fn to_existences(&self) -> ExistenceField {
            let mut f = ExistenceField::default();
            self.iter().enumerate().for_each(|(y, line)| {
                line.iter().enumerate().for_each(|(x, &n)| {
                    f[y + 2][x] = n > 0;
                })
            });

            f
        }
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
    fn field_is_only_with_dropping_on_init() {
        let board = Board::new(Tetrimino::T);

        let expected = [
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
    fn move_right_once() {
        let mut board = Board::new(Tetrimino::T);
        assert!(board.try_move_right());

        let expected = [
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
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn move_right_to_limit() {
        let mut board = Board::new(Tetrimino::T);
        for _ in 0..4 {
            assert!(board.try_move_right());
        }
        assert!(!board.try_move_right());

        let expected = [
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
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn move_left_once() {
        let mut board = Board::new(Tetrimino::T);
        assert!(board.try_move_left());

        let expected = [
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
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn move_left_to_limit() {
        let mut board = Board::new(Tetrimino::T);
        for _ in 0..3 {
            assert!(board.try_move_left());
        }
        assert!(!board.try_move_left());

        let expected = [
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
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn spin_right() {
        let mut board = Board::new(Tetrimino::S);

        let expected = [
            [0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 0, 0, 0, 0, 0],
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

        assert!(board.try_spin_right());

        let expected = [
            [0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
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
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn drop_t_hardly() {
        let mut board = Board::new(Tetrimino::T);

        assert_eq!(board.drop_hardly(), Some(18));

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
            [0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn drop_vertical_i_hardly() {
        let mut board = Board::new(Tetrimino::I);

        assert!(board.try_spin_right());
        assert_eq!(board.drop_hardly(), Some(16));

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
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn gen_all_minos() {
        let mut minos = MinoBag::gen_shuffled_all_minos();

        assert!(minos.len() == Tetrimino::all().len());

        minos.sort();
        minos.dedup();
        assert!(minos.len() == Tetrimino::all().len());
    }

    #[test]
    fn peek_bag_minos() {
        let bag = MinoBag::new();
        let l = Tetrimino::all().len();

        assert_eq!(bag.peek(l).len(), l);
    }

    #[test]
    #[should_panic]
    fn peek_bag_minos_exceeded_limit() {
        let bag = MinoBag::new();
        let l = Tetrimino::all().len();

        bag.peek(l + 1);
    }

    #[test]
    fn spawn_mino_j() {
        let mut game = Game::new();
        game.bag.queue = vec!(Tetrimino::J).into();
        assert_eq!(game.spawn_mino_from_bag(), SpawnResult::Success);

        let expected = [
            [0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
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

        assert!(game.board.field().similar(expected));
    }

    #[test]
    fn spawn_mino_i() {
        let mut game = Game::new();
        game.bag.queue = vec!(Tetrimino::I).into();
        assert_eq!(game.spawn_mino_from_bag(), SpawnResult::Success);

        let expected = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1, 0, 0, 0],
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

        assert!(game.board.field().similar(expected));
    }

    #[test]
    fn spawn_mino_o() {
        let mut game = Game::new();
        game.bag.queue = vec!(Tetrimino::O).into();
        assert_eq!(game.spawn_mino_from_bag(), SpawnResult::Success);

        let expected = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
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

        assert!(game.board.field().similar(expected));
    }
}