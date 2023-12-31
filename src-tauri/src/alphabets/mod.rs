use std::collections::HashMap;

pub trait Alphabet {
    fn get_alphabet() -> String;
}

pub trait FrequencyOfSymbols {
    fn get_alphabet_frequency() -> HashMap<char, f64>;
}

pub struct RussianAlphabet();

impl Alphabet for RussianAlphabet {
    fn get_alphabet() -> String {
        String::from("абвгдеёжзийклмнопрстуфхцчшщъыьэюя")
    }
}

impl FrequencyOfSymbols for RussianAlphabet {
    fn get_alphabet_frequency() -> HashMap<char, f64> {
        HashMap::from([
            ('о', 10.983),
            ('е', 8.483),
            ('а', 7.998),
            ('и', 7.367),
            ('н', 6.7),
            ('т', 6.318),
            ('с', 5.473),
            ('р', 4.746),
            ('в', 4.533),
            ('л', 4.343),
            ('к', 3.486),
            ('м', 3.203),
            ('д', 2.977),
            ('п', 2.804),
            ('у', 2.615),
            ('я', 2.001),
            ('ы', 1.898),
            ('ь', 1.735),
            ('г', 1.687),
            ('з', 1.641),
            ('б', 1.592),
            ('ч', 1.45),
            ('й', 1.208),
            ('х', 0.966),
            ('ж', 0.94),
            ('ш', 0.718),
            ('ю', 0.638),
            ('ц', 0.486),
            ('щ', 0.361),
            ('э', 0.331),
            ('ф', 0.267),
            ('ъ', 0.037),
            ('ё', 0.013),
        ])
    }
}
