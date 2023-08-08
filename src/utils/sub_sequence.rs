use crate::generators::naive::NaiveGenerator;
use crate::lookups::element_decay::ELEMENT_DECAY_LOOKUP;
use crate::lookups::sequence_element::SEQUENCE_ELEMENT_LOOKUP;
use crate::utils::split::is_valid_split;
use rayon::prelude::*;
use std::fmt;
#[derive(Default)]
pub struct SubSequence {
    pub numbers: Option<String>,
    pub atom: Option<String>,
}

impl fmt::Display for SubSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(numbers) = &self.numbers {
            write!(f, "{}", numbers)?;
        }
        if let Some(atom) = &self.atom {
            write!(f, "{}", atom)?;
        }
        Ok(())
    }
}

impl SubSequence {
    pub fn new(numbers: Option<String>, atom: Option<String>) -> SubSequence {
        SubSequence { numbers, atom }
    }

    pub fn number_to_atom(&mut self) {
        if let Some(numbers) = &self.numbers {
            if let Some(atom) = SEQUENCE_ELEMENT_LOOKUP.get(numbers) {
                self.atom = Some(String::from(*atom));
                self.numbers = None;
            }
        }
    }

    pub fn is_atomic(&self) -> bool {
        self.atom.is_some()
    }

    pub fn is_not_atomic(&self) -> bool {
        self.atom.is_none()
    }

    fn next_atomic(self) -> Vec<Self> {
        ELEMENT_DECAY_LOOKUP
            .get(&self.atom.unwrap())
            .unwrap()
            .split('.')
            .map(|s| SubSequence::new(None, Some(s.to_string())))
            .collect()
    }

    pub fn next(self) -> Vec<Self> {
        if self.is_atomic() {
            self.next_atomic()
        } else {
            // Fallbacks to naive generator
            let mut results: Vec<Self> = vec![];
            let chars: Vec<char> = self.numbers.unwrap().chars().collect();
            let numbers = NaiveGenerator::new().next_sequence(&chars);

            let generated = SubSequence::new(Some(numbers.into_iter().collect()), None);

            let numbers = generated.numbers.unwrap();
            let length = numbers.len();

            let mut start: usize = 0;

            for i in 1..length {
                let l = &numbers[start..i];
                let r = &numbers[i..];

                if is_valid_split(l, r) {
                    results.push(Self::new(Some(String::from(l)), None));
                    start = i;
                }
            }

            results.push(Self::new(Some(String::from(&numbers[start..])), None));

            results.par_iter_mut().for_each(|e| e.number_to_atom());
            results
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::sub_sequence::SubSequence;
    use test_case::test_case;

    #[test_case("1113213211".into())]
    fn sub_sequence_next_split(numbers: String) {
        let s = SubSequence::new(numbers.into(), None);
        let res = s.next();
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].atom.as_ref().unwrap(), "Lu");
        assert_eq!(res[1].atom.as_ref().unwrap(), "In");
    }

    #[test_case("131112".into())]
    fn sub_sequence_next_no_split(number: String) {
        let s = SubSequence::new(number.into(), None);
        let res = s.next();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].atom.as_ref().unwrap(), "Ni");
    }
    #[test_case("Zn".into())]
    fn sub_sequence_next_atomic(atom: String) {
        let s = SubSequence::new(None, atom.into());
        let res = s.next_atomic();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].atom.as_ref().unwrap(), "Cu");
    }

    #[test_case("Ni".into())]
    fn sub_sequence_next_atomic_multiple(atom: String) {
        let s = SubSequence::new(None, atom.into());
        let res = s.next_atomic();
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].atom.as_ref().unwrap(), "Zn");
        assert_eq!(res[1].atom.as_ref().unwrap(), "Co");
    }
}
