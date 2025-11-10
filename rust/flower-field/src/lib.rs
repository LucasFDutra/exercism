use std::vec;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let garden_height = garden.len();
    let garden_width = if garden_height > 0 {
        garden[0].len()
    } else {
        0
    };

    let garden_chars: Vec<Vec<char>> = garden.iter().map(|row| row.chars().collect()).collect();
    let mut new_garden: Vec<Vec<char>> = vec![vec![' '; garden_width]; garden_height];

    for i in 0..garden_height {
        for j in 0..garden_width {
            if garden_chars[i][j] == '*' {
                new_garden[i][j] = '*';
            } else {
                let mut flower_count = 0;
                if j > 0 && garden_chars[i][j - 1] == '*' {
                    flower_count += 1;
                }
                if j < garden_width - 1 && garden_chars[i][j + 1] == '*' {
                    flower_count += 1;
                }
                if i > 0 && garden_chars[i - 1][j] == '*' {
                    flower_count += 1;
                }
                if i < garden_height - 1 && garden_chars[i + 1][j] == '*' {
                    flower_count += 1;
                }
                if i > 0 && j > 0 && garden_chars[i - 1][j - 1] == '*' {
                    flower_count += 1;
                }
                if i > 0 && j < garden_width - 1 && garden_chars[i - 1][j + 1] == '*' {
                    flower_count += 1;
                }
                if i < garden_height - 1 && j > 0 && garden_chars[i + 1][j - 1] == '*' {
                    flower_count += 1;
                }
                if i < garden_height - 1
                    && j < garden_width - 1
                    && garden_chars[i + 1][j + 1] == '*'
                {
                    flower_count += 1;
                }

                if flower_count > 0 {
                    new_garden[i][j] = std::char::from_digit(flower_count, 10).unwrap();
                }
            }
        }
    }
    return new_garden.iter().map(|row| row.iter().collect()).collect();
}
