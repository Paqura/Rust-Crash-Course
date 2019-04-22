pub fn run() {
	println!("Hello world!");
	println!("Number: {}", 1);

	// Basic formatting
	println!("{} is {}", "Slava", "Avals");

	// Basic math
	println!("10 + 10 = {}", 10 + 10);

	// Positional arguments
	println!(
		"{0} is {1}, and {0}",
		"Slava", "Avals",
	);

	// Named arguments
	println!(
		"{name} love {lang}",
		name = "Slava", lang = "Rust",
	);

	// Placeholder traits
	// По факту переводит в определенную систему исчесления
	// Выведет 1010 а 12
	println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

	// Placeholder for Debug
	println!("{:?}", (123, "Hello world", true));
}