# view

A macro for constructing views.

```rust
let images = vec!["coffee.png","cream.png","sugar.png"];
let v = view!{
  VStack(direction:TOP_TO_BOTTOM){
    Image("company.png")
    Button(text:"order".to_owned(),style:BOLD)
      .on_click(|x|{console_log("ordered!")})
    ( images.into::<Vec<Image>>() )
  }
};
```

will translate to

```rust
let images = vec!["coffee.png","cream.png","sugar.png"];
let v = VStack { direction: LEFT_TO_RIGHT, ..Default::default() }.construct({
  let mut children = ViewList::new();
  children.push(Image::new("company.png"));
  children.push({
      let a = Button { text: "a".to_owned(), style: BOLD, ..Default::default() };
      a.on_click(|x|{console_log("hey")});
      a
    });
  for i in images.into<Vec<Image>>().into_iter() {
    children.push(i)
  }
  children
});
```

This project really isn't framework specific, but it does have certain rules about components

* they must implement default if you want property based construction
* they must have a 'new' constructor if you want simple construction
* they must implement Component trait
* they must have custom construct behavior if children are pregenerated

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

impl Component for VStack{
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
