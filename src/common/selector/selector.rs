use crate::common::lifecycle::stateful::Stateful;
use crate::node_group::NodeStats;

pub trait Selector<T> {
    fn next(&mut self) -> Option<&T>;
}

#[derive(Clone)]
pub struct RoundRobinSelector<T, F> {
    collection: Vec<T>,
    current_index: usize,
    skip_filter_fn: F,
}

pub type SelectorFilterType = Box<dyn Fn(&NodeStats) -> bool + Send + Sync>;

impl<T, F> RoundRobinSelector<T, F>
where
    F: Fn(&T) -> bool + Send + Sync,
{
    pub fn new(skip_filter_fn: F) -> Self {
        Self {
            collection: Vec::new(),
            current_index: 0,
            skip_filter_fn,
        }
    }

    pub fn add(&mut self, element: T) {
        self.collection.push(element);
    }
}

impl<T, F> Selector<T> for RoundRobinSelector<T, F>
where
    F: Fn(&T) -> bool + Send + Sync,
{
    fn next(&mut self) -> Option<&T> {
        let mut filtered_elements = 0;
        while self.current_index < self.collection.len()
            && filtered_elements != self.collection.len()
        {
            let el = self.collection.get(self.current_index);
            match el {
                None => {
                    self.current_index = 0;
                    continue;
                }
                Some(el) => {
                    // move index
                    if self.current_index == self.collection.len() - 1 {
                        self.current_index = 0;
                    } else {
                        self.current_index += 1;
                    }
                    if (self.skip_filter_fn)(el) {
                        filtered_elements += 1;
                        continue;
                    }
                    return Some(el);
                }
            };
        }
        None
    }
}

#[derive(Clone)]
pub(crate) struct RandomSelector<T, F> {
    collection: Vec<T>,
    skip_filter_fn: F,
}

impl<T, F> RandomSelector<T, F>
where
    F: Fn(&T) -> bool + Send + Sync,
{
    pub fn new(skip_filter_fn: F) -> Self {
        Self {
            collection: Vec::new(),
            skip_filter_fn,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::selector::selector::{RoundRobinSelector, Selector};
    #[test]
    fn test_round_robin_selector_with_always_filter() {
        let filter = |x: &String| true;
        let mut selector = RoundRobinSelector::new(filter);
        selector.add(String::from("1"));
        let option = selector.next();
        assert!(option.is_none());
    }

    #[test]
    fn test_round_robin_selector_with_one_filter() {
        let filter = |x: &String| x == "1";
        let mut selector = RoundRobinSelector::new(filter);
        selector.add(String::from("1"));
        selector.add(String::from("2"));
        let option = selector.next();
        assert!(option.is_some());
        assert_eq!(option.unwrap(), "2");
    }

    #[test]
    fn test_round_robin_selector_without_filter() {
        let no_filter = |x: &String| false;
        let mut selector = RoundRobinSelector::new(no_filter);
        selector.add(String::from("1"));
        selector.add(String::from("2"));

        let option = selector.next();
        assert!(option.is_some());
        assert_eq!(option.unwrap(), "1");

        let option = selector.next();
        assert!(option.is_some());
        assert_eq!(option.unwrap(), "2");

        let option = selector.next();
        assert!(option.is_some());
        assert_eq!(option.unwrap(), "1");
    }
}
