/// Process and Thread Management
/// 
/// Implements modern process model with:
/// - Async-first design
/// - Capability-based process isolation
/// - Lightweight thread management
/// - Resource tracking

use core::sync::atomic::{AtomicU32, Ordering};
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use alloc::string::String;
use crate::capability::{Capability, CapabilityId};
use crate::memory::MemoryRegion;

/// Process ID type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessId(pub u32);

/// Thread ID type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadId(pub u32);

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Created,
    Ready,
    Running,
    Blocked,
    Terminated,
}

/// Thread state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Created,
    Ready,
    Running,
    Blocked,
    Terminated,
}

/// Process control block
#[derive(Debug)]
pub struct ProcessControlBlock {
    pub pid: ProcessId,
    pub parent_pid: Option<ProcessId>,
    pub state: ProcessState,
    pub capabilities: Vec<Capability>,
    pub memory_regions: Vec<MemoryRegion>,
    pub threads: Vec<ThreadId>,
    pub priority: u8,
    pub creation_time: u64,
    pub cpu_time: u64,
}

/// Thread control block
#[derive(Debug)]
pub struct ThreadControlBlock {
    pub tid: ThreadId,
    pub pid: ProcessId,
    pub state: ThreadState,
    pub stack_pointer: usize,
    pub instruction_pointer: usize,
    pub priority: u8,
    pub cpu_time: u64,
}

/// Process creation parameters
#[derive(Debug, Clone)]
pub struct ProcessCreateParams {
    pub program_path: String,
    pub arguments: Vec<String>,
    pub environment: Vec<(String, String)>,
    pub capabilities: Vec<CapabilityId>,
    pub memory_limit: Option<usize>,
    pub cpu_limit: Option<u64>,
}

/// Thread creation parameters
#[derive(Debug, Clone)]
pub struct ThreadCreateParams {
    pub entry_point: usize,
    pub stack_size: usize,
    pub priority: u8,
}

/// Process manager errors
#[derive(Debug)]
pub enum ProcessError {
    ProcessNotFound,
    ThreadNotFound,
    InsufficientPermissions,
    ResourceExhausted,
    InvalidParameters,
    SystemError(String),
}

/// Process manager
pub struct ProcessManager {
    processes: Vec<ProcessControlBlock>,
    threads: Vec<ThreadControlBlock>,
    next_pid: AtomicU32,
    next_tid: AtomicU32,
    ready_queue: VecDeque<ThreadId>,
    current_thread: Option<ThreadId>,
}

impl ProcessManager {
    /// Create a new process manager
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            threads: Vec::new(),
            next_pid: AtomicU32::new(1),
            next_tid: AtomicU32::new(1),
            ready_queue: VecDeque::new(),
            current_thread: None,
        }
    }
    
    /// Create a new process
    pub fn create_process(&mut self, params: ProcessCreateParams) -> Result<ProcessId, ProcessError> {
        let pid = ProcessId(self.next_pid.fetch_add(1, Ordering::SeqCst));
        
        // TODO: Load program from file system
        // TODO: Set up initial memory regions
        // TODO: Validate capabilities
        
        let pcb = ProcessControlBlock {
            pid,
            parent_pid: None, // TODO: Get current process PID
            state: ProcessState::Created,
            capabilities: Vec::new(), // TODO: Convert CapabilityIds to Capabilities
            memory_regions: Vec::new(),
            threads: Vec::new(),
            priority: 128, // Default priority
            creation_time: 0, // TODO: Get current time
            cpu_time: 0,
        };
        
        self.processes.push(pcb);
        
        // Create initial thread
        let thread_params = ThreadCreateParams {
            entry_point: 0x400000, // TODO: Get entry point from executable
            stack_size: 0x100000,  // 1MB stack
            priority: 128,
        };
        
        let tid = self.create_thread(pid, thread_params)?;
        
        // Add thread to process
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.threads.push(tid);
            process.state = ProcessState::Ready;
        }
        
        Ok(pid)
    }
    
    /// Create a new thread
    pub fn create_thread(&mut self, pid: ProcessId, params: ThreadCreateParams) -> Result<ThreadId, ProcessError> {
        // Verify process exists
        if !self.processes.iter().any(|p| p.pid == pid) {
            return Err(ProcessError::ProcessNotFound);
        }
        
        let tid = ThreadId(self.next_tid.fetch_add(1, Ordering::SeqCst));
        
        // TODO: Allocate stack
        // TODO: Set up initial register state
        
        let tcb = ThreadControlBlock {
            tid,
            pid,
            state: ThreadState::Ready,
            stack_pointer: params.entry_point + params.stack_size, // TODO: Proper stack setup
            instruction_pointer: params.entry_point,
            priority: params.priority,
            cpu_time: 0,
        };
        
        self.threads.push(tcb);
        self.ready_queue.push_back(tid);
        
        Ok(tid)
    }
    
    /// Terminate a process
    pub fn terminate_process(&mut self, pid: ProcessId) -> Result<(), ProcessError> {
        // Find and terminate all threads in the process
        let thread_ids: Vec<ThreadId> = self.threads
            .iter()
            .filter(|t| t.pid == pid)
            .map(|t| t.tid)
            .collect();
        
        for tid in thread_ids {
            self.terminate_thread(tid)?;
        }
        
        // Mark process as terminated
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.state = ProcessState::Terminated;
            // TODO: Clean up process resources
        } else {
            return Err(ProcessError::ProcessNotFound);
        }
        
        Ok(())
    }
    
    /// Terminate a thread
    pub fn terminate_thread(&mut self, tid: ThreadId) -> Result<(), ProcessError> {
        if let Some(thread) = self.threads.iter_mut().find(|t| t.tid == tid) {
            thread.state = ThreadState::Terminated;
            // TODO: Clean up thread resources
        } else {
            return Err(ProcessError::ThreadNotFound);
        }
        
        // Remove from ready queue
        self.ready_queue.retain(|&t| t != tid);
        
        // If this was the current thread, schedule next
        if self.current_thread == Some(tid) {
            self.current_thread = None;
            self.schedule_next();
        }
        
        Ok(())
    }
    
    /// Get process by PID
    pub fn get_process(&self, pid: ProcessId) -> Option<&ProcessControlBlock> {
        self.processes.iter().find(|p| p.pid == pid)
    }
    
    /// Get thread by TID
    pub fn get_thread(&self, tid: ThreadId) -> Option<&ThreadControlBlock> {
        self.threads.iter().find(|t| t.tid == tid)
    }
    
    /// Schedule next thread to run
    pub fn schedule_next(&mut self) -> Option<ThreadId> {
        // Simple round-robin scheduler
        if let Some(next_tid) = self.ready_queue.pop_front() {
            // Verify thread is still ready
            if let Some(thread) = self.threads.iter().find(|t| t.tid == next_tid) {
                if thread.state == ThreadState::Ready {
                    self.current_thread = Some(next_tid);
                    // Put thread back at end of queue for round-robin
                    self.ready_queue.push_back(next_tid);
                    return Some(next_tid);
                }
            }
        }
        
        None
    }
    
    /// Get current running thread
    pub fn get_current_thread(&self) -> Option<ThreadId> {
        self.current_thread
    }
    
    /// Yield current thread
    pub fn yield_thread(&mut self) {
        if let Some(current) = self.current_thread {
            // Move current thread to back of ready queue
            self.ready_queue.push_back(current);
        }
        
        // Schedule next thread
        self.schedule_next();
    }
}

/// Global process manager instance
static mut PROCESS_MANAGER: Option<ProcessManager> = None;

/// Initialize process and thread management
pub fn init() {
    crate::println!("Initializing process and thread management...");
    
    let process_manager = ProcessManager::new();
    
    unsafe {
        PROCESS_MANAGER = Some(process_manager);
    }
    
    crate::println!("Process and thread management initialized");
}

/// Get the global process manager
pub fn get_process_manager() -> Option<&'static mut ProcessManager> {
    unsafe { PROCESS_MANAGER.as_mut() }
}