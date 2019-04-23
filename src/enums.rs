enum Movement {
	Up,
	Down,
	Left,
	Right,
}

fn move_avatar(m: Movement) {
	// Perform action dependion on info
	match m {
		Movement::Up => println!("Moving up"),
		Movement::Down => println!("Moving down"),
		Movement::Left => println!("Moving left"),
		Movement::Right => println!("Moving right"),
	}
}

pub fn run() {
	move_avatar(Movement::Up);
	move_avatar(Movement::Down);
	move_avatar(Movement::Left);
	move_avatar(Movement::Right);
}