//! Maintenance Interrupt Status Register, GICH_MISR
//! The GICH_MISR characteristics are:
//!
//! ## Purpose
//!
//! Indicates which maintenance interrupts are asserted.
//!
//! ## Configuration
//!
//! This register is present only when FEAT_GICv3_LEGACY is implemented and EL2 is implemented. Otherwise, direct accesses to GICH_MISR are RES0.
//!
//! This register is available when the GIC implementation supports interrupt virtualization.
//!
//! ## Attributes
//!
//! GICH_MISR is a 32-bit register.

use tock_registers::register_bitfields;
use tock_registers::registers::ReadOnly;

register_bitfields! {u32,
    pub GICH_MISR [
        /// [31:8] Reserved, RES0.
        Reserved31_8 OFFSET(8) NUMBITS(24) [],
        /// [7] VGrp1D
        /// vPE Group 1 Disabled maintenance interrupt status.
        VGrp1D OFFSET(7) NUMBITS(1) [
            NotAsserted = 0,
            Asserted = 1
        ],
        /// [6] VGrp1E
        /// vPE Group 1 Enabled maintenance interrupt status.
        VGrp1E OFFSET(6) NUMBITS(1) [
            NotAsserted = 0,
            Asserted = 1
        ],
        /// [5] VGrp0D
        /// vPE Group 0 Disabled maintenance interrupt status.
        VGrp0D OFFSET(5) NUMBITS(1) [
            NotAsserted = 0,
            Asserted = 1
        ],
        /// [4] VGrp0E
        /// vPE Group 0 Enabled maintenance interrupt status.
        VGrp0E OFFSET(4) NUMBITS(1) [
            NotAsserted = 0,
            Asserted = 1
        ],
        /// [3] NP
        /// No Pending maintenance interrupt status.
        NP OFFSET(3) NUMBITS(1) [
            NotAsserted = 0,
            Asserted = 1
        ],
        /// [2] LRENP
        /// List Register Entry Not Present maintenance interrupt status.
        LRENP OFFSET(2) NUMBITS(1) [
            NotAsserted = 0,
            Asserted = 1
        ],
        /// [1] U
        /// Underflow maintenance interrupt status.
        U OFFSET(1) NUMBITS(1) [
            NotAsserted = 0,
            Asserted = 1
        ],
        /// [0] EOI
        /// End Of Interrupt maintenance interrupt status.
        EOI OFFSET(0) NUMBITS(1) [
            NotAsserted = 0,
            Asserted = 1
        ]
    ]
}

/// Maintenance Interrupt Status Register, GICH_MISR
pub type GichMisrReg = ReadOnly<u32, GICH_MISR::Register>;
