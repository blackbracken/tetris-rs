use ScoringAction::*;

#[derive(Debug)]
pub struct ScoringReward {
    pub action: ScoringAction,
    pub with_back_to_back: bool,
    pub combo: usize,
}

impl ScoringReward {
    pub fn new(action: ScoringAction, with_back_to_back: bool, combo: usize) -> ScoringReward {
        ScoringReward {
            action,
            with_back_to_back,
            combo,
        }
    }

    pub fn score(&self) -> usize {
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
pub enum ScoringAction {
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
    pub fn is_subjected_to_back_to_back(&self) -> bool {
        match self {
            Tetris | TSpinSingle | TSpinDouble | TSpinTriple => true,
            _ => false
        }
    }
}