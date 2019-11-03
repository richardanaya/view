use crate::View;

#[derive(Default)]
pub struct Button {
    pub text: String,
    pub style: u8,
    pub has_click_handler: bool,
}

impl Button {
    pub fn construct(&self, _children: Option<Vec<View>>) {}
    pub fn on_click(&mut self, _f: Box<dyn Fn() -> ()>) {
        self.has_click_handler = true;
    }
}
