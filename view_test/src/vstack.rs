use crate::View;

#[derive(Default)]
pub struct VStack {
    pub children: Vec<Box<dyn crate::View>>,
}

impl VStack {
    pub fn construct(&mut self, children: Vec<Box<dyn View>>) {
        self.children = children;
    }
}

impl View for VStack {}
