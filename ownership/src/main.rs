mod slice;
fn main() {
    // moving();
    slice::slice_call();
    // let string_reference = generate_string();
}

// pub fn moving() {
//     let mut text = String::from(
//         "Data stored in the heap can only be moved once.
//         Copying only work by enabling the clone method or if the data type is a primitive",
//     );
//     let text2 = text.clone();
//     caller(&mut text);
//     println!("Number: {}", text2);
// }

// fn caller(x: &mut String) {
//     // we mutate the original string by setting it to a mutable reference
//     x.push_str("hello");
//     println!("{}", &x);
// }

// fn generate_string() -> &String {
//     let text = String::from("are you here");

//     &text
// }

/*
    - a mutable data type can only have one mutable reference.
    if a data type i.e a string literal has two it can lead to data races which inherently cause problems.
    data races occur when two references try to read and change a data type without being in sync.

    - having immutable and mutable references at the same time wont work. first the immutable references (&) have to go out of scope and then you can use the mutable references (&mut)

    - returning pointer refernces in a function that after the call is going out of scope and thus out of memorry will result in an error.


*/
