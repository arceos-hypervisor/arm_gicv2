//! Empty List Register Status Register, GICH_ELRSR
//! The GICH_ELRSR characteristics are:
//!
//! ## Purpose
//!
//! Indicates which List registers contain valid interrupts.
//!
//! ## Configuration
//!
//! This register is present only when FEAT_GICv3_LEGACY is implemented and EL2 is implemented. Otherwise, direct accesses to GICH_ELRSR are RES0.
//!
//! This register is available when the GIC implementation supports interrupt virtualization.
//!
//! ## Attributes
//!
//! GICH_ELRSR is a 32-bit register.

use tock_registers::register_bitfields;
use tock_registers::registers::ReadOnly;

register_bitfields! {u32,
    pub GICH_ELRSR [
        /// [31:16] Reserved, RES0.
        Reserved31_16 OFFSET(16) NUMBITS(16) [],
        /// [15:0] Status<n>
        /// Status bit for List register <n>.
        Status OFFSET(15) NUMBITS(16) []
    ]
}

/// Empty List Register Status Register, GICH_ELRSR
pub type GichElrsrReg = ReadOnly<u32, GICH_ELRSR::Register>;
