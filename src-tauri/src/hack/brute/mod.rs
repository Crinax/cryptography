use std::marker::PhantomData;

use crate::cipher::{Cipher, cesar::Cesar};
use crate::alphabets::Alphabet;

pub struct BruteForce<'a, T: Cipher, A: Alphabet> {
    cipher: &'a T,
    alphabet_kind: PhantomData<A>
}

impl<'a, T: Cipher, A: Alphabet> BruteForce<'a, T, A> {
    pub fn new(cipher: &'a T) -> BruteForce<'a, T, A> {
        BruteForce { cipher, alphabet_kind: PhantomData }
    }
}

impl<'a, A: Alphabet> BruteForce<'a, Cesar<'a>, A> {
    pub fn decrypt(&self) -> Vec<String> {
        let mut result = Vec::new();

        for i in 0..A::get_alphabet().chars().count() {
            result.push(self.cipher.encrypt(i as i64, true));
        }

        result
    }
}
