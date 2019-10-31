# view

A macro for constructing views.

```rust
view!{
  VStack(direction:LeftToRight){
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
VStack { foo: 42, children: vec![
  {
    let a = Button { text: "a".to_owned(), ..Default::default() };
    a.on_click(|x|{console_log("hey")});
    a
  },
  Button { text: "b".to_owned(), ..Default::default() },
  Button { text: "c".to_owned(), ..Default::default() },
], ..Default::default() }
```
