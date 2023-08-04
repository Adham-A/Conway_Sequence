use crate::lookups::element_decay::ELEMENT_DECAY_LOOKUP;

#[derive(Default)]
pub struct SubSequence {
    numbers: Option<String>,
    atom: Option<String>,
}

impl SubSequence {
    pub fn new() -> SubSequence {
        SubSequence {
            numbers: None,
            atom: None,
        }
    }

    pub fn number_to_atom(&mut self) {
        if let Some(numbers) = &self.numbers {
            println!("{:?}", &self.numbers);
            if let Some(atom) = ELEMENT_DECAY_LOOKUP.get(numbers) {
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

    pub fn next(self) -> Vec<SubSequence> {
        vec![self]
    }
}
