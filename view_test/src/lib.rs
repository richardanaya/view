#![feature(proc_macro_hygiene)]
use view::*;

#[derive(Default)]
struct Button{
    text:String
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let v = view!{
            Header
            Footer
        };
    }
}
