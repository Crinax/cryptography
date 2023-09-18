use self::cesar::Cesar;

pub mod cesar;

pub trait Cipher {
    fn get_message(&self) -> String;
}

impl<'a> Cipher for Cesar<'a> {
    fn get_message(&self) -> String {
        self.message.into()
    }
}
