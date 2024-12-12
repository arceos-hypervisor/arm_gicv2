//! Software Generated Interrupt Register, GICD_SGIR
//! The GICD_SGIR characteristics are:
//!
//! ## Purpose
//!
//! Controls the generation of SGIs.
//!
//! ## Usage constraints
//!
//! It is IMPLEMENTATION DEFINED whether the GICD_SGIR has any effect when the forwarding of interrupts by Distributor is disabled by the GICD_CTLR settings.
//!
//! ## Configurations
//!
//! This register is available in all configurations of the GIC. If the GIC implements the Security Extensions this register is Common.
//!
//! The NSATT field, bit [15], is implemented only if the GIC implements the Security Extensions.

use tock_registers::register_bitfields;
use tock_registers::registers::WriteOnly;

register_bitfields! {u32,
    pub GICD_SGIR [
        /// Bits [31:26] Reserved.
        Reserved31 OFFSET(26) NUMBITS(6) [],
        /// [25:24] TargetListFilter
        /// Determines how the distributor must process the requested SGI:
        /// - 0b00 Forward the interrupt to the CPU interfaces specified in the CPUTargetList fielda.
        /// - 0b01 Forward the interrupt to all CPU interfaces except that of the processor that requested the interrupt.
        /// - 0b10 Forward the interrupt only to the CPU interface of the processor that requested the interrupt.
        /// - 0b11 Reserved.
        TargetListFilter OFFSET(24) NUMBITS(2) [
            ForwardToCPUTargetList = 0b00,
            ForwardToAllExceptRequester = 0b01,
            ForwardToRequester = 0b10,
            Reserved = 0b11
        ],
        /// [23:16] CPUTargetList
        /// When TargetList Filter = 0b00, defines the CPU interfaces to which the Distributor must forward the interrupt.
        /// Each bit of CPUTargetList[7:0] refers to the corresponding CPU interface, for example CPUTargetList[0] corresponds to CPU interface 0. Setting a bit to 1 indicates that the interrupt must be forwarded to the corresponding interface.
        /// If this field is 0x00 when TargetListFilter is 0b00, the Distributor does not forward the interrupt to any CPU interface.
        CPUTargetList OFFSET(16) NUMBITS(8) [],
        /// [15] NSATT
        /// Implemented only if the GIC includes the Security Extensions.
        /// Specifies the required security value of the SGI:
        /// - 0 Forward the SGI specified in the SGIINTID field to a specified CPU interface only if the SGI is configured as Group 0 on that interface.
        /// - 1 Forward the SGI specified in the SGIINTID field to a specified CPU interfaces only if the SGI is configured as Group 1 on that interface.
        ///
        /// This field is writable only by a Secure access. Any Non-secure write to the GICD_SGIR generates an SGI only if the specified SGI is programmed as Group 1, regardless of the value of bit[15] of the write.
        NSATT OFFSET(15) NUMBITS(1) [],
        /// [14:4] - Reserved, SBZ.
        Reserved14_4 OFFSET(4) NUMBITS(11) [],
        /// [3:0] SGIINTID
        /// The Interrupt ID of the SGI to forward to the specified CPU interfaces.
        /// The value of this field is the Interrupt ID, in the range 0-15,
        /// for example a value of 0b0011 specifies Interrupt ID 3.
        SGIINTID OFFSET(0) NUMBITS(4) []
    ]
}

/// Software Generated Interrupt Register, GICD_SGIR
pub type GicdSgirReg = WriteOnly<u32, GICD_SGIR::Register>;
