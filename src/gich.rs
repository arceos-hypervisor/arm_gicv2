//! GIC Hypervisor Interface
//!
//! This module provides an interface to interact with the Generic Interrupt Controller (GIC)
//! in a hypervisor context. It allows the hypervisor to manage virtual interrupts for virtual
//! machines by accessing and manipulating GIC hypervisor interface registers.
//!

use core::ptr::NonNull;

use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::register_structs;

use crate::regs::gich::*;
use crate::GIC_LIST_REGS_NUM;

register_structs! {
    /// GIC Hypervisor Interface registers.
    #[allow(non_snake_case)]
    GicHypervisorInterfaceRegs {
        /// Hypervisor Control Register.
        (0x0000 => HCR: GichHcrReg),
        /// VGIC Type Register.
        (0x0004 => VTR: GichVtrReg),
        /// Virtual Machine Control Register.
        (0x0008 => VMCR: GichVmcrReg),
        (0x000c => _reserved0: [u32; 1]),
        /// Maintenance Interrupt Control Register.
        (0x0010 => MISR: GichMisrReg),
        (0x0014 => _reserved1: [u32; 1]),
        /// End of Interrupt Status Register.
        (0x0020 => EISR: [GichEisrReg; GIC_LIST_REGS_NUM / 32]),
        (0x0028 => _reserved2: [u32; 1]),
        /// End of Interrupt Clear Register.
        (0x0030 => ELRSR: [GichElrsrReg; GIC_LIST_REGS_NUM / 32]),
        (0x0038 => _reserved3: [u32; 1]),
        /// Active Priorities Register
        (0x00f0 => APR: GichAprReg),
        (0x00f4 => _reserved4: [u32; 1]),
        /// List Registers.
        (0x0100 => LR: [GichLrReg; GIC_LIST_REGS_NUM]),
        (0x0200 => _reserved5: [u32; 256]),
        (0x1000 => @END),
    }
}

/// The GIC Hypervisor Interface.
/// GIC (Generic Interrupt Controller) Hypervisor Interface.
///
/// This structure represents the hypervisor interface of a GICv2 interrupt controller.
/// It provides access to the GIC hypervisor interface registers through a non-null pointer.
///
/// # Safety
///
/// The base pointer must point to valid GIC hypervisor interface registers mapped in memory.
pub struct GicHypervisorInterface {
    base: NonNull<GicHypervisorInterfaceRegs>,
}

unsafe impl Send for GicHypervisorInterface {}
unsafe impl Sync for GicHypervisorInterface {}

/// GicHypervisorInterface provides an interface to interact with the GIC (Generic Interrupt Controller)
/// in a hypervisor context. It allows reading and writing to various GIC registers.
impl GicHypervisorInterface {
    /// Construct a new GIC Hypervisor interface instance from the base address.
    ///
    /// # Arguments
    ///
    /// * `base` - A mutable pointer to the base address of the GIC registers.
    ///
    /// # Returns
    ///
    /// A new instance of `GicHypervisorInterface`.
    pub const fn new(base: *mut u8) -> Self {
        Self {
            base: NonNull::new(base).unwrap().cast(),
        }
    }

    /// Get a reference to the GIC Hypervisor Interface Registers.
    ///
    /// # Safety
    ///
    /// This function dereferences a raw pointer, so it is marked as unsafe.
    const fn regs(&self) -> &GicHypervisorInterfaceRegs {
        unsafe { self.base.as_ref() }
    }

    /// Get the value of the HCR (Hypervisor Control Register).
    ///
    /// # Returns
    ///
    /// The current value of the HCR register.
    pub fn get_hcr(&self) -> u32 {
        self.regs().HCR.get()
    }

    /// Set the value of the HCR (Hypervisor Control Register).
    ///
    /// # Arguments
    ///
    /// * `hcr` - The value to set the HCR register to.
    pub fn set_hcr(&self, hcr: u32) {
        self.regs().HCR.set(hcr);
    }

    /// Get the value of the ELRSR (Empty List Register Status Register) at the specified index.
    ///
    /// # Arguments
    ///
    /// * `idx` - The index of the ELRSR register to read.
    ///
    /// # Returns
    ///
    /// The current value of the specified ELRSR register.
    pub fn elrsr(&self, idx: usize) -> u32 {
        self.regs().ELRSR[idx].get()
    }

    /// Get the value of the EISR (Empty Interrupt Status Register) at the specified index.
    ///
    /// # Arguments
    ///
    /// * `idx` - The index of the EISR register to read.
    ///
    /// # Returns
    ///
    /// The current value of the specified EISR register.
    pub fn eisr(&self, idx: usize) {
        self.regs().EISR[idx].get();
    }

    /// Get the value of the MISR (Maintenance Interrupt Status Register).
    ///
    /// # Returns
    ///
    /// The current value of the MISR register.
    pub fn misr(&self) -> u32 {
        self.regs().MISR.get()
    }

    /// Get the value of the APR (Active Priority Register).
    ///
    /// # Returns
    ///
    /// The current value of the APR register.
    pub fn apr(&self) -> u32 {
        self.regs().APR.get()
    }

    /// Get the value of the LR (List Register) at the specified index.
    ///
    /// # Arguments
    ///
    /// * `idx` - The index of the LR register to read.
    ///
    /// # Returns
    ///
    /// The current value of the specified LR register.
    pub fn get_lr(&self, idx: usize) -> u32 {
        self.regs().LR[idx].get()
    }

    /// Set the value of the LR (List Register) at the specified index.
    ///
    /// # Arguments
    ///
    /// * `idx` - The index of the LR register to write.
    /// * `lr` - The value to set the LR register to.
    pub fn set_lr(&self, idx: usize, lr: u32) {
        self.regs().LR[idx].set(lr);
    }

    /// Get the value of the VTR (Virtualization Translation Register).
    ///
    /// # Returns
    ///
    /// The number of List Registers supported by the GIC, derived from the VTR register.
    pub fn get_vtr(&self) -> u32 {
        let vtr: u32 = self.regs().VTR.get();
        (vtr & 0b11111) + 1
    }
}
