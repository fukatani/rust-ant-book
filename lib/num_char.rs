fn char_to_digit(ch: char) -> usize {
    ch.to_digit(36).unwrap() as usize - 10
}

fn digit_to_char(num: usize) -> char {
    std::char::from_digit(num as u32 + 10, 36).unwrap()
}

fn print_array<T: std::string::ToString>(array: &Vec<T>) {
    println!(
        "{}",
        array
            .iter()
            .map(|ref x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn main() {
    assert_eq!(char_to_digit('a'), 0);
    assert_eq!(char_to_digit('b'), 1);
    assert_eq!(char_to_digit('z'), 25);

    assert_eq!(digit_to_char(0), 'a');
    assert_eq!(digit_to_char(1), 'b');
    assert_eq!(digit_to_char(25), 'z');

    print_array(&vec![0, 11, 3]);
}
