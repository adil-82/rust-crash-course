// Reference Pointers = Point to a resource in memory

pub fn run() {
    println!("**************k_pointer_ref.rs***************");
    // Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;
    println!("Values of array: {:?}", ( arr2, arr1 ));

    // With non-primitives, if you assign anotheer variable to a piece of data, 
    // the first variable will no longer hold that value. 
    // You'll need to use a Reference (&) to point to the resource.

    // Vector:
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Values of Vec: {:?}", (&vec1, vec2));

    
}
