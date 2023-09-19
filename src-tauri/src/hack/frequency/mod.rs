use std::collections::HashMap;
use std::marker::PhantomData;

use crate::cipher::{Cipher, cesar::Cesar};
use crate::alphabets::{Alphabet, RussianAlphabet, FrequencyOfSymbols};

pub struct FrequencyAnalysis<'a, T: Cipher, A: Alphabet> {
    cipher: &'a T,
    alphabet_kind: PhantomData<A>
}

impl<'a, T: Cipher, A: Alphabet> FrequencyAnalysis<'a, T, A> {
    pub fn new(cipher: &'a T) -> FrequencyAnalysis<'a, T, A> {
        FrequencyAnalysis { cipher, alphabet_kind: PhantomData }
    }
}

impl<'a> FrequencyAnalysis<'a, Cesar<'a>, RussianAlphabet> {
    pub fn decrypt(&self) -> String {
        let alphabet = RussianAlphabet::get_alphabet();
        let alphabet_freq = RussianAlphabet::get_alphabet_frequency();
        let message = self.cipher.get_message();
        let filtered_message = message.chars()
            .filter(|&c| {
                let lower_case = c.to_lowercase().next().unwrap();

                alphabet.contains(c) || alphabet.contains(lower_case)
            });
        let message_count_symbols = filtered_message.clone().count() as f64;
        let message_symbol_freq: HashMap<char, f64> = filtered_message
            .map(|c| {
                let counts = message.chars().filter(|cm| *cm == c).count() as f64;

                (c, counts / message_count_symbols * 100.0)
            })
            .collect();

        let most_freq_symbol = message_symbol_freq.into_iter()
            .reduce(|a, b| if f64::max(a.1, b.1) == b.1 { b } else { a })
            .unwrap();

        let nearest_symbol = alphabet_freq.into_iter()
            .map(|(key, value)| (key, (value - most_freq_symbol.1).abs()))
            .reduce(|a, b| if f64::min(a.1, b.1) == b.1 { b } else { a })
            .unwrap();

        let most_freq_pos = alphabet.chars().position(|c| c == most_freq_symbol.0).unwrap();
        let nearest_position = alphabet.chars().position(|c| c == nearest_symbol.0).unwrap();

        let shift = (nearest_position as i64) - (most_freq_pos as i64);

        self.cipher.encrypt(shift, true)
    }
}
