use std::collections::VecDeque;
use std::time::Duration;

use rand::prelude::SliceRandom;

use crate::tetris::board::{Board, MinoEntity, RemovedLines, Spin};
use crate::tetris::tetrimino::Tetrimino;

const NATURAL_DROP_INTERVAL: Duration = Duration::from_secs(1);
const COMBO_INITIAL: usize = 1;

pub type PutOrJustDropped = Option<RemovedLines>;

#[derive(Debug)]
pub struct PutResult {
    pub removed_lines: RemovedLines,
    pub reward: Option<ScoringReward>,
}

impl PutResult {
    fn new(removed_lines: RemovedLines, reward: Option<ScoringReward>) -> PutResult {
        PutResult {
            removed_lines,
            reward,
        }
    }
}

pub enum DroppedOrNothing {
    Dropped(PutOrJustDropped),
    Nothing,
}

impl DroppedOrNothing {
    pub fn dropped(result: PutOrJustDropped) -> DroppedOrNothing {
        DroppedOrNothing::Dropped(result)
    }
}

pub struct Game {
    pub board: Board,
    pub bag: MinoBag,
    pub hold_mino: Option<Tetrimino>,
    pub did_already_hold: bool,

    elapsed: Duration,
    last_dropped: Duration,

    pub score: usize,
    ready_back_to_back: bool,
    rotated_just_before: bool,
    combo: usize,
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
            elapsed: Duration::ZERO,
            last_dropped: Duration::ZERO,
            score: 0,
            ready_back_to_back: false,
            rotated_just_before: false,
            combo: COMBO_INITIAL,
        }
    }

    pub fn elapse(&mut self, delta: Duration) -> DroppedOrNothing {
        self.elapsed += delta;

        if self.last_dropped + NATURAL_DROP_INTERVAL < self.elapsed {
            self.last_dropped = self.elapsed;
            let result = self.drop_one();

            return DroppedOrNothing::dropped(result);
        }

        DroppedOrNothing::Nothing
    }

    pub fn move_left(&mut self) -> bool {
        self.last_dropped = self.elapsed;
        self.rotated_just_before = false;

        self.board.try_move_x(-1)
    }

    pub fn move_right(&mut self) -> bool {
        self.last_dropped = self.elapsed;
        self.rotated_just_before = false;

        self.board.try_move_x(1)
    }

    pub fn spin_left(&mut self) -> bool {
        self.spin(SpinDirection::Left)
    }

    pub fn spin_right(&mut self) -> bool {
        self.spin(SpinDirection::Right)
    }

    fn spin(&mut self, direction: SpinDirection) -> bool {
        self.last_dropped = self.elapsed;
        self.rotated_just_before = true;

        let spin = self.board.try_spin(direction);
        match spin {
            Some(Spin::TSpin) => {
                self.ready_back_to_back = true;
            }
            Some(_) => {
                self.ready_back_to_back = false;
            }
            None => (),
        }

        spin.is_some()
    }

    pub fn drop_one(&mut self) -> PutOrJustDropped {
        self.last_dropped = self.elapsed;

        if self.board.drop_one() {
            self.rotated_just_before = false;
            return None;
        }

        let put_result = self.calc_put_result_if_did();
        if let Some(reward) = put_result.reward {
            self.score += reward.score();
        }

        Some(put_result.removed_lines)
    }

    pub fn soft_drop(&mut self) -> PutOrJustDropped {
        self.score += 1;
        self.drop_one()
    }

    pub fn hard_drop(&mut self) -> RemovedLines {
        self.score += 2 * self.board.hard_drop();

        let put_result = self.calc_put_result_if_did();
        if let Some(reward) = put_result.reward {
            self.score += reward.score();
        }

        put_result.removed_lines
    }

    fn calc_put_result_if_did(&self) -> PutResult {
        let lines = self.board.filled_lines();
        if lines.is_empty() {
            return PutResult::new(lines, None);
        }

        let did_t_spin = self.board.dropping == Tetrimino::T
            && self.rotated_just_before
            && self.board.satisfies_cond_for_t_spin();
        let action = if did_t_spin {
            match lines.len() {
                1 => ScoringAction::TSpinSingle,
                2 => ScoringAction::TSpinDouble,
                3 => ScoringAction::TSpinTriple,
                _ => unreachable!(),
            }
        } else {
            match lines.len() {
                1 => ScoringAction::Single,
                2 => ScoringAction::Double,
                3 => ScoringAction::Triple,
                4 => ScoringAction::Tetris,
                _ => unreachable!(),
            }
        };

        let reward = ScoringReward::new(action, self.ready_back_to_back, self.combo);

        let r = PutResult::new(lines, Some(reward));
        println!("result: {:?}", r);
        r
    }

    pub fn try_swap_hold(&mut self) {
        if !self.did_already_hold {
            self.last_dropped = self.elapsed;
            self.did_already_hold = true;

            match self.hold_mino {
                Some(spawned) => {
                    let held = self.board.dropping;

                    self.hold_mino = Some(held);
                    self.board.spawn(spawned);
                }
                None => {
                    let spawned = self.bag.queue.pop_front().unwrap();
                    let held = self.board.dropping;

                    self.hold_mino = Some(held);
                    self.board.spawn(spawned);
                }
            }
        }
    }

    pub fn remove_lines(&mut self) {
        let put_result = self.calc_put_result_if_did();
        self.board.remove_lines();
    }

    pub fn put_and_spawn(&mut self) -> bool {
        self.board.determine_dropping_mino();

        self.did_already_hold = false;
        let mino = self.bag.pop();

        self.last_dropped = self.elapsed;

        if 0 < self.board.filled_lines().len() {
            self.combo += 1;
        } else {
            self.combo = COMBO_INITIAL;
        }

        self.board.spawn(mino)
    }
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
            .map(|idx| self.queue.get(idx).unwrap().to_owned())
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

pub enum DropResult {
    SoftDropped,
    Put,
    Failed,
}

pub enum SpinDirection {
    Left,
    Right,
}

#[derive(Debug)]
pub struct ScoringReward {
    action: ScoringAction,
    with_back_to_back: bool,
    combo: usize,
}

impl ScoringReward {
    fn new(action: ScoringAction, with_back_to_back: bool, combo: usize) -> ScoringReward {
        ScoringReward {
            action,
            with_back_to_back,
            combo,
        }
    }

    pub fn score(&self) -> usize {
        use ScoringAction::*;
        let action_score = match self.action {
            Single => 100,
            Double => 300,
            Triple => 500,
            Tetris => 800,
            TSpinSingle => 800,
            TSpinDouble => 1200,
            TSpinTriple => 1600,
            PerfectClear => 5000,
        };

        let back_to_back_bonus = if self.action.is_subjected_to_back_to_back() {
            action_score * 2 / 3
        } else {
            0
        };

        let combo_score = 50 * (self.combo.saturating_sub(1));

        action_score + back_to_back_bonus + combo_score
    }
}

#[derive(Debug)]
enum ScoringAction {
    Single,
    Double,
    Triple,
    Tetris,
    TSpinSingle,
    TSpinDouble,
    TSpinTriple,
    PerfectClear,
}

impl ScoringAction {
    fn is_subjected_to_back_to_back(&self) -> bool {
        use ScoringAction::*;

        match self {
            Tetris | TSpinSingle | TSpinDouble | TSpinTriple => true,
            _ => false
        }
    }
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
mod tests {
    use crate::macros::rect_vec;

    use super::*;

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
    fn gen_all_minos() {
        let mut minos = MinoBag::gen_shuffled_all_minos();

        assert_eq!(minos.len(), Tetrimino::all().len());

        minos.sort();
        minos.dedup();
        assert_eq!(minos.len(), Tetrimino::all().len());
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
}