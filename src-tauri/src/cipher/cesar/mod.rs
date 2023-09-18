use crate::math;

pub struct Cesar<'a> {
    pub message: &'a str,
    alphabet: &'a str,
}

impl<'a> Cesar<'a> {
    pub fn new(message: &'a str, alphabet: &'a str) -> Cesar<'a> {
        Cesar {
            message,
            alphabet
        }
    }

    pub fn encrypt(&self, shift: i64) -> String {
        self.message.chars().into_iter()
            .map(|c| {
                let pos: i64 = self.alphabet.chars()
                    .position(|a| a == c)
                    .unwrap()
                    .try_into()
                    .unwrap();

                let complete_shift: usize = math::cropping_modulo(pos + shift, self.alphabet.chars().count().try_into().unwrap())
                    .try_into()
                    .unwrap();

                self.alphabet.chars().nth(complete_shift).unwrap()
            })
            .collect()
    }
}
