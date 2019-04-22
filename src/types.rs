pub fn run() {
	// Default is "i32"
	let x = 1;

	// Default is "f64"
	let y = 2.5;

	// Add explicit type
	let w: i32 = 123123;
	let z: i64 = 213121233123;

	// Find max size
	println!("Max i32: {}", std::i32::MAX);
	println!("Max i32: {}", std::i64::MAX);

	// Boolean
	let is_active: bool = true;
	let is_greater: bool = 10 > 5;

	// Char
	// Use single quote
	let char1 = 'a';
	let face = '\u{1F600}';

	println!("{:?}", (x, y, w, z, is_active, is_greater, char1, face));
}