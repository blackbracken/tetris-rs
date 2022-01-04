use num_traits::{FromPrimitive, ToPrimitive};

#[derive(FromPrimitive, ToPrimitive)]
pub enum SelectedItem {
    PlayFortyLine,
    Exit,
}

impl SelectedItem {
    pub fn next(&self) -> Option<SelectedItem> {
        FromPrimitive::from_usize(self.index() + 1)
    }

    pub fn prev(&self) -> Option<SelectedItem> {
        FromPrimitive::from_usize(self.index() - 1)
    }

    fn index(&self) -> usize {
        ToPrimitive::to_usize(self).unwrap()
    }
}
