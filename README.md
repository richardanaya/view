# view

```toml
[dependencies]
view = "0.0"
anyvec = "0.2.1"
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
  o.construct({
    let mut children = AnyVec::new();
    children.push({
      let o = Image::new("company.png");
      o.construct(None)
      o
    });
    children.push({
      let o = Button { text: "a".to_owned(), style: BOLD, ..Default::default() };
      o.on_click(|x|{ do_order() });
      o.construct(Some({
        let o = Image::new("order_icon.png");
        o.construct(None)
        o
      }));
      o
    });
    for i in Image::from_names(images).into_iter() {
      children.push(i)
    }
    children.push({
      let o = Footer{ ..Default::default() };
      o.construct(None)
      o
    });
    Some(children)
  });
  o
};
```

This project really isn't framework specific, but it does have certain rules:

* views must have a function `fn construct(&mut self, children:Option<AnyVec>)` implemented ( a View trait is included to help with this )
* views must implement Default if you want property based construction
* views must have a 'new' constructor if you want simple construction

Here's a simple example to follow:

```rust
#[derive(Default)]
struct VStack {
  direction: u8,
  children: AnyVec
}

impl VStack {
  fn new(direction:u8){
    ...
  }
  
  fn construct(&mut self, children:Option<AnyVec>) { 
    self.children = children.unwrap();
  }
}
```
