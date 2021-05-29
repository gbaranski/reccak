use crate::Input;

#[derive(Clone)]
pub struct PermutationIterator {
    universe: &'static [u8],
    size: usize,
    prev: Option<Input>,
}

pub fn permutations(universe: &'static [u8], size: usize) -> PermutationIterator {
    PermutationIterator {
        size,
        universe,
        prev: None,
    }
}

impl Iterator for PermutationIterator {
    type Item = Input;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.universe.len();

        match self.prev {
            None => {
                let zeroes: Input = std::iter::repeat(0).take(self.size).collect();
                let result = zeroes
                    .iter()
                    .map(|&i| self.universe[i as usize].clone())
                    .collect::<Input>();
                self.prev = Some(zeroes);
                Some(result)
            }
            Some(ref mut indexes) => match indexes.iter().position(|&i| i + 1 < n as u8) {
                None => None,
                Some(position) => {
                    for index in indexes.iter_mut().take(position) {
                        *index = 0;
                    }
                    indexes[position] += 1;
                    let universe = self.universe;
                    let result = indexes
                        .iter()
                        .map(|&i| universe[i as usize].clone())
                        .collect();
                    Some(result)
                }
            },
        }
    }
}
