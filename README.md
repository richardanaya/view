# view

A macro for constructing views that is non framework specific.

```rust
let images = vec!["coffee.png","cream.png","sugar.png"];
let v = view!{
  // views with children
  VStack(direction:TOP_TO_BOTTOM) {
    // simple view construction
    Image("company.png") 
    // complex view construction
    Button(text:"order".to_owned(),style:BOLD) 
      .on_click(|x|{console_log("ordered!")})
    // views from iterables
    ( Image::from_names(images) ) 
  }
};
```

will translate to

```rust
let images = vec!["coffee.png","cream.png","sugar.png"];
let v = { 
  let o = VStack { direction: LEFT_TO_RIGHT, ..Default::default() }
  o.construct({
    let mut children = ViewList::new();
    children.push({
      let o = Image::new("company.png");
      o.construct(None)
      o
    });
    children.push({
      let o = Button { text: "a".to_owned(), style: BOLD, ..Default::default() };
      o.construct(None);
      o.on_click(|x|{console_log("hey")});
      o
    });
    for i in Image::from_names(images).into_iter() {
      children.push(i)
    }
    Some(children)
  });
  o
};
```

This project really isn't framework specific, but it does have certain rules about components

* they must implement Default if you want property based construction
* they must have a 'new' constructor if you want simple construction
* they must have a function `fn construct(&mut self, children:Option<ViewList>)` implemented ( a View trait in included to help with this )

Here's a simple example to follow:

```rust
#[derive(Default)]
struct VStack {
  direction: u8,
  children: ViewList
}

impl VStack {
  fn new(direction:u8){
    ...
  }
  
  fn construct(&mut self, children:Option<ViewList>) { 
    self.children = children.unwrap();
  }
}
```

# View trait

An optional helper trait exists for defining view behavior

```rust
trait View {
  fn construct(&mut self, children:Option<ViewList>)
}
```
