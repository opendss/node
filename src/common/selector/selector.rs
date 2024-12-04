use crate::common::lifecycle::stateful::Stateful;
use rand::{Rng, RngCore};
use std::collections::HashSet;

pub(crate) struct RoundRobinIndexSelector {
    index_size: u32,
    current_index: u32,
}

impl RoundRobinIndexSelector {
    pub fn new(init_size: u32) -> Self {
        Self {
            index_size: init_size,
            current_index: 0,
        }
    }

    pub(crate) fn next(&mut self) -> u32 {
        if self.current_index == self.index_size {
            self.current_index = 0;
        }
        let index = self.current_index;
        self.current_index += 1;
        index
    }
}

pub(crate) struct RandomIndexSelector {
    rng: rand::rngs::ThreadRng,
    index_size: u32,
}

impl RandomIndexSelector {
    pub fn new(init_size: u32) -> Self {
        Self {
            index_size: init_size,
            rng: rand::thread_rng(),
        }
    }

    fn next(&mut self, num: u32) -> HashSet<u32> {
        let mut vec: HashSet<u32> = HashSet::new();
        while vec.len() < num as usize {
            let i = self.rng.gen_range(0..self.index_size);
            vec.insert(i);
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use crate::common::selector::selector::RoundRobinIndexSelector;
    #[test]
    fn test_round_robin_index_selector() {
        let init_size = 10;
        let mut selector = RoundRobinIndexSelector::new(init_size);
        for i in 0..init_size {
            let index = selector.next();
            assert_eq!(i, index);
        }
        let index = selector.next();
        assert_eq!(0, index);
    }

    #[test]
    fn test_random_index_selector() {
        
    }
}
