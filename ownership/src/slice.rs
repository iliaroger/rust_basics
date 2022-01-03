pub fn slice_call() {
    let new_string = "Hello world";
    let first_slice = &new_string[0..5];
    let second_slice = &new_string[6..];
    let full_slice = &new_string[..];
    println!(
        "frist slice: {}, second slice: {}, all: {}",
        first_slice, second_slice, full_slice
    );

    for (i, &item) in new_string.as_bytes().iter().enumerate() {
        if item == b'l' {
            println!("i: {}, item: {}", i, item);
        }
    }

    // b' ' is a byte literal to search for ascii characters
}
