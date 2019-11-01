# 🏗️ View

```toml
[dependencies]
view = "0.0"
```

Constructing view heirarchies in Rust is a bit tedious. This is a macro for constructing views that is non framework specific. It's more struct oriented compared to a technology like JSX and borrows in broad strokes some ideas from SwiftUI.

This example shows everything that's possible

```rust
let images = vec!["coffee.png","cream.png","sugar.png"];
let v = view!{
  VStack {
    Image("company.png") 
    Button(text:"order".to_owned(),style:BOLD)
      .on_click(|x|{ do_order() }) { 
        Image("order_icon.png") 
      }
    For(i in images.iter()) { Image(i) }
    Footer
    If(show_legal) { Legal }
  }
};
```

Below is all the code this macro saves you from writing yourself.

```rust
let images = vec!["coffee.png","cream.png","sugar.png"];
let v = { 
  let o = VStack { ..Default::default() }
  o.construct(Some({
    let mut children = vec![];
    children.push({
      let o = Image::new("company.png");
      o.construct(None)
      View::Image(o)
    });
    children.push({
      let o = Button { text: "a".to_owned(), style: BOLD, ..Default::default() };
      o.on_click(|x|{ do_order() });
      o.construct(Some({
        let mut children = vec![];
        children.push({
          let o = Image::new("order_icon.png");
          o.construct(None)
          View::Image(o)
        });
        children
      }));
      View::Button(o)
    });
    for i in images.iter() {
      children.push({
        let o = Image::new(i);
        o.construct(None)
        View::Image(o)
      });
    }
    children.push({
      let o = Footer{ ..Default::default() };
      o.construct(None)
      View::Footer(o)
    });
    if show_legal {
      children.push({
        let o = Legal{ ..Default::default() };
        o.construct(None)
        View::Legal(o)
      });
    }
    Some(children)
  }));
  View::VStack(o)
};
```

This project isn't framework specific, but it does have a few rules:
* you must specify an enum `View` that contains all views (this is so view containers can efficiently hold a variety of views)
* views must have a function `fn construct(&mut self, children:Option<Vec<View>>)` implemented somehow
* views must implement Default trait for property construction (e.g `Button(text:"click me".to_owned())` )
* views must have a 'new' constructor function for simple construction (e.g `Button("click me")` )

Here's a basic example of implementing these rules, though they can be implemented in any way you choose.

```rust

enum View {
  VStack(VStack)
  Button(Button)
}

#[derive(Default)]
struct VStack {
  direction: u8,
  children: Vec<View>
}

impl VStack {
  fn new(direction:u8) -> Self {
    VStack{ direction:direction, children:vec![] }
  }
  
  fn construct(&mut self, children:Option<Vec<View>>) { 
    self.children = children.unwrap();
  }
}

#[derive(Default)]
struct Button {
  text:String
}

impl Button {
  fn new(text:String) -> Self {
    Button{text:text}
  }
  
  fn construct(&mut self, children:Option<Vec<View>>) {}
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
