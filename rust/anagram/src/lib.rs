fn same_word(source: &str, input: &str) -> bool {
    source.to_lowercase() == input.to_lowercase()
}

fn same_letters(source: &str, input: &str) -> bool {
    let mut input_letters = input.to_lowercase().chars().collect::<Vec<_>>();
    let mut source_letters = source.to_lowercase().chars().collect::<Vec<_>>();

    input_letters.sort();
    source_letters.sort();
    input_letters == source_letters
}


pub fn anagrams_for<'a>(word: &'a str, inputs: &'a [&str]) -> Vec<&'a str> {
    let outputs = inputs.iter()
                        .filter(|&input| !same_word(word, input))
                        .filter_map(|&input| {
                            if same_letters(word, input) {
                                Some(input)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
    outputs
}
