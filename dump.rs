use crate::memory_manager::MemoryManager;

/// Dumps the current memory contents by calling the `dump` method on the `MemoryManager`.
///
/// # Parameters
/// - `manager`: A reference to the `MemoryManager` instance.
///
/// # Behavior
/// This function prints the current state of memory allocations managed by the `MemoryManager`.
/// It calls the `dump` method, which outputs the details of all memory blocks, including their ID,
/// starting address, size, and data.
pub fn dump(manager: &MemoryManager) {
    // Call the dump method on the MemoryManager to display memory contents
    manager.dump();
}
