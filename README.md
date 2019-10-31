# view

A macro for constructing views.

```rust
let v = view!{
  VStack(direction:LEFT_TO_RIGHT){
    Button(text:"a".to_owned(),style:BOLD)
      .on_click(|x|{console_log("hey")})
    Button("b")
    Button("c")
  }
};
```

will translate to

```rust
let v = VStack { direction: LEFT_TO_RIGHT, children: {
  let mut children = AnyVec::new();
  children.push({
      let a = Button { text: "a".to_owned(), style: BOLD, ..Default::default() };
      a.on_click(|x|{console_log("hey")});
      a
    });
  children.push(Button::new("b"));
  children.push(Button::new("c"));
  children
}, ..Default::default() };
```
