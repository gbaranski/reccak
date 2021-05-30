use crate::Input;

#[derive(Clone)]
pub struct PermutationIterator {
    universe: &'static [u8],
    size: usize,
    prev: Input,
}

pub fn permutations(universe: &'static [u8], size: usize) -> PermutationIterator {
    let prev = std::iter::repeat(0).take(size).collect::<Input>();

    PermutationIterator {
        universe,
        size,
        prev,
    }
}

impl Iterator for PermutationIterator {
    type Item = Input;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.universe.len();

        match self.prev.iter().position(|i| *i + 1 < n as u8) {
            None => None,
            Some(position) => {
                for index in self.prev.iter_mut().take(position) {
                    *index = 0;
                }
                self.prev[position] += 1;
                let universe = self.universe;
                let result = self.prev.iter().map(|&i| universe[i as usize]).collect();
                Some(result)
            }
        }
    }
}
