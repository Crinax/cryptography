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

    pub fn encrypt(&self, shift: i64, ignore_unregistered: bool) -> String {
        self.message.chars().into_iter()
            .map(|c| {
                let pos: Option<usize> = self.alphabet.chars().position(|a| a == c);

                if let None = pos {
                    if !ignore_unregistered {
                        panic!("Symbol does not exists in alphabet");
                    }

                    return c
                }

                let pos: i64 = pos.unwrap().try_into().unwrap();

                let complete_shift: usize = math::cropping_modulo(pos + shift, self.alphabet.chars().count().try_into().unwrap())
                    .try_into()
                    .unwrap();

                self.alphabet.chars().nth(complete_shift).unwrap()
            })
            .collect()
    }
}
