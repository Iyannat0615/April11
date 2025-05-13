use crate::memory_manager::MemoryManager;

/// Attempts to delete a memory allocation by its ID.
///
/// # Parameters
///
/// - `manager`: A mutable reference to the `MemoryManager` instance.
/// - `id`: The `usize` identifier of the memory block to delete. Internally cast to `u16`.
///
/// # Behavior
///
/// - If the specified ID exists, the corresponding memory block is cleared
///   and a success message is printed.
/// - If the ID is not found, an error message is printed.
pub fn delete(manager: &mut MemoryManager, id: usize) {
    // Attempt to delete the memory associated with the provided ID.
    // The ID is cast to u16 as the MemoryManager uses u16 for IDs.
    if manager.delete(id as u16).is_some() {
        // If deletion is successful, print a success message
        println!("Delete successful for ID {}", id);
    } else {
        // If the ID is not found, print a failure message
        println!("Delete failed for ID {}: ID not found", id);
    }
}
