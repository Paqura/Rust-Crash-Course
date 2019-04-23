// Struct - custom data types

// Default Struct
// struct Color {
// 	red: u8,
// 	green: u8,
// 	blue: u8,
// }

// // Tuple Struct
// struct TupleColor(u8, u8, u8);

// pub fn run() {
// 	let color1 = Color {
// 		red: 255,
// 		green: 0,
// 		blue: 0,
// 	};

// 	let color2 = TupleColor(255, 255, 255);

// 	println!("Color: {} {} {}", color1.red, color1.green, color1.blue);
// 	println!("Color: {} {} {}", color2.0, color2.1, color2.2);
// }

// Person Struct
struct Person {
	first_name: String,
	last_name: String,
}

impl Person {
	fn new(f_name: &str, l_name: &str) -> Person {
		Person {
			first_name: f_name.to_string(),
			last_name: l_name.to_string(),
		}
	}

	fn full_name(&self) -> String {
		format!("{} {}", self.first_name, self.last_name)
	}

	fn set_last_name(&mut self, l_name: &str) {
		self.last_name = l_name.to_string();
	}

	// To Tuple
	fn to_tuple(self) -> (String, String) {
		(self.first_name, self.last_name)
	}
}

pub fn run() {
	let mut person = Person::new("Slava", "Avals");
	println!("{} {}", person.first_name, person.last_name);
	println!("Fullname {}", person.full_name());

	person.set_last_name("Doe");
	println!("New Fullname: {}", person.full_name());

	println!("To tuple: {:?}", person.to_tuple());
}

