use super::View;

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

    pub fn add_view_child<'a, T>(&'a mut self, _child: T)
    where
        T: 'static + View,
    {
        //self.children.push(t);
    }
}

impl View for Button {}
