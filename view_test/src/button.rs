use crate::View;

#[derive(Default)]
pub struct Button {
    pub text:String,
    pub style:u8
}

impl Button {
    pub fn construct(&self, _children: Option<Vec<View>>) {}
}
