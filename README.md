# 🏗️ View

```toml
[dependencies]
view = "0.4"
```
Constructing view heirarchies in Rust is a bit tedious. This is a macro for constructing views in a non framework specific manner. It's more struct oriented compared to a technology like JSX and borrows in broad strokes some ideas from SwiftUI.

This example shows everything that's possible
```rust
let images = vec!["coffee.png","cream.png","sugar.png"];
let show_coupon = false;
let coupon = getTodaysCoupon();
​
let v = view!{
  VStack {
    Image("company.png") 
    Button(text:"order".to_string(),style:BOLD)
      .on_click(|| do_order()) { 
        Image("order_icon.png") 
      }
    For(i in images.iter()) { Image(i) }
    If(show_coupon) { coupon }
    Legal
  }
};
```

<p align="center">
<img src="ui.png" float=right>
</p>

Below is all the code this macro saves you from writing yourself.

```rust
let images = vec!["coffee.png", "cream.png", "sugar.png"];
let show_legal = false;

let s = {
  let mut o = VStack {
      ..Default::default()
  };
  o.add_view_child({
      let mut o = Image::new("company.png");
      o
  });
  o.add_view_child({
      let mut o = Button {
          text: "order".to_string(),
          style: BOLD,
          ..Default::default()
      };
      o.on_click(|| do_order());
      o.on_click(|| do_order());
      o.add_view_child({
          let mut o = Image::new("order_icon.png");
          o
      });
      o
  });
  for i in images.iter() {
      o.add_view_child({
          let mut o = Image::new(i);
          o
      });
  }
  o.add_view_child({
      let mut o = Footer {
          ..Default::default()
      };
      o
  });
  if show_legal {
      o.add_view_child({
          let mut o = Legal {
              ..Default::default()
          };
          o
      });
  }
  o
};
```

This project isn't framework specific, but it does have a few rules:
* views that have children must have a function `add_view_child` implemented 
* views must implement Default trait for property construction (e.g `Button(text:"click me".to_string())` )
* views must have a 'new' constructor function for simple construction (e.g `Button("click me")` )

Here's a basic example of implementing these rules, though they can be implemented in any way you choose with any trait you like.

```rust

trait View {}

#[derive(Default)]
struct VStack {
  direction: u8,
  children: Vec<Box<View>>
}

impl VStack {
  fn new(direction:u8) -> Self {
    VStack{ direction:direction, children:vec![] }
  }
  
  fn add_view_child<'a, T>(&'a mut self, child: Box<T>)
  where
      T: 'static + View,
  {
      self.children.push(child);
  }
}

impl View for VStack {}

#[derive(Default)]
struct Button {
  text:String
}

impl Button {
  fn new(text:String) -> Self {
    Button{text:text}
  }
}

impl View for Button {}
```

# Let's make a virtual dom

```rust
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
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in view by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
