# view

```toml
[dependencies]
view = "0.0"
```

A macro for constructing views that is non framework specific.

```rust
let images = vec!["coffee.png","cream.png","sugar.png"];
let v = view!{
  VStack(direction:TOP_TO_BOTTOM) {
    Image("company.png") 
    Button(text:"order".to_owned(),style:BOLD)
      .on_click(|x|{ do_order() }) { 
        Image("order_icon.png") 
      }
    ( Image::from_names(images) ) 
    Footer
  }
};
```

will translate to

```rust
let images = vec!["coffee.png","cream.png","sugar.png"];
let v = { 
  let o = VStack { direction: TOP_TO_BOTTOM, ..Default::default() }
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
    for i in Image::from_names(images).into_iter() {
      children.push(i)
    }
    children.push({
      let o = Footer{ ..Default::default() };
      o.construct(None)
      View::Footer(o)
    });
    Some(children)
  }));
  View::VStack(o)
};
```

This project really isn't framework specific, but it does have certain rules:
* you must specify an enum `View` that contains all views (this is so view containers can efficiently hold a variety of views)
* views must have a function `fn construct(&mut self, children:Option<Vec<View>>)` implemented
* views must implement Default trait for property construction (e.g `Button(text:"click me".to_owned())` )
* views must have a 'new' constructor function for simple construction (e.g `Button("click me")` )

Here's a simple example to follow:

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
