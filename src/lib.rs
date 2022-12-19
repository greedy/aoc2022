pub mod inputs;
pub mod grid;
pub mod iterators;
pub mod bitset;

pub mod prelude {
    pub use clap::Parser;
    pub use std::io::prelude::*;
    pub use color_eyre::eyre::{Report, Result, eyre, bail};
    pub use crate::inputs::InputCLI;

    pub use crate::iterators::AocItertools;
}
