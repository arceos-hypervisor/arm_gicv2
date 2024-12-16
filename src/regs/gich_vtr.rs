//! Virtual Type Register, GICH_VTR
//! The GICH_VTR characteristics are:
//!
//! ## Purpose
//!
//! Indicates the number of implemented virtual priority bits and List registers.
//!
//! ## Configuration
//!
//! This register is present only when FEAT_GICv3_LEGACY is implemented and EL2 is implemented. Otherwise, direct accesses to GICH_VTR are RES0.
//!
//! This register is available when the GIC implementation supports interrupt virtualization.
//!
//! ## Attributes
//!
//! GICH_VTR is a 32-bit register.

use tock_registers::fields::FieldValue;
use tock_registers::register_bitfields;
use tock_registers::registers::ReadOnly;

register_bitfields! {u32,
    pub GICH_VTR [
        /// [31:29] PRIbits
        /// The number of virtual priority bits implemented, minus one.
        PRIbits OFFSET(29) NUMBITS(3) [],
        /// [28:26] PREbits
        /// The number of virtual preemption bits implemented, minus one.
        PREbits OFFSET(26) NUMBITS(3) [],
        /// [25:23] IDbits
        /// The number of virtual interrupt identifier bits supported.
        IDbits OFFSET(23) NUMBITS(3) [
            Bits16 = 0b000,
            Bits24 = 0b001,
            Reserved0 = 0b010,
            Reserved1 = 0b011,
            Reserved2 = 0b100,
            Reserved3 = 0b101,
            Reserved4 = 0b110,
            Reserved5 = 0b111
        ],
        /// [22] SEIS
        /// SEI support. Indicates whether the virtual CPU interface supports generation of SEIs.
        SEIS OFFSET(22) NUMBITS(1) [
            NotSupported = 0,
            Supported = 1
        ],
        /// [21] A3V
        /// Affinity 3 valid. Indicates whether the virtual CPU interface supports nonzero values of the Aff3 field.
        A3V OFFSET(21) NUMBITS(1) [
            NotValid = 0,
            Valid = 1
        ],
        /// [20:5] Reserved, RES0.
        Reserved20_5 OFFSET(5) NUMBITS(16) [],
        /// [4:0] ListRegs
        /// The number of implemented List registers, minus one.
        ListRegs OFFSET(0) NUMBITS(5) []
    ]
}

/// Virtual Type Register, GICH_VTR
pub type GichVtrReg = ReadOnly<u32, GICH_VTR::Register>;
