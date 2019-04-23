// Массив - фиксированный список элементов

use std::mem;

pub fn run() {
	let numbers: [i32; 5] = [666, 3432, 3423, 1122, 666];

	println!("{:?}", numbers);

	//Single values as another lang
	println!("{}", numbers[0]);

	// Arrays are stack occupies
	// Return 20 bytes
	println!("Array occupies {} bytes", mem::size_of_val(&numbers));

	// Get slice
	let slice: &[i32] = &numbers[..2];
	println!("{:?}", slice);
}