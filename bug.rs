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
    // Try to access the element at index 0 after removing all elements. This will panic
    println!("Element at index 0: {}", vec[0]);
}