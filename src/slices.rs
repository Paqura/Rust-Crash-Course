// Срезы (slices) - это ссылочный тип, не использующий владение.
// Это непрерывная коллекция упорядоченных элементов.
// Рассмотрим учебную задачу.
// Необходимо написать функцию, входным параметром которой является строка.
// Выходным значением функции является первое слово, которое будет найдено в этой строке.
// Если функция не находит разделителя слов (пробела), она возвращает эту строку.

fn first_word(s: &String) -> &str {
	// Для нахождения пробела в строке необходимо превратить в массив байтов,
	// используя метод as_bytes
	let bytes = s.as_bytes();

	// Метод iter возвращает каждый элемент коллекции.
	// Метод enumerate передаёт результаты работы метода iter в кортеж.
	// Первый элемент этого кортежа возвращает индекс, второй элемент - ссылку на элемент.
	for (i, &item) in bytes.iter().enumerate() {
		//Нам надо найти байт, который представляет собой значение пробела.
		// Для этого мы приводим символьную константу ' ' к типу байт b' '
			if item == b' ' {
					return &s[0..i];
			}
	}

	&s[..]
}

pub fn run() {
	let word = String::from("Hello world!");
	let index = first_word(&word);
	// Return 'Hello'
	println!("{}", index);
}