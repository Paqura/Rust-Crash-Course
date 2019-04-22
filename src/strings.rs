pub fn run() {
	// Immutable
	// If to use a dash before var name - println will be ignore it
	let _hello = "Hello";

	// Mutable
	let mut hello_mut = String::from("Hello");

	// Get length
	println!("Length: {}", hello_mut.len());

	// Push char
	hello_mut.push(' ');
	hello_mut.push('W');

	// Push str
	hello_mut.push_str("orld");

	println!("{}", hello_mut);

	println!("Capacity: {}", hello_mut.capacity());
	println!("Is empty: {}", hello_mut.is_empty());
	println!("Contains 'World' {}", hello_mut.contains("World"));
	println!("Replace {}", hello_mut.replace("World", "Horld"));

	// Loop
	for word in hello_mut.split_whitespace() {
		println!("{}", word);
	}
}