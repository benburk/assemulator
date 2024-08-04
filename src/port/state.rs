use std::collections::VecDeque;

use super::{
    util::{input, read_int},
    Port,
};

#[derive(Default)]
pub struct State<T> {
    /// Buffer of chars read from stdin.
    chars: VecDeque<u8>,
    phantom: std::marker::PhantomData<T>,
}

impl<T> State<T>
where
    T: std::fmt::Display + Copy,
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
    T: From<u8> + TryFrom<u64>,
{
    pub fn read_port(&mut self, port: Port) -> T {
        match port {
            Port::Char => {
                // if there are chars in the buffer, read from it
                // otherwise read from stdin
                if self.chars.is_empty() {
                    self.chars.extend(input("> ").as_bytes());
                }
                T::from(self.chars.pop_front().unwrap_or_default())
            }
            Port::Ticker => read_int::<T>(),
        }
    }

    pub fn write_port(&mut self, port: Port, value: T) {
        match port {
            // print char to screen
            Port::Char => {
                print!("{}", char::from(u8::try_from(value).unwrap()));
            }
            // print unsigned integer to screen
            Port::Ticker => println!("{value}"),
        }
    }
}
