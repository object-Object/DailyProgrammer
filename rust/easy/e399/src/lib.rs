use std::collections::BTreeMap;

pub struct WordList {
    pub words: Vec<String>,
    pub words_by_letter_sum: BTreeMap<u32, Vec<String>>,
}

impl WordList {
    pub fn new(words: Vec<String>) -> WordList {
        let mut words_by_letter_sum = BTreeMap::new();
        for word in &words {
            let vec = words_by_letter_sum
                .entry(Self::letter_sum(word))
                .or_insert(Vec::new());
            vec.push(word.clone());
        }
        for vec in words_by_letter_sum.values_mut() {
            vec.sort_by(|a, b| a.len().cmp(&b.len()));
        }

        WordList {
            words,
            words_by_letter_sum,
        }
    }

    pub fn letter_sum(input: &str) -> u32 {
        let mut output: u32 = 0;
        for c in input.chars() {
            output += (c as u32) - ('a' as u32) + 1;
        }
        output
    }

    pub fn with_letter_sum(&self, sum: u32) -> Option<Vec<String>> {
        match self.words_by_letter_sum.get(&sum) {
            Some(vec) => Some(vec.clone()),
            None => None,
        }
    }

    pub fn with_letter_sum_predicate<F>(&self, f: F) -> Option<Vec<String>>
    where
        F: Fn(u32) -> bool,
    {
        let mut output = Vec::new();
        for (&letter_sum, words) in &self.words_by_letter_sum {
            if f(letter_sum) {
                output.extend(words.iter().cloned());
            }
        }
        match output.len() {
            0 => None,
            _ => Some(output),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::WordList;

    #[test]
    fn empty_str() {
        assert_eq!(WordList::letter_sum(""), 0);
    }

    #[test]
    fn a() {
        assert_eq!(WordList::letter_sum("a"), 1);
    }

    #[test]
    fn z() {
        assert_eq!(WordList::letter_sum("z"), 26);
    }

    #[test]
    fn cab() {
        assert_eq!(WordList::letter_sum("cab"), 6);
    }

    #[test]
    fn excellent() {
        assert_eq!(WordList::letter_sum("excellent"), 100);
    }

    #[test]
    fn microspectrophotometries() {
        assert_eq!(WordList::letter_sum("microspectrophotometries"), 317);
    }
}
