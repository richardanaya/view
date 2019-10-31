# view

A macro for constructing views.

```rust
let images = vec!["coffee.png","cream.png","sugar.png"];
let v = view!{
  // views with children
  VStack(direction:TOP_TO_BOTTOM) {
    // simple view construction
    Image("company.png") 
    // complex viw construction
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
  o};
```

This project really isn't framework specific, but it does have certain rules about components

* they must implement Default if you want property based construction
* they must have a 'new' constructor if you want simple construction
* they must implement View trait to handle children (or derive if you do nothing with children)

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
}

impl View for VStack{
  fn construct(&mut self, children:Option<ViewList>) { 
    children = children.unwrap();
  }
}
```

# Component
```rust
trait Component {
  fn construct(&mut self, children:Option<ViewList>)
}
```
