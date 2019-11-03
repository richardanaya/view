mod button;
mod footer;
mod image;
mod legal;
mod vstack;

use button::Button;
use footer::Footer;
use image::Image;
use legal::Legal;
use vstack::VStack;

#[derive(Debug)]
pub enum View {
    Button(Button),
    VStack(VStack),
    Image(Image),
    Legal(Legal),
    Footer(Footer),
}

#[cfg(test)]
mod tests {
    use crate::*;
    use view::*;

    const BOLD: u8 = 0;

    #[test]
    fn basic_0() {
        let o = view! {
            Button
        };
        if let View::Button(_) = o {
        } else {
            panic!("should be a button")
        }
    }

    #[test]
    fn basic_1() {
        let o = view! {
            VStack
        };
        if let View::VStack(v) = o {
            assert_eq!(0, v.children.len());
        } else {
            panic!("should be a vstack")
        }
    }

    #[test]
    fn basic_2() {
        let o = view! {
            VStack {
                Button
            }
        };
        if let View::VStack(v) = o {
            assert_eq!(1, v.children.len());
        } else {
            panic!("should be a vstack")
        }
    }

    #[test]
    fn basic_3() {
        let o = view! {
            VStack {
                Button
                Button
            }
        };
        if let View::VStack(v) = o {
            assert_eq!(2, v.children.len());
        } else {
            panic!("should be a vstack")
        }
    }

    #[test]
    fn basic_4() {
        let o = view! {
            VStack {
                Button
                VStack {
                    Button
                    Button
                }
            }
        };
        if let View::VStack(v) = o {
            assert_eq!(2, v.children.len());
            if let View::Button(_) = &v.children[0] {
            } else {
                panic!("should be a button")
            }
            if let View::VStack(v2) = &v.children[1] {
                assert_eq!(2, v2.children.len());
            } else {
                panic!("should be a vstack")
            }
        } else {
            panic!("should be a vstack")
        }
    }

    #[test]
    fn basic_if() {
        let show_button = false;
        let o = view! {
            VStack {
                If(show_button) {
                }
            }
        };
        if let View::VStack(v) = o {
            assert_eq!(0, v.children.len());
        } else {
            panic!("should be a vstack")
        }
    }

    #[test]
    fn basic_if_2() {
        let show_button = true;
        let o = view! {
            VStack {
                If(show_button) {
                    Button
                }
            }
        };
        if let View::VStack(v) = o {
            assert_eq!(1, v.children.len());
        } else {
            panic!("should be a vstack")
        }
    }

    #[test]
    fn basic_for() {
        let o = view! {
            VStack {
                For(i in 0..10) {
                    Button
                }
            }
        };
        if let View::VStack(v) = o {
            assert_eq!(10, v.children.len());
        } else {
            panic!("should be a vstack")
        }
    }

    #[test]
    fn basic_simple() {
        let o = view! {
            Image("hey")
        };
        if let View::Image(i) = o {
            assert_eq!("hey", i.path);
        } else {
            panic!("should be a image")
        }
    }

    #[test]
    fn basic_complex() {
        let o = view! {
            Button(text:"order".to_string(),style:BOLD)
        };
        if let View::Button(b) = o {
            assert_eq!("order", b.text);
            assert_eq!(BOLD, b.style);
        } else {
            panic!("should be a button")
        }
    }

    #[test]
    fn basic_modification() {
        let o = view! {
            Button
                .on_click(Box::new(||do_order()))
        };
        if let View::Button(b) = o {
            assert_eq!(1, b.num_click_handlers);
        } else {
            panic!("should be a button")
        }
    }

    #[test]
    fn basic_modification_2() {
        let o = view! {
            Button
                .on_click(Box::new(||do_order()))
                .on_click(Box::new(||do_order()))
        };
        if let View::Button(b) = o {
            assert_eq!(2, b.num_click_handlers);
        } else {
            panic!("should be a button")
        }
    }

    fn do_order() {}

    #[test]
    fn basic_modification_3() {
        let o = view! {
            VStack {
                Button(text:"order".to_string(),style:BOLD)
                .on_click(Box::new(||do_order()))
                .on_click(Box::new(||do_order())){
                    Image("order_icon.png")
                }
            }
        };

        if let View::VStack(s) = o {
            assert_eq!(1, s.children.len());
            if let View::Button(b) = &s.children[0] {
                assert_eq!(2, b.num_click_handlers);
            } else {
                panic!("should be a button")
            }
        } else {
            panic!("should be a vstack")
        }
    }

    #[test]
    fn full() {
        let images = vec!["coffee.png", "cream.png", "sugar.png"];
        let show_legal = false;

        let o = view! {
            VStack {
                Image("company.png")
                Button(text:"order".to_string(),style:BOLD)
                .on_click(Box::new(||do_order()))
                .on_click(Box::new(||do_order())){
                    Image("order_icon.png")
                }
                For(i in images.iter()) { Image(i) }
                Footer
                If(show_legal) { Legal }
            }
        };

        if let View::VStack(s) = o {
            assert_eq!(6, s.children.len());
            if let View::Image(i) = &s.children[0] {
                assert_eq!("company.png", i.path);
            } else {
                panic!("should be a button")
            }
            if let View::Button(b) = &s.children[1] {
                assert_eq!(2, b.num_click_handlers);
            } else {
                panic!("should be a button")
            }
        } else {
            panic!("should be a vstack")
        }
    }
}
