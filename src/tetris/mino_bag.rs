use std::collections::VecDeque;

use rand::prelude::SliceRandom;

use crate::tetris::model::tetrimino::Tetrimino;

/// # ミノを保有するバッグ
///
/// ゲームに出現させるテトリミノを, 七種一巡の法則に従って供給する.
pub struct MinoBag {
    queue: VecDeque<Tetrimino>
}

impl MinoBag {
    pub fn new() -> MinoBag {
        let mut queue = MinoBag::gen_shuffled_all_minos();
        let mut added = MinoBag::gen_shuffled_all_minos();
        queue.append(&mut added);

        MinoBag {
            queue: queue.into(),
        }
    }

    pub fn pop(&mut self) -> Tetrimino {
        let p = self.queue.pop_front().unwrap();

        if self.queue.len() < Tetrimino::all().len() {
            let added = MinoBag::gen_shuffled_all_minos();
            self.queue.append(&mut added.into());
        }

        p
    }

    /// ピークする個数はテトリミノ一巡分を超過してはならない.
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
