

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' |
                'A' | 'E' | 'I' | 'O' | 'U')
}

fn piggyfy(mut word :String) -> String {
    if let Some(first) = word.chars().next() {
        if is_vowel(first) {
            word.push_str("hay")
        } else {
            word.remove(0);
            word.push(first.to_lowercase().next().unwrap());
            word.push_str("ay");
        }
    }
    word
}

pub fn main() {
    let text = String::from("Apple Banana Happiness Oblivion Test Hour Amphibian ");

    let mut result = String::new();
    for word in text.split_whitespace() {
        let word = piggyfy(word.to_string());
        result.push_str(&word);
        result.push(' ');
    }
    println!("{}", result);
}