use view::*;

struct VNode {
    vnode_type: &'static str,
    children: Vec<VNode>,
    classes: Vec<String>,
    text: Option<String>,
}

impl VNode {
    fn add_class(&mut self, c: &str) {
        self.classes.push(c.to_string())
    }

    fn add_view_child(&mut self, child: VNode) {
        self.children.push(child);
    }

    fn render_to_string(&self) -> String {
        if let Some(t) = &self.text {
            t.clone()
        } else {
            format!(
                "<{} class=\"{}\">{}</{}>",
                self.vnode_type,
                self.classes.join(","),
                self.children
                    .iter()
                    .map(|c| c.render_to_string())
                    .collect::<Vec<String>>()
                    .join(""),
                self.vnode_type
            )
        }
    }
}

type Div = VNode;

impl Default for Div {
    fn default() -> Self {
        VNode {
            vnode_type: "div",
            children: vec![],
            classes: vec![],
            text: None,
        }
    }
}

type Text = VNode;

impl Text {
    fn new(t: &str) -> Self {
        VNode {
            vnode_type: "text",
            children: vec![],
            classes: vec![],
            text: Some(t.to_string()),
        }
    }
}

fn main() {
    let html = view! {
      Div.add_class("greeting"){
        Text("Hello World!")
      }
    };
    println!("{}", html.render_to_string());
}
