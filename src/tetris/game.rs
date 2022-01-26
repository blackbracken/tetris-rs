use std::time::Duration;

use crate::tetris::{
    board::{Board, RemovedLines},
    mino_bag::MinoBag,
    model::{
        score::{ScoringAction, ScoringReward},
        spin::SpinDirection,
        tetrimino::Tetrimino,
    },
};

const NATURAL_DROP_INTERVAL: Duration = Duration::from_secs(1);
const COMBO_INITIAL: usize = 1;

pub type PutOrJustDropped = Option<RemovedLines>;

pub struct Game {
    pub board: Board,
    pub bag: MinoBag,
    pub hold_mino: Option<Tetrimino>,
    pub did_already_hold: bool,

    pub elapsed: Duration,
    last_dropped: Duration,

    pub score: usize,
    ready_back_to_back: bool,
    rotated_just_before: bool,
    combo: usize,
    pub removed_line_count: usize,
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
            removed_line_count: 0,
        }
    }

    pub fn elapse(&mut self, delta: Duration) -> DroppedOrNothing {
        self.elapsed += delta;

        if self.last_dropped + NATURAL_DROP_INTERVAL < self.elapsed {
            self.last_dropped = self.elapsed;
            let result = self.drop_one();

            return DroppedOrNothing::Dropped(result);
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

        self.board.try_spin(direction).is_some()
    }

    pub fn drop_one(&mut self) -> PutOrJustDropped {
        self.last_dropped = self.elapsed;

        if self.board.drop_one() {
            self.rotated_just_before = false;
            return None;
        }

        Some(self.prepare_putting().removed_lines)
    }

    pub fn soft_drop(&mut self) -> PutOrJustDropped {
        self.score += 1;
        self.drop_one()
    }

    pub fn hard_drop(&mut self) -> RemovedLines {
        self.score += 2 * self.board.hard_drop();

        self.prepare_putting().removed_lines
    }

    #[warn(unused_must_use)]
    fn prepare_putting(&mut self) -> PutResult {
        let put_result = self.calc_put_result_if_did();

        self.removed_line_count += put_result.removed_lines.len();
        if let Some(ref reward) = put_result.reward {
            self.score += reward.score();
            self.ready_back_to_back = reward.action.is_subjected_to_back_to_back()
        }

        put_result
    }

    fn calc_put_result_if_did(&self) -> PutResult {
        let lines = self.board.filled_lines();
        if lines.is_empty() {
            return PutResult::new(lines, None);
        }

        let did_perfect_clear = self
            .board
            .confirmed_field
            .iter()
            .all(|line| line.iter().all(|e| e.is_air()));
        let did_t_spin = self.board.dropping == Tetrimino::T
            && self.rotated_just_before
            && self.board.satisfies_cond_for_t_spin();

        let action = if did_perfect_clear {
            ScoringAction::PerfectClear
        } else if did_t_spin {
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
                    let spawned = self.bag.pop();
                    let held = self.board.dropping;

                    self.hold_mino = Some(held);
                    self.board.spawn(spawned);
                }
            }
        }
    }

    pub fn remove_lines(&mut self) {
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

#[derive(new)]
pub enum DroppedOrNothing {
    Dropped(PutOrJustDropped),
    Nothing,
}

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

#[derive(Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Into<Point> for (isize, isize) {
    fn into(self) -> Point {
        Point {
            x: self.0,
            y: self.1,
        }
    }
}

impl Into<Point> for (f32, f32) {
    fn into(self) -> Point {
        Point {
            x: self.0 as isize,
            y: self.1 as isize,
        }
    }
}
