use enum_iterator::IntoEnumIterator;
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(FromPrimitive, ToPrimitive, IntoEnumIterator, Debug, PartialEq, Eq, Hash)]
pub enum SelectedItem {
    PlayFortyLine,
    Exit,
}

impl SelectedItem {
    pub fn all() -> Vec<SelectedItem> {
        SelectedItem::into_enum_iter().collect()
    }

    pub fn name(&self) -> &'static str {
        match *self {
            SelectedItem::PlayFortyLine => "Play 40Line",
            SelectedItem::Exit => "Exit",
        }
    }

    pub fn next(&self) -> Option<SelectedItem> {
        FromPrimitive::from_isize(self.index() + 1)
    }

    pub fn prev(&self) -> Option<SelectedItem> {
        FromPrimitive::from_isize(self.index() - 1)
    }

    fn index(&self) -> isize {
        ToPrimitive::to_isize(self).unwrap()
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case(SelectedItem::PlayFortyLine, Some(SelectedItem::Exit))]
    #[test_case(SelectedItem::Exit, None)]
    fn test_next(src: SelectedItem, ans: Option<SelectedItem>) {
        assert_eq!(src.next(), ans)
    }

    #[test_case(SelectedItem::Exit, Some(SelectedItem::PlayFortyLine))]
    #[test_case(SelectedItem::PlayFortyLine, None)]
    fn test_prev(src: SelectedItem, ans: Option<SelectedItem>) {
        assert_eq!(src.prev(), ans)
    }
}
