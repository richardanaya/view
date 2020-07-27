use crate::View;

#[derive(Default)]
pub struct Button {
    pub text: String,
    pub style: u8,
    pub num_click_handlers: usize,
}

impl Button {
    pub fn on_click(&mut self, _f: impl Fn()) {
        self.num_click_handlers += 1;
    }

    pub fn construct(&mut self, _children: Vec<Box<dyn View>>) {
        // do something with image
    }
}

impl View for Button {}
