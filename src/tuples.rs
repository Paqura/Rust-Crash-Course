pub fn run() {
	let person: (&str, &str, i8) = ("Slava", "Avals", 27);

	println!("{}, {}, {}", person.0, person.1, person.2);
	println!("{}", i8::max_value());
}