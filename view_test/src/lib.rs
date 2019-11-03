#![feature(proc_macro_hygiene)]
use view::*;

mod button;
mod vstack;
mod image;
mod footer;
mod legal;

use button::Button;
use vstack::VStack;
use image::Image;
use legal::Legal;
use footer::Footer;


pub enum View {
    Button(Button),
    VStack(VStack),
    Image(Image),
    Legal(Legal),
    Footer(Footer),
}

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
    let show_button = true;
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
fn full() {
    let images = vec!["coffee.png","cream.png","sugar.png"];
    let show_legal = false;
    /*let v = view!{
        VStack {
            Image("company.png") 
            Button(text:"order".to_string(),style:BOLD)
            .on_click(|x|{ do_order() }) { 
                Image("order_icon.png") 
            }
            For(i in images.iter()) { Image(i) }
            Footer
            If(show_legal) { Legal }
        }
    }; */ 
             

    let _v = view!{
        VStack {
            Image("company.png")
            For(i in images.iter()) { Image(i) }
            Footer
            If(show_legal) { Legal }
        }
    }; 
}
