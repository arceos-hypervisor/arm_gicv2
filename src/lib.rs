#![no_std]
#![feature(const_option)]
#![feature(const_nonnull_new)]
#![doc = include_str!("../README.md")]

use core::ops::Range;

mod regs;

mod gic_v2;

pub use gic_v2::{GicCpuInterface, GicDistributor};

/// Interrupt ID 0-15 are used for SGIs (Software-generated interrupt).
///
/// SGI is an interrupt generated by software writing to a GICD_SGIR register in
/// the GIC. The system uses SGIs for interprocessor communication.
pub const SGI_RANGE: Range<usize> = 0..16;

/// Interrupt ID 16-31 are used for PPIs (Private Peripheral Interrupt).
///
/// PPI is a peripheral interrupt that is specific to a single processor.
pub const PPI_RANGE: Range<usize> = 16..32;

/// Interrupt ID 32-1019 are used for SPIs (Shared Peripheral Interrupt).
///
/// SPI is a peripheral interrupt that the Distributor can route to any of a
/// specified combination of processors.
pub const SPI_RANGE: Range<usize> = 32..1020;

/// Maximum number of interrupts supported by the GIC.
pub const GIC_MAX_IRQ: usize = 1024;

/// Number of bits used to configure the trigger mode for each interrupt.
pub const GIC_CONFIG_BITS: usize = 2;

/// GICC_CTLR register bits.
///     bit 0 EnableGrp0:
/// Enable for the signaling of Group 0 interrupts by the CPU interface to the connected processor:
/// * 0: Disable signaling of Group 0 interrupts.
/// * 1: Enable signaling of Group 0 interrupts.
pub const GICC_CTLR_EN_BIT: u32 = 0x1;

/// GICC_CTLR register bits:
///     bit 9 EOImodeNS:
/// Controls the behavior of Non-secure accesses to the GICC_EOIR and GICC_DIR registers
/// * 0: GICC_EOIR has both priority drop and deactivate interrupt functionality. Accesses to the GICC_DIR are unpredictable.
/// * 1: GICC_EOIR has priority drop functionality only. The GICC_DIR register has deactivate interrupt functionality.
pub const GICC_CTLR_EOIMODENS_BIT: u32 = 1 << 9;

/// GICD_CTLR register bits:
///    bit 0 EnableGrp0:
/// Enable for the signaling of Group 0 interrupts by the Distributor to the connected processors:
/// * 0: Disable signaling of Group 0 interrupts.
/// * 1: Enable signaling of Group 0 interrupts.
const GICD_CTLR_EN_BIT: u32 = 0x1;

/// Interrupt trigger mode.
pub enum TriggerMode {
    /// Edge-triggered.
    ///
    /// This is an interrupt that is asserted on detection of a rising edge of
    /// an interrupt signal and then, regardless of the state of the signal,
    /// remains asserted until it is cleared by the conditions defined by this
    /// specification.
    Edge = 0,
    /// Level-sensitive.
    ///
    /// This is an interrupt that is asserted whenever the interrupt signal
    /// level is active, and deasserted whenever the level is not active.
    Level = 1,
}

/// Different types of interrupt that the GIC handles.
pub enum InterruptType {
    /// Software-generated interrupt.
    ///
    /// SGIs are typically used for inter-processor communication and are
    /// generated by a write to an SGI register in the GIC.
    SGI,
    /// Private Peripheral Interrupt.
    ///
    /// Peripheral interrupts that are private to one core.
    PPI,
    /// Shared Peripheral Interrupt.
    ///
    /// Peripheral interrupts that can delivered to any connected core.
    SPI,
}

/// Translate an interrupt of a given type to a GIC INTID.
pub const fn translate_irq(id: usize, int_type: InterruptType) -> Option<usize> {
    match int_type {
        InterruptType::SGI => {
            if id < SGI_RANGE.end {
                Some(id)
            } else {
                None
            }
        }
        InterruptType::PPI => {
            if id < PPI_RANGE.end - PPI_RANGE.start {
                Some(id + PPI_RANGE.start)
            } else {
                None
            }
        }
        InterruptType::SPI => {
            if id < SPI_RANGE.end - SPI_RANGE.start {
                Some(id + SPI_RANGE.start)
            } else {
                None
            }
        }
    }
}
