//! List Registers, ``GICH_LR<n>``, n = 0 - 15
//! The `GICH_LR<n>` characteristics are:
//!
//! ## Purpose
//!
//! These registers provide context information for the virtual CPU interface.
//!
//! ## Configuration
//!
//! This register is present only when FEAT_GICv3_LEGACY is implemented and EL2 is implemented. Otherwise, direct accesses to `GICH_LR<n>` are RES0.
//!
//! This register is available when the GIC implementation supports interrupt virtualization.
//!
//! A maximum of 16 List registers can be provided. GICH_VTR.ListRegs defines the number implemented. Unimplemented List registers are RAZ/WI.
//!
//! ## Attributes
//!
//! `GICH_LR<n>` is a 32-bit register.

use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;

register_bitfields! {u32,
    pub GICH_LR [
        /// [31] HW
        /// Indicates whether this virtual interrupt is a hardware interrupt.
        HW OFFSET(31) NUMBITS(1) [
            Software = 0,
            Hardware = 1
        ],
        /// [30] Group
        /// Indicates whether the interrupt is Group 0 or Group 1.
        Group OFFSET(30) NUMBITS(1) [
            Group0 = 0,
            Group1 = 1
        ],
        /// [29:28] State
        /// The state of the interrupt.
        State OFFSET(28) NUMBITS(2) [
            Inactive = 0b00,
            Pending = 0b01,
            Active = 0b10,
            ActiveAndPending = 0b11
        ],
        /// [27:23] Priority
        /// The priority of this interrupt.
        Priority OFFSET(23) NUMBITS(5) [],
        /// [22:20] Reserved, RES0.
        Reserved22_20 OFFSET(20) NUMBITS(3) [],
        /// [19:10] pINTID
        /// The physical interrupt ID.
        /// The function of this field depends on the value of GICH_LR<n>.HW.
        pINTID OFFSET(10) NUMBITS(10) [],
        /// [9:0] vINTID
        /// The virtual interrupt ID.
        vINTID OFFSET(0) NUMBITS(10) []
    ]
}

/// List Registers, GICH_LR<n>, n = 0 - 15
pub type GichLrReg = ReadWrite<u32, GICH_LR::Register>;
