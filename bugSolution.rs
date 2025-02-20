fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Before removing: {:?}", vec);
    let removed = vec.remove(0); //Remove the element at index 0
    println!("Removed element: {}", removed);
    println!("After removing: {:?}", vec);
    let removed2 = vec.remove(0); //Remove the element at index 0 again
    println!("Removed element: {}", removed2);
    println!("After removing: {:?}", vec);
    // Check if the vector is empty before accessing elements
    if !vec.is_empty() {
        println!("Element at index 0: {}", vec[0]);
    } else {
        println!("Vector is empty");
    }
} 