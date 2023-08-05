mod generators {
    pub mod naive;
    pub mod smart;
}

pub use generators::naive::NaiveGenerator;
pub use generators::smart::SmartGenerator;

mod lookups {
    pub mod element_decay;
    pub mod sequence_element;
}

pub use lookups::element_decay::ELEMENT_DECAY_LOOKUP;
pub use lookups::sequence_element::SEQUENCE_ELEMENT_LOOKUP;
mod utils {
    pub mod split;
    pub mod sub_sequence;
}

pub use utils::split;
pub use utils::sub_sequence::SubSequence;
