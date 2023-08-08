use crate::utils::sub_sequence::SubSequence;
use rayon::prelude::*;
use std::fmt;
pub struct Sequence {
    data: Vec<SubSequence>,
}

impl Sequence {
    pub fn new(seed: String) -> Sequence {
        Sequence {
            data: vec![SubSequence::new(Some(seed.clone()), None)],
        }
    }

    pub fn next(mut self) -> Self {
        let tmp = self.data;
        self.data = tmp.into_par_iter().flat_map(|f| f.next()).collect();
        self
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let it = self.data.iter();
        write!(
            f,
            "{}",
            it.map(|s| s.to_string()).collect::<Vec<String>>().join(".")
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::sequence::Sequence;
    use test_case::test_case;

    #[test_case("1".into(),50)]
    fn sequence_test_with_seed(seed: String, iterations: usize) {
        println!("{seed}");
        let mut s = Sequence::new(seed);
        for _ in 0..iterations {
            s = s.next();
            println!("{}", s);
        }
    }
}
