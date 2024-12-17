//! Virtual Machine Control Register, GICH_VMCR
//! The GICH_VMCR characteristics are:
//!
//! ## Purpose
//!
//! Enables the hypervisor to save and restore the virtual machine view of the GIC state. This register is updated when a virtual machine updates the virtual CPU interface registers.
//!
//! ## Configuration
//!
//! This register is present only when FEAT_GICv3_LEGACY is implemented and EL2 is implemented. Otherwise, direct accesses to GICH_VMCR are RES0.
//!
//! This register is available when the GIC implementation supports interrupt virtualization.
//!
//! ## Attributes
//!
//! GICH_VMCR is a 32-bit register.

use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;

register_bitfields! {u32,
    pub GICH_VMCR [
        /// [31:24] VPMR
        /// Virtual priority mask. The priority mask level for the CPU interface.
        VPMR OFFSET(24) NUMBITS(8) [],
        /// [23:21] VBPR0
        /// Virtual Binary Point Register, Group 0.
        VBPR0 OFFSET(21) NUMBITS(3) [],
        /// [20:18] VBPR1
        /// Virtual Binary Point Register, Group 1.
        VBPR1 OFFSET(18) NUMBITS(3) [],
        /// [17:10] Reserved, RES0.
        Reserved17_10 OFFSET(10) NUMBITS(8) [],
        /// [9] VEOIM
        /// Virtual EOImode.
        VEOIM OFFSET(9) NUMBITS(1) [
            DropPriorityAndDeactivate = 0,
            DropPriorityOnly = 1
        ],
        /// [8:5] Reserved, RES0.
        Reserved8_5 OFFSET(5) NUMBITS(4) [],
        /// [4] VCBPR
        /// Virtual Common Binary Point Register.
        VCBPR OFFSET(4) NUMBITS(1) [
            Group1PreemptionByABPR = 0,
            Group1PreemptionByBPR = 1
        ],
        /// [3] VFIQEn
        /// Virtual FIQ enable.
        VFIQEn OFFSET(3) NUMBITS(1) [
            Group0AsIRQ = 0,
            Group0AsFIQ = 1
        ],
        /// [2] VAckCtl
        /// Virtual AckCtl.
        VAckCtl OFFSET(2) NUMBITS(1) [
            Group1ReturnsINTID1022 = 0,
            Group1ReturnsActualINTID = 1
        ],
        /// [1] VENG1
        /// Virtual interrupt enable, Group 1.
        VENG1 OFFSET(1) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// [0] VENG0
        /// Virtual interrupt enable, Group 0.
        VENG0 OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ]
}

/// Virtual Machine Control Register, GICH_VMCR
pub type GichVmcrReg = ReadWrite<u32, GICH_VMCR::Register>;
