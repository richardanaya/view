use super::View;

#[derive(Default)]
pub struct VStack {
    pub children: Vec<Box<dyn crate::View>>,
}

impl VStack {
    pub fn add_view_child<'a, T>(&'a mut self, child: T)
    where
        T: 'static + View,
    {
        self.children.push(Box::new(child));
    }
}

impl View for VStack {}
