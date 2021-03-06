// После передачи переменной в функцию - она перестаёт существовать в области видимости
// т.е. ей больше нельзя управлять.
// Для этого есть ссылки & - пример &var.
// Изначально ссылки иммутабельны, и изменение ссылки приведёт к ошибке,
// чтобы справиться с этим используем mut - &mut var

// Изменяемая ссылочная переменная имеет значительное ограничение: у одной переменной
// может быть только одна изменяемая ссылочная переменная в данной области видимости.

pub fn run() {
	let mut example = String::from("Example");

	//inner_scope_fn(example);

	// Получаем ошибку
	//println!("{}", example);

	// Корректная передача ссылки
	// inner_scope_fn(&example);
	// println!("{}", example);

	// Передача мутабельной ссылки
	inner_scope_fn(&mut example);
	// Print Example!
	println!("{}", example);
}

// Передача владения переменной
// fn inner_scope_fn(str: String) {
// 	str.len();
// }

// Передача ссылки
// fn inner_scope_fn(str: &String) {
// 	str.len();
// }

// Передача мутабельной ссылки
fn inner_scope_fn(str: &mut String) {
	str.push_str("!");
}
