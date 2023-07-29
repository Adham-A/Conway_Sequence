mod generators {
    pub mod naive;
    pub mod smart;
}

pub use generators::naive::NaiveGenerator;
pub use generators::smart::SmartGenerator;

mod lookups {
    pub mod elements;
}

pub use lookups::elements::DECAYS;
