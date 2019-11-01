use crate::View;

#[derive(Default)]
pub struct VStack {
    pub children: Vec<crate::View>,
}

impl VStack {
    pub fn construct(&mut self, children: Option<Vec<View>>) {
        self.children = children.unwrap_or(vec![]);
    }
}
