//! Active Priorities Registers, `GICH_APR<n>`, n = 0 - 3
//! The `GICH_APR<n>` characteristics are:
//!
//! ## Purpose
//!
//! These registers track which preemption levels are active in the virtual CPU interface, and indicate the current active priority.
//!
//! ## Configuration
//!
//! This register is present only when FEAT_GICv3_LEGACY is implemented and EL2 is implemented. Otherwise, direct accesses to `GICH_APR<n>` are RES0.
//!
//! This register is available when the GIC implementation supports interrupt virtualization.
//!
//! The number of registers required depends on how many bits are implemented in `GICH_LR<n>.Priority`:
//! - When 5 priority bits are implemented, 1 register is required (GICH_APR0).
//! - When 6 priority bits are implemented, 2 registers are required (GICH_APR0, GICH_APR1).
//! - When 7 priority bits are implemented, 4 registers are required (GICH_APR0, GICH_APR1, GICH_APR2, GICH_APR3).
//! Unimplemented registers are RAZ/WI.
//!
//! ## Attributes
//!
//! `GICH_APR<n>` is a 32-bit register.

use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;

register_bitfields! {u32,
    pub GICH_APR [
        /// [31:0] P<x>
        /// Active priorities. Each bit indicates whether there is an interrupt active at the priority corresponding to that bit.
        ACTIVE_PRIORITY_BITS OFFSET(0) NUMBITS(32) []
    ]
}

/// Active Priorities Register, GICH_APR<n>, n = 0 - 3
pub type GichAprReg = ReadWrite<u32, GICH_APR::Register>;
