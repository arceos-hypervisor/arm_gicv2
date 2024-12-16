//! Hypervisor Control Register, GICH_HCR
//! The GICH_HCR characteristics are:
//!
//! ## Purpose
//!
//! Controls the virtual CPU interface.
//!
//! ## Configuration
//!
//! This register is present only when FEAT_GICv3_LEGACY is implemented and EL2 is implemented. Otherwise, direct accesses to GICH_HCR are RES0.
//!
//! This register is available when the GIC implementation supports interrupt virtualization.
//!
//! ## Attributes
//!
//! GICH_HCR is a 32-bit register.

use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;

register_bitfields! {u32,
    pub GICH_HCR [
        /// [31:27] EOICount
        /// Counts the number of EOIs received that do not have a corresponding entry in the List registers.
        EOICount OFFSET(27) NUMBITS(5) [],
        /// [26:8] Reserved, RES0.
        Reserved26_8 OFFSET(8) NUMBITS(19) [],
        /// [7] VGrp1DIE
        /// VM Group 1 Disabled Interrupt Enable.
        VGrp1DIE OFFSET(7) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// [6] VGrp1EIE
        /// VM Group 1 Enabled Interrupt Enable.
        VGrp1EIE OFFSET(6) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// [5] VGrp0DIE
        /// VM Group 0 Disabled Interrupt Enable.
        VGrp0DIE OFFSET(5) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// [4] VGrp0EIE
        /// VM Group 0 Enabled Interrupt Enable.
        VGrp0EIE OFFSET(4) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// [3] NPIE
        /// No Pending Interrupt Enable.
        NPIE OFFSET(3) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// [2] LRENPIE
        /// List Register Entry Not Present Interrupt Enable.
        LRENPIE OFFSET(2) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// [1] UIE
        /// Underflow Interrupt Enable.
        UIE OFFSET(1) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// [0] En
        /// Enable.
        En OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ]
}

/// Hypervisor Control Register, GICH_HCR
pub type GichHcrReg = ReadWrite<u32, GICH_HCR::Register>;
