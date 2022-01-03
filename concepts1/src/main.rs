// fn shadowing() {
//     let x = 5;
//     println!(
//         "You can use a variable name twice. Still preserving mutability: {}",
//         &x
//     );
//     let x = "five";
//     println!("Mutability is still preserved in both cases: {}", &x);
// }

// fn tupel() {
//     let data_entries = ("robert", 43);
//     let (name, _) = data_entries;
//     println!("Entry name: {}", (name));
// }

// fn array_example() {
//     let numbers = [2, 4, 5];
//     let new_array = [2; 6];
//     println!("Number: {}", numbers[2]);
//     println!("Other array: {}", new_array[4]);
// }

// fn basic_function(x: f32, y: f32) -> f32 {
//     println!("function naming follow the snake case principle");
//     x + y
// }

// fn fruity_loops() {
//     let mut counter = 0;
//     let random_integers = [2, 4, 534, 234, 1235, 543234];

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter;
//         }
//     };

//     for num in random_integers.iter() {
//         println!("Number: {}", num)
//     }

//     for num in 1..9 {
//         println!("{}", num);
//     }
// }

fn main() {}
