mod button;
mod footer;
mod image;
mod legal;
mod vstack;

pub use button::Button;
pub use footer::Footer;
pub use image::Image;
pub use legal::Legal;
pub use vstack::VStack;

pub trait View {}

#[cfg(test)]
mod tests {
    use crate::*;
    use view::*;

    const BOLD: u8 = 0;

    #[test]
    fn basic_0() {
        let o: Button = view! {
            Button
        };
        assert_eq!(0, o.style);
    }

    #[test]
    fn basic_1() {
        let v: VStack = view! {
            VStack
        };
        assert_eq!(0, v.children.len());
    }

    #[test]
    fn basic_2() {
        let v = view! {
            VStack {
                Button
            }
        };
        assert_eq!(1, v.children.len());
    }

    #[test]
    fn basic_3() {
        let v: VStack = view! {
            VStack {
                Button
                Button
            }
        };
        assert_eq!(2, v.children.len());
    }

    #[test]
    fn basic_4() {
        let v: VStack = view! {
            VStack {
                Button
                VStack {
                    Button
                    Button
                }
            }
        };
        assert_eq!(2, v.children.len());
    }


    #[test]
    fn basic_if_empty() {
        let v: VStack = view! {
            VStack {
                If(false) {
                    
                }
            }
        };
        assert_eq!(0, v.children.len());
    }


    #[test]
    fn basic_if() {
        let show_button = false;
        let v: VStack = view! {
            VStack {
                If(show_button) {
                    Button
                }
            }
        };
        assert_eq!(0, v.children.len());
    }

    #[test]
    fn basic_if_2() {
        let show_button = true;
        let v = view! {
            VStack {
                If(show_button) {
                    Button
                }
            }
        };
        assert_eq!(1, v.children.len());
    }

    #[test]
    fn basic_for() {
        let v = view! {
            VStack {
                For(i in 0..10) {
                    Button
                }
            }
        };
        assert_eq!(10, v.children.len());
    }

    #[test]
    fn basic_simple() {
        let i = view! {
            Image("hey")
        };
        assert_eq!("hey", i.path);
    }

    #[test]
    fn basic_complex() {
        let b = view! {
            Button(text:"order".to_string(),style:BOLD)
        };

        assert_eq!("order", b.text);
        assert_eq!(BOLD, b.style);
    }

    #[test]
    fn basic_modification() {
        let b = view! {
            Button
                .on_click(||do_order())
        };
        assert_eq!(1, b.num_click_handlers);
    }

    fn do_order() {}

    #[test]
    fn basic_modification_2() {
        let o = view! {
            Button
                .on_click(||do_order())
                .on_click(||do_order())
        };
        assert_eq!(2, o.num_click_handlers);
    }

    #[test]
    fn basic_modification_3() {
        let o = view! {
            VStack {
                Button(text:"order".to_string(),style:BOLD)
                .on_click(||do_order())
                .on_click(||do_order()){
                    Image("order_icon.png")
                }
            }
        };

        assert_eq!(1, o.children.len());
    }

    #[test]
    fn full() {
        let images = vec!["coffee.png", "cream.png", "sugar.png"];
        let show_legal = false;

        let s = view! {
            VStack {
                Image("company.png")
                Button(text:"order".to_string(),style:BOLD)
                .on_click(||do_order())
                .on_click(||do_order()){
                    Image("order_icon.png")
                }
                For(i in images.iter()) { Image(i) }
                Footer
                If(show_legal) { Legal }
            }
        };

        assert_eq!(6, s.children.len());
    }
}
