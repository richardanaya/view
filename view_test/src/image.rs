use crate::View;

#[derive(Default)]
pub struct Image {
    pub path:String
}

impl Image {
    pub fn new(s:&str)-> Self{
        Image{path:s.to_string()}
    }
    pub fn construct(&self, _children: Option<Vec<View>>) {}
}
