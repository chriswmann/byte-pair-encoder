#![deny(missing_docs)]
//! Library to provide byte pair encoding of strings.

use std::collections::HashMap;
use std::fmt::Debug;

/// Define a hashable type to use as a key in a HashMap when counting
#[derive(Hash, PartialEq, Eq)]
pub struct TupleStruct(char, char);

impl Debug for TupleStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {:?}", self.0, self.1)
    }
}

/// Count the number of occurrences of each integer in an iterator
pub struct Counter {
    counts: HashMap<TupleStruct, usize>,
}

/// Create a new counter using Counter()
impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

impl Counter {
    /// Create a new counter
    pub fn new() -> Self {
        Counter {
            counts: std::collections::HashMap::new(),
        }
    }

    /// Count the number of occurrences of each integer in an iterator
    pub fn count(&mut self, string: &str)
    {
        let s_iter = string.chars().zip(string.chars().skip(1));
        for (c1, c2) in s_iter {
            let key = TupleStruct(c1, c2);
            *self.counts.entry(key).or_insert(0) += 1;
        }
    }

    /// Get the counts as a HashMap
    pub fn counts(&self) -> &HashMap<TupleStruct, usize> {
        &self.counts
    }

    /// Get the most common pair of characters
    pub fn most_common(&self) {
        let max = self.counts.iter().max_by(|_, b| b.1.cmp(&b.1)).unwrap().1;
        self.counts.iter()
        .filter_map(|(key, &val)| if val == *max { Some(key) } else { None })
        .collect()

    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_counter_counts_correct_number_of_character_pairs() {
        let mut counter = Counter::new();
        let my_string = String::from("Cat at 😃!");
        counter.count(&my_string);
        // println!("{:?}", counter.counts());
        assert_eq!(counter.counts().get(&TupleStruct(' ', 'a')), Some(&1));
        assert_eq!(counter.counts().get(&TupleStruct(' ', '😃')), Some(&1));
        assert_eq!(counter.counts().get(&TupleStruct('C', 'a')), Some(&1));
        assert_eq!(counter.counts().get(&TupleStruct('a', 't')), Some(&2));
        assert_eq!(counter.counts().get(&TupleStruct('t', ' ')), Some(&2));
        assert_eq!(counter.counts().get(&TupleStruct('😃', '!')), Some(&1));
    }

    #[test]
    fn test_most_common_returns_expected_result() {
        let mut counter = Counter::new();
        let my_string = String::from("Cat at 😃!");
        counter.count(&my_string);
        // println!("{:?}", counter.counts());
        println!("{:?}", counter.most_common());
    }

}
