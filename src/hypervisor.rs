/// Hypervisor Integration Module
/// 
/// This module implements the hypervisor-first design principle of RustOS,
/// providing abstraction layers for different hypervisors (KVM, Xen, etc.)

use core::fmt;
use alloc::vec::Vec;

/// Supported hypervisor types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HypervisorType {
    Kvm,
    Xen,
    HyperV,
    VMware,
    Qemu,
}

/// Hypervisor interface trait
pub trait Hypervisor {
    fn init(&self) -> Result<(), HypervisorError>;
    fn get_type(&self) -> HypervisorType;
    fn get_memory_layout(&self) -> MemoryLayout;
    fn register_interrupt_handler(&self, vector: u8, handler: fn()) -> Result<(), HypervisorError>;
}

/// Memory layout information from hypervisor
#[derive(Debug, Clone)]
pub struct MemoryLayout {
    pub total_memory: u64,
    pub usable_memory: u64,
    pub reserved_regions: Vec<MemoryRegion>,
}

/// Memory region descriptor
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub start: u64,
    pub size: u64,
    pub region_type: MemoryRegionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryRegionType {
    Usable,
    Reserved,
    Mmio,
    Hypervisor,
}

/// Hypervisor-specific errors
#[derive(Debug)]
pub enum HypervisorError {
    InitializationFailed,
    UnsupportedFeature,
    HardwareNotSupported,
    InvalidConfiguration,
}

impl fmt::Display for HypervisorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HypervisorError::InitializationFailed => write!(f, "Hypervisor initialization failed"),
            HypervisorError::UnsupportedFeature => write!(f, "Unsupported hypervisor feature"),
            HypervisorError::HardwareNotSupported => write!(f, "Hardware not supported"),
            HypervisorError::InvalidConfiguration => write!(f, "Invalid hypervisor configuration"),
        }
    }
}

/// Global hypervisor instance
static mut HYPERVISOR_INSTANCE: Option<&'static dyn Hypervisor> = None;

/// Initialize hypervisor integration
pub fn init() {
    crate::println!("Initializing hypervisor integration...");
    
    // Detect hypervisor type
    let hypervisor_type = detect_hypervisor();
    crate::println!("Detected hypervisor: {:?}", hypervisor_type);
    
    // Initialize appropriate hypervisor driver
    match hypervisor_type {
        HypervisorType::Kvm => {
            // Initialize KVM-specific integration
            crate::println!("Initializing KVM integration");
        }
        HypervisorType::Xen => {
            // Initialize Xen-specific integration  
            crate::println!("Initializing Xen integration");
        }
        _ => {
            crate::println!("Generic hypervisor initialization");
        }
    }
    
    crate::println!("Hypervisor integration initialized");
}

/// Detect the current hypervisor
fn detect_hypervisor() -> HypervisorType {
    // TODO: Implement proper hypervisor detection using CPUID, etc.
    // For now, default to KVM
    HypervisorType::Kvm
}

/// Get the current hypervisor instance
pub fn get_hypervisor() -> Option<&'static dyn Hypervisor> {
    unsafe { HYPERVISOR_INSTANCE }
}