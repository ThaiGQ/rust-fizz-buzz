fn fizz_buz() {
    let mut fizz_buzz_count: i32 = 0;

    for num in 1..=300 {
        let result: Option<String> = match_number(num);

        if result.is_some() {
            let word: String = result.unwrap();

            println!("{}", word);

            if word.eq("fizz buzz") {
                fizz_buzz_count += 1
            }
        }
    }

    println!("fizz buzz was printed {} times", fizz_buzz_count)
}

fn match_number(number: i32) -> Option<String> {
    let multiple_of_3: bool = number % 3 == 0;
    let multiple_of_5: bool = number % 5 == 0;
    let mut word: String = String::new();

    if multiple_of_3 && multiple_of_5 {
        word.push_str("fizz buzz");
        Some(word)
    } else if multiple_of_3 {
        word.push_str("fizz");
        Some(word)
    } else if multiple_of_5 {
        word.push_str("buzz");
        Some(word)
    } else {
        None
    }
}

fn main() {
    println!("Welcome to fizz buzz!");
    fizz_buz()
}
