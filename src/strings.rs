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

	// With capacity
	let mut str_with_capacity = String::with_capacity(10);
	str_with_capacity.push_str("game of thrones");

	// Nothing happen
	assert_eq!(15, str_with_capacity.len());

	// Panic capacity == 20
	//assert_eq!(10, str_with_capacity.capacity());

	// Variable removed after assignment another variable
	let s1 = String::from("Hello");
	let s2 = s1;
	// Error
	println!("{}", s1);
	println!("{}", s2);

	// Method clone - bad perfomance, but solve the problem
	let s_1 = String::from("Hello");
	let s_2 = s1.clone();
	// No error
	println!("{}", s1);
	println!("{}", s2);

}