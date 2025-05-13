use crate::memory_manager::MemoryManager; // Import the MemoryManager module

// Reads and prints the data associated with the given ID from the memory manager.
//
// Parameters:
// - `manager`: A reference to the MemoryManager instance.
// - `id`: The ID (usize) of the memory block to read.
//
// Behavior:
// - If the ID exists, the function prints the data as a UTF-8 string.
// - If the ID is not found, it prints an error message.
pub fn read(manager: &MemoryManager, id: usize) {
    // Attempt to read the data associated with the ID (cast to u16)
    if let Some(data) = manager.read(id as u16) {
        // Print the successfully read data as a UTF-8 string
        println!(
            "Read successful for ID {}: {}",
            id,
            String::from_utf8_lossy(&data)
        );
    } else {
        // Print an error message if the ID is not found
        println!("Read failed for ID {}: ID not found", id);
    }
}
