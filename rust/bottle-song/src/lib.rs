fn number_to_words(n: u32, capitalize: bool) -> String {
    match n {
        0 => {
            if capitalize {
                "No".to_string()
            } else {
                "no".to_string()
            }
        }
        1 => {
            if capitalize {
                "One".to_string()
            } else {
                "one".to_string()
            }
        }
        2 => {
            if capitalize {
                "Two".to_string()
            } else {
                "two".to_string()
            }
        }
        3 => {
            if capitalize {
                "Three".to_string()
            } else {
                "three".to_string()
            }
        }
        4 => {
            if capitalize {
                "Four".to_string()
            } else {
                "four".to_string()
            }
        }
        5 => {
            if capitalize {
                "Five".to_string()
            } else {
                "five".to_string()
            }
        }
        6 => {
            if capitalize {
                "Six".to_string()
            } else {
                "six".to_string()
            }
        }
        7 => {
            if capitalize {
                "Seven".to_string()
            } else {
                "seven".to_string()
            }
        }
        8 => {
            if capitalize {
                "Eight".to_string()
            } else {
                "eight".to_string()
            }
        }
        9 => {
            if capitalize {
                "Nine".to_string()
            } else {
                "nine".to_string()
            }
        }
        10 => {
            if capitalize {
                "Ten".to_string()
            } else {
                "ten".to_string()
            }
        }
        _ => n.to_string(),
    }
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut verses = Vec::new();

    for i in 0..take_down {
        let bottles = number_to_words(start_bottles - i, true);
        let next_bottles = number_to_words(start_bottles - i - 1, false);
        let bottles_word = if start_bottles - i == 1 {
            "bottle"
        } else {
            "bottles"
        };
        let next_bottles_word = if start_bottles - i - 1 == 1 {
            "bottle"
        } else {
            "bottles"
        };

        let verse = format!(
            "{bottles} green {bottles_word} hanging on the wall,\n\
            {bottles} green {bottles_word} hanging on the wall,\n\
            And if one green bottle should accidentally fall,\n\
            There'll be {next_bottles} green {next_bottles_word} hanging on the wall."
        );
        verses.push(verse);
    }

    let verses_ = verses.join("\n\n");
    println!("{}", verses_);
    return verses_;
}
