mod gicd_sgir;

pub use gicd_sgir::*;

#[cfg(feature = "hv")]
pub mod gich;
