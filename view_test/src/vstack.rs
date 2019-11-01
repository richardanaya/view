use crate::View;

#[derive(Default)]
pub struct VStack {
    pub children: Vec<crate::View>,
}

impl VStack {
    pub fn construct(&self, _children: Option<Vec<View>>) {}
}
