use crate::memory_manager::MemoryManager; // Import the MemoryManager module

// Updates an existing block of memory with new data for the given ID.
//
// Parameters:
// - `manager`: A mutable reference to the MemoryManager instance.
// - `id`: The ID (usize) of the memory block to update.
// - `data`: The new data (Vec<u8>) to write into the memory block.
//
// Behavior:
// - If the ID exists and the new data fits within the originally allocated size,
//   the memory is updated and a success message is printed.
// - If the ID is not found or the new data is too large, a failure message is printed.
pub fn update(manager: &mut MemoryManager, id: usize, data: Vec<u8>) {
    // Attempt to update the memory block with the new data
    if manager.update(id as u16, data).is_some() {
        // Update was successful
        println!("Update successful for ID {}", id);
    } else {
        // Update failed due to size overflow or missing ID
        println!("Update failed for ID {}: Not enough space or ID not found", id);
    }
}
