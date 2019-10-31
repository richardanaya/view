#![no_std]
extern crate alloc;
use view::*;

#[derive(Default)]
struct Button{}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn basic_0() {
        let _v = view!{
            Button
        };
    }
}
