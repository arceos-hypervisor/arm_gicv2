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
        // Status [
            Status15 OFFSET(15) NUMBITS(1) [
                NoEOI = 0,
                EOIAsserted = 1
            ],
            Status14 OFFSET(14) NUMBITS(1) [
                NoEOI = 0,
                EOIAsserted = 1
            ],
            Status13 OFFSET(13) NUMBITS(1) [
                NoEOI = 0,
                EOIAsserted = 1
            ],
            Status12 OFFSET(12) NUMBITS(1) [
                NoEOI = 0,
                EOIAsserted = 1
            ],
            Status11 OFFSET(11) NUMBITS(1) [
                NoEOI = 0,
                EOIAsserted = 1
            ],
            Status10 OFFSET(10) NUMBITS(1) [
                NoEOI = 0,
                EOIAsserted = 1
            ],
            Status9 OFFSET(9) NUMBITS(1) [
                NoEOI = 0,
                EOIAsserted = 1
            ],
            Status8 OFFSET(8) NUMBITS(1) [
                NoEOI = 0,
                EOIAsserted = 1
            ],
            Status7 OFFSET(7) NUMBITS(1) [
                NoEOI = 0,
                EOIAsserted = 1
            ],
            Status6 OFFSET(6) NUMBITS(1) [
                NoEOI = 0,
                EOIAsserted = 1
            ],
            Status5 OFFSET(5) NUMBITS(1) [
                NoEOI = 0,
                EOIAsserted = 1
            ],
            Status4 OFFSET(4) NUMBITS(1) [
                NoEOI = 0,
                EOIAsserted = 1
            ],
            Status3 OFFSET(3) NUMBITS(1) [
                NoEOI = 0,
                EOIAsserted = 1
            ],
            Status2 OFFSET(2) NUMBITS(1) [
                NoEOI = 0,
                EOIAsserted = 1
            ],
            Status1 OFFSET(1) NUMBITS(1) [
                NoEOI = 0,
                EOIAsserted = 1
            ],
            Status0 OFFSET(0) NUMBITS(1) [
                NoEOI = 0,
                EOIAsserted = 1
            ]
        // ]
    ]
}

/// End Interrupt Status Register, GICH_EISR
pub type GichEisrReg = ReadOnly<u32, GICH_EISR::Register>;
