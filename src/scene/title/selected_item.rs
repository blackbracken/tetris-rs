use num_traits::{FromPrimitive, ToPrimitive};

#[derive(FromPrimitive, ToPrimitive, Debug, PartialEq, Eq, Hash)]
pub enum SelectedItem {
    PlayFortyLine,
    Exit,
}

impl SelectedItem {
    pub fn all() -> Vec<SelectedItem> {
        let mut items = vec![SelectedItem::PlayFortyLine];

        loop {
            match items.last().unwrap().next() {
                None => return items,
                Some(next) => items.push(next),
            };
        }
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
    use std::mem::variant_count;

    use test_case::test_case;

    use super::*;

    #[test]
    fn test_variant_count() {
        assert_eq!(SelectedItem::all().len(), variant_count::<SelectedItem>())
    }

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
