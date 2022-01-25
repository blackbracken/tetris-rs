use std::{
    cmp::Ordering,
    convert::{TryFrom, TryInto},
    time::Duration,
};
use std::ops::Deref;

enum Repeat {
    Count(Duration, u32),
    Infinite(Duration),
}

impl Repeat {
    pub fn latency(&self) -> &Duration {
        match self {
            Repeat::Count(d, _) => d,
            Repeat::Infinite(d) => d,
        }
    }
}

impl PartialEq<u32> for Repeat {
    fn eq(&self, other: &u32) -> bool {
        matches!(self, Repeat::Count(_, c) if c == other)
    }
}

impl PartialOrd<u32> for Repeat {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        match self {
            Repeat::Infinite(_) => Some(Ordering::Greater),
            Repeat::Count(_, c) if c < other => Some(Ordering::Less),
            Repeat::Count(_, c) if c > other => Some(Ordering::Greater),
            Repeat::Count(_, _) => Some(Ordering::Equal),
        }
    }
}

/// 定期的な条件分岐を行うための, ステートフルなタイマーとして表現される.
///
/// 前回のbeepから, `bootwait`,`repeat` 以上の時間が経過したことを beep/鳴る と表現し,
/// その期間内のbeepが行われないようにすることを consume/消費する と表現する.
///
/// このタイマーが多重に鳴っている状態でそれを消費するとき, 全てのbeepは消費される.
pub struct Timer {
    /// 生成されてから最初に鳴るまでの時間.
    bootwait: Duration,

    /// `bootwait` 以降の繰り返しで鳴る間隔.
    repeat: Repeat,

    /// タイマーが開始してからの経過時間.
    elapsed: Duration,

    /// 今までにタイマーが鳴った回数.
    beeped_count: u32,
}

impl Timer {
    pub fn new(bootwait: Duration, latency: Duration, count: u32) -> Timer {
        Timer {
            bootwait,
            repeat: Repeat::Count(latency, count),
            elapsed: Duration::ZERO,
            beeped_count: 0,
        }
    }

    pub fn infinite(bootwait: Duration, latency: Duration) -> Timer {
        Timer {
            bootwait,
            repeat: Repeat::Infinite(latency),
            elapsed: Duration::ZERO,
            beeped_count: 0,
        }
    }

    pub fn repeat(latency: Duration, count: u32) -> Timer {
        Timer::new(latency, latency, count)
    }

    pub fn single(bootwait: Duration) -> Timer {
        Timer::new(bootwait, Duration::ZERO, 1)
    }

    pub fn elapse(&mut self, delta: Duration) {
        self.elapsed = self.elapsed.saturating_add(delta)
    }

    pub fn consume_if_beep(&mut self) -> bool {
        let should_consume = self.is_beeping();
        if should_consume {
            self.consume();
        }

        should_consume
    }

    pub fn is_beeping(&self) -> bool {
        if self.elapsed < self.bootwait {
            false
        } else {
            let last_beeped = if self.beeped_count == 0 {
                Duration::ZERO
            } else {
                self.bootwait + *self.repeat.latency() * (self.beeped_count - 1)
            };

            self.repeat > self.beeped_count && self.elapsed - last_beeped >= *self.repeat.latency()
        }
    }

    fn consume(&mut self) {
        self.beeped_count = match () {
            _ if self.elapsed < self.bootwait => 0,
            _ if self.bootwait + *self.repeat.latency() > self.elapsed => 1,
            _ if *self.repeat.latency() == Duration::ZERO => self.beeped_count + 1,
            _ => {
                let r = (self.elapsed - self.bootwait).as_millis() / self.repeat.latency().as_millis();
                let r = u32::try_from(r).unwrap();

                r + 1
            }
        };
    }
}

#[cfg(test)]
mod repeat_tests {
    use test_case::test_case;

    use super::*;

    #[test_case(Repeat::Count(Duration::ZERO, 1), 5, Ordering::Greater)]
    #[test_case(Repeat::Count(Duration::ZERO, 5), 1, Ordering::Less)]
    #[test_case(Repeat::Count(Duration::ZERO, 1), 1, Ordering::Equal)]
    #[test_case(Repeat::Infinite(Duration::ZERO), 1, Ordering::Greater)]
    #[test_case(Repeat::Infinite(Duration::ZERO), u32::MAX, Ordering::Greater)]
    #[test_case(Repeat::Infinite(Duration::ZERO), 0, Ordering::Greater)]
    fn compare_u32(left: Repeat, right: u32, ordering: Ordering) {
        assert!(matches!(left.partial_cmp(&right), Some(ordering)));
    }
}

#[cfg(test)]
mod timer_tests {
    use super::*;

    #[test]
    fn test_timer_never_beep() {
        let sec = Duration::from_secs(1);
        let mut timer = Timer::repeat(sec, 0);

        assert!(!timer.is_beeping());

        timer.elapse(sec);
        assert!(!timer.is_beeping());
    }

    #[test]
    fn test_timer_beep_once() {
        let sec = Duration::from_secs(1);
        let half = sec / 2;
        let mut timer = Timer::single(sec);

        assert!(!timer.is_beeping());

        timer.elapse(half);
        assert!(!timer.is_beeping());

        timer.elapse(half);
        assert!(timer.is_beeping());

        let beeped = timer.consume_if_beep();
        assert!(beeped);
        assert!(!timer.is_beeping());

        timer.elapse(half);
        assert!(!timer.is_beeping());
    }

    #[test]
    fn test_timer_beep_infinitely() {
        let sec = Duration::from_secs(1);
        let half = sec / 2;
        let mut timer = Timer::infinite(sec, sec);

        assert!(!timer.is_beeping());

        timer.elapse(sec);
        assert!(timer.is_beeping());
        let beeped = timer.consume_if_beep();
        assert!(beeped);
        assert!(!timer.is_beeping());

        for _ in 1..=100 {
            timer.elapse(half);
            assert!(!timer.is_beeping());

            timer.elapse(half);
            let beeped = timer.consume_if_beep();
            assert!(beeped);

            assert!(!timer.is_beeping());
        }
    }

    #[test]
    fn test_if_consume_many_times_at_same_time() {
        let sec = Duration::from_secs(1);
        let mut timer = Timer::single(sec);

        assert!(!timer.is_beeping());

        timer.elapse(sec);
        assert!(timer.is_beeping());

        let beeped = timer.consume_if_beep();
        assert!(beeped);

        for _ in 1..=10 {
            let beeped = timer.consume_if_beep();
            assert!(!beeped);
            assert!(!timer.is_beeping());
        }
    }
}
