// Variables in Rust immutable by default
// use mut for it

pub fn run() {
	let name = "Slavals";
	let mut age = 26;
	println!("My name is {}, my age is {}", name, age);
	age = 27;
	println!("My name is {}, my age is {}", name, age);

	// Define contant
	const ID: i32 = 001;
	println!("ID: {}", ID);

	//Assign multiple vars
	let (my_name, my_age) = ("Slavals", 27);
}