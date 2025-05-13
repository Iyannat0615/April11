mod insert;
mod memory_manager;
mod dump;

use memory_manager::MemoryManager;
use insert::insert;
use dump::dump;

fn main() {
    // Create a new instance of the memory manager
    let mut manager = MemoryManager::new();

    // Insert data with ID 1
    let id1 = 1;
    let data1 = vec![72, 101, 108, 108, 111]; // "Hello"
    insert(&mut manager, id1, data1);

    // Insert data with ID 2
    let id2 = 2;
    let data2 = vec![82, 117, 115, 116]; // "Rust"
    insert(&mut manager, id2, data2);

    // Read and print data with ID 1
    match manager.read(id1) {
        Some(data) => println!("Read data for ID {}: {}", id1, String::from_utf8_lossy(&data)),
        None => println!("No data found for ID {}", id1),
    }

    // Update data for ID 2
    let updated_data = vec![80, 121, 116, 104, 111, 110]; // "Python"
    manager.update(id2, updated_data);

    // Read and print updated data for ID 2
    match manager.read(id2) {
        Some(data) => println!("Updated data for ID {}: {}", id2, String::from_utf8_lossy(&data)),
        None => println!("No data found for ID {}", id2),
    }

    // Delete data with ID 1
    manager.delete(id1);

    // Dump memory content
    dump(&manager);  // This will print out the memory contents
}
