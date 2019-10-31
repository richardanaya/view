#[derive(Default)]
struct Button{}

#[cfg(test)]
mod tests {
    use view::*;
    use crate::*;

    #[test]
    fn basic_0() {
        let _v = view!{
            Button
        };
    }
}
