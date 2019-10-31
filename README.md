# view

A macro for constructing views.

```rust
let v = view!{
  VStack(direction:LEFT_TO_RIGHT){
    Button(text:"a".to_owned()) {
      .on_click(|x|{console_log("hey")})
    }
    Button(text:"b".to_owned())
    Button(text:"c".to_owned())
  }
}
```

will translate to

```rust
let v = VStack { direction: LEFT_TO_RIGHT, children: vec![
  {
    let a = Button { text: "a".to_owned(), ..Default::default() };
    a.on_click(|x|{console_log("hey")});
    a
  },
  Button { text: "b".to_owned(), ..Default::default() },
  Button { text: "c".to_owned(), ..Default::default() },
], ..Default::default() };
```
