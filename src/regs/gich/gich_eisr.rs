//! End Interrupt Status Register, GICH_EISR
//! The GICH_EISR characteristics are:
//!
//! ## Purpose
//!
//! Indicates which List registers have outstanding EOI maintenance interrupts.
//!
//! ## Configuration
//!
//! This register is present only when FEAT_GICv3_LEGACY is implemented and EL2 is implemented. Otherwise, direct accesses to GICH_EISR are RES0.
//!
//! This register is available when the GIC implementation supports interrupt virtualization.
//!
//! ## Attributes
//!
//! GICH_EISR is a 32-bit register.

use tock_registers::register_bitfields;
use tock_registers::registers::ReadOnly;

register_bitfields! {u32,
    pub GICH_EISR [
        /// [31:16] Reserved, RES0.
        Reserved31_16 OFFSET(16) NUMBITS(16) [],
        /// [15:0] Status<n>
        /// EOI maintenance interrupt status for List register <n>.
        Status OFFSET(0) NUMBITS(16) [],
    ]
}

/// End Interrupt Status Register, GICH_EISR
pub type GichEisrReg = ReadOnly<u32, GICH_EISR::Register>;
