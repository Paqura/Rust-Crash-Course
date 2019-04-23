// Вектор - изменяемый массив

use std::mem;

pub fn run() {
	let mut numbers: Vec<i32> = vec![666, 3432, 3423, 1122, 666];

	println!("{:?}", numbers);

	// Push on to vector
	numbers.push(5);
	numbers.push(16545);

	// Pop on to vector
	numbers.pop();

	//Single values as another lang
	println!("{}", numbers[0]);

	// Vector are stack occupies
	// Return 24 bytes more than Array
	println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

	// Get slice
	// let slice: &[i32] = &numbers[..2];
	// println!("{:?}", slice);

	// Loop
	for x in numbers.iter() {
		println!("{}", x);
	}

	// Loop & mutate vector
	for x in numbers.iter_mut() {
		*x *= 10;
	}

	println!("Mutate vector: {:?}", numbers);
}