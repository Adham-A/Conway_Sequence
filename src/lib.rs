mod generators {
    pub mod naive;
    pub mod smart;
}

// Re-export the public items from the modules
pub use generators::naive::NaiveGenerator;
pub use generators::smart::SmartGenerator;