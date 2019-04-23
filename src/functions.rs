pub fn run() {
	greeting("Hello world");

	// Closure
	let n3: i32 = 10;
	// Plus outside var n3
	let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
	println!("Closure sum: {}", add_nums(2, 3));
}

fn greeting(str: &str) {
	println!("{}", str);
}