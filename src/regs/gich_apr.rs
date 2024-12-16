//! Active Priorities Registers, GICH_APR<n>, n = 0 - 3
//! The GICH_APR<n> characteristics are:
//!
//! ## Purpose
//!
//! These registers track which preemption levels are active in the virtual CPU interface, and indicate the current active priority.
//!
//! ## Configuration
//!
//! This register is present only when FEAT_GICv3_LEGACY is implemented and EL2 is implemented. Otherwise, direct accesses to GICH_APR<n> are RES0.
//!
//! This register is available when the GIC implementation supports interrupt virtualization.
//!
//! The number of registers required depends on how many bits are implemented in GICH_LR<n>.Priority:
//! - When 5 priority bits are implemented, 1 register is required (GICH_APR0).
//! - When 6 priority bits are implemented, 2 registers are required (GICH_APR0, GICH_APR1).
//! - When 7 priority bits are implemented, 4 registers are required (GICH_APR0, GICH_APR1, GICH_APR2, GICH_APR3).
//! Unimplemented registers are RAZ/WI.
//!
//! ## Attributes
//!
//! GICH_APR<n> is a 32-bit register.

use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;

register_bitfields! {u32,
    pub GICH_APR [
        /// [31:0] P<x>
        /// Active priorities. Each bit indicates whether there is an interrupt active at the priority corresponding to that bit.
        INTERRUPT31 OFFSET(31) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT30 OFFSET(30) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT29 OFFSET(29) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT28 OFFSET(28) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT27 OFFSET(27) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT26 OFFSET(26) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT25 OFFSET(25) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT24 OFFSET(24) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT23 OFFSET(23) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT22 OFFSET(22) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT21 OFFSET(21) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT20 OFFSET(20) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT19 OFFSET(19) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT18 OFFSET(18) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT17 OFFSET(17) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT16 OFFSET(16) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT15 OFFSET(15) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT14 OFFSET(14) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT13 OFFSET(13) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT12 OFFSET(12) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT11 OFFSET(11) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT10 OFFSET(10) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT9 OFFSET(9) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT8 OFFSET(8) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT7 OFFSET(7) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT6 OFFSET(6) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT5 OFFSET(5) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT4 OFFSET(4) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT3 OFFSET(3) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT2 OFFSET(2) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT1 OFFSET(1) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ],
        INTERRUPT0 OFFSET(0) NUMBITS(1) [
            NoActiveInterrupt = 0,
            ActiveInterrupt = 1
        ]
    ]
}

/// Active Priorities Register, GICH_APR<n>, n = 0 - 3
pub type GichAprReg = ReadWrite<u32, GICH_APR::Register>;
