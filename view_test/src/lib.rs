use view::*;

mod button;
mod vstack;

use button::Button;
use vstack::VStack;

pub enum View {
    Button(Button),
    VStack(VStack)
}
    
#[test]
fn basic_0() {
    let _v = view!{
        Button
    };
}

#[test]
fn basic_1() {
    let v = view!{
        VStack
    };
    assert_eq!(0,v.children.len());
}

#[test]
fn basic_2() {
    let v = view!{
        VStack {
            Button
        }
    };
    assert_eq!(1,v.children.len());
}

#[test]
fn basic_3() {
    let v = view!{
        VStack {
            Button
            Button
        }
    };
    assert_eq!(2,v.children.len());
}

#[test]
fn basic_4() {
    let v = view!{
        VStack {
            Button
            VStack {
                Button
                Button
            }
        }
    };
    assert_eq!(2,v.children.len());
}
