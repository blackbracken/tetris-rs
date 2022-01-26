use std::{collections::HashSet, hash::Hash, mem::take, time::Duration};

pub trait AnimationProperty {
    fn duration(&self) -> &Duration;
    fn elapse(self, delta: &Duration) -> Self;
    fn is_active(&self) -> bool;
}

#[derive(Default)]
pub struct AnimationProperties<T>
where
    T: AnimationProperty,
{
    props: Vec<T>,
}

impl<T> AnimationProperties<T>
where
    T: AnimationProperty,
{
    pub fn new() -> Self {
        AnimationProperties { props: Vec::new() }
    }

    pub fn elapse(&mut self, delta: &Duration) {
        let props = take(&mut self.props)
            .into_iter()
            .map(|prop| prop.elapse(delta))
            .filter(|prop| prop.is_active())
            .collect();

        self.props = props;
    }

    pub fn add(&mut self, prop: T) {
        self.props.push(prop);
    }

    pub fn props(&self) -> &Vec<T> {
        &self.props
    }
}
