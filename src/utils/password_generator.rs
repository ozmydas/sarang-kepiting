use fake::{
    faker::lorem::raw::Word,
    locales::{DE_DE, EN, FR_FR},
    Fake,
};
use rand::Rng;
use regex::Regex;

pub fn generate_password(
    over_12: bool,
    use_nonenglish: bool,
    use_specialchar: bool,
    use_uppercase: bool,
    use_number: bool,
) -> String {
    let mut rng = rand::rng();

    let mut output = generator(
        if over_12 {
            rng.random_range(14..=20)
        } else {
            rng.random_range(8..=12)
        },
        use_specialchar,
        use_nonenglish,
    );

    if !use_number {
        let re = Regex::new(r"[\d\s]+").unwrap();
        output = re.replace_all(&output, "").to_string();
    }

    if !use_uppercase {
        output = output.to_lowercase();
    }

    output.replace(" ", "")
} // end func

/*********/

fn generator(max_length: usize, use_specialchar: bool, use_nonenglish: bool) -> String {
    let mut generated_password = String::new();

    loop {
        let kata = decider(use_nonenglish);
        generated_password.push_str(&kata);

        if !use_specialchar {
            let re = Regex::new(r"[^a-zA-Z0-9\s]").unwrap();
            generated_password = re.replace_all(&generated_password, "").to_string();
        }

        if generated_password.len() >= max_length {
            break;
        }
    }

    generated_password
} // end func

fn decider(use_nonenglish: bool) -> String {
    let mut rng = rand::rng();
    let random_index = rng.random_range(0..=8);
    let charuniq = ["!", "@", "#", "$", "%", "&", "?"];
    let uppercase = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];

    match random_index {
        0 => {
            if use_nonenglish {
                Word(DE_DE).fake()
            } else {
                Word(EN).fake()
            }
        }
        1 => {
            if use_nonenglish {
                Word(FR_FR).fake()
            } else {
                Word(EN).fake()
            }
        }
        2 | 3 => rng.random_range(0..1000).to_string(),
        4 | 5 => uppercase[rng.random_range(0..uppercase.len())].to_string(),
        _ => charuniq[rng.random_range(0..charuniq.len())].to_string(),
    }
} // end func
