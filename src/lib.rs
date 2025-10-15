//! Rust support for RISC-V ACLINT (Advanced Core Local Interruptor) peripheral.
//!
//! RISC-V ACLINT is defined in <https://github.com/riscv/riscv-aclint>.
#![no_std]

use core::cell::UnsafeCell;

/// Machine-level time counter register.
#[repr(transparent)]
pub struct MTIME(UnsafeCell<u64>);

/// Machine-level time compare register.
#[repr(transparent)]
pub struct MTIMECMP(UnsafeCell<u64>);

/// Machine-level IPI register.
#[repr(transparent)]
pub struct MSIP(UnsafeCell<u32>);

/// Set supervisor-level IPI register.
#[repr(transparent)]
pub struct SETSSIP(UnsafeCell<u32>);

/// Machine-level Software Interrupt Device (MSWI).
///
/// # Usage
///
/// ```no_run
/// impl rustsbi::Ipi for Clint {
///     #[inline]
///     fn send_ipi(&self, hart_mask: HartMask) -> SbiRet {
///         for i in hart_ids() {
///             if hart_mask.has_bit(i) && remote_hsm(i).map_or(false, |hsm| hsm.allow_ipi()) {
///                 // we assume this MSWI device covers hart id beginning from #0.
///                 self.mswi().set_msip(i);
///             }
///         }
///         SbiRet::success(0)
///     }
/// }
/// ```
#[repr(C)]
pub struct MSWI {
    /// HART index 0..4095 machine-level IPI registers.
    pub msip: [MSIP; 4095],
    _reserved: u32,
}

/// Supervisor-level Software Interrupt Device (SSWI).
#[repr(C)]
pub struct SSWI {
    pub setssip: [SETSSIP; 4095],
    _reserved: u32,
}

/// SiFive Core-Local Interruptor (CLINT) device.
#[repr(C)]
pub struct SifiveClint {
    /// Machine-level inter-processor (or software) interrupts.
    pub mswi: MSWI,
    /// Machine-level fixed-frequency counter and timer events; compartion part.
    pub mtimecmp: [MTIMECMP; 4095],
    /// Machine-level fixed-frequency counter and timer events; number of cycles part.
    pub mtime: MTIME,
}

impl SifiveClint {
    /// Read `MTIME` register.
    #[inline]
    pub fn read_mtime(&self) -> u64 {
        unsafe { self.mtime.0.get().read_volatile() }
    }

    /// Write `MTIME` register.
    #[inline]
    pub fn write_mtime(&self, val: u64) {
        unsafe { self.mtime.0.get().write_volatile(val) }
    }

    /// Read `MTIMECMP` register for the given hart.
    #[inline]
    pub fn read_mtimecmp(&self, hart_idx: usize) -> u64 {
        unsafe { self.mtimecmp[hart_idx].0.get().read_volatile() }
    }

    /// Write `MTIMECMP` register for the given hart.
    #[inline]
    pub fn write_mtimecmp(&self, hart_idx: usize, val: u64) {
        unsafe { self.mtimecmp[hart_idx].0.get().write_volatile(val) }
    }

    /// Read machine-level software interrupt state for given hart.
    #[inline]
    pub fn read_msip(&self, hart_idx: usize) -> bool {
        unsafe { self.mswi.msip[hart_idx].0.get().read_volatile() != 0 }
    }

    /// Set machine-level software interrupt for given hart.
    #[inline]
    pub fn set_msip(&self, hart_idx: usize) {
        unsafe { self.mswi.msip[hart_idx].0.get().write_volatile(1) }
    }

    /// Clear machine-level software interrupt for given hart.
    #[inline]
    pub fn clear_msip(&self, hart_idx: usize) {
        unsafe { self.mswi.msip[hart_idx].0.get().write_volatile(0) }
    }
}

#[test]
fn test() {
    assert_eq!(core::mem::size_of::<MSWI>(), 0x4000);
    assert_eq!(core::mem::size_of::<SSWI>(), 0x4000);
    assert_eq!(core::mem::size_of::<[MTIMECMP; 4095]>(), 0x7ff8);
    assert_eq!(core::mem::size_of::<SifiveClint>(), 0xc000);
}
