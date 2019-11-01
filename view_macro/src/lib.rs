extern crate proc_macro;

use proc_macro::{Delimiter, TokenStream, TokenTree};
use proc_macro_hack::proc_macro_hack;
//use std::collections::HashMap;
use std::iter::Peekable;
use std::str::FromStr;

type PeekableTokenStream = Peekable<proc_macro::token_stream::IntoIter>;

#[derive(Debug)]
enum NodeType {
    Empty,
    //Simple(Vec<String>),
    //Complex(HashMap<String, String>),
    //Iterable,
}

/*
#[derive(Debug)]
struct NodeModifier {
    function: String,
    parameters: String,
}*/

#[derive(Debug)]
struct Node {
    name: String,
    node_type: NodeType,
    //modifiers: Option<Vec<NodeModifier>>,
    children: Option<Vec<Node>>,
}

impl Node {
    fn parse_view_node_children(
        mut input: PeekableTokenStream,
    ) -> Result<(Option<Vec<Node>>, PeekableTokenStream), String> {
        let mut has_children = false;
        if let Some(TokenTree::Group(t)) = input.peek() {
            if t.delimiter() == Delimiter::Brace {
                has_children = true;
            } else {
                return Err("unexpected punctuation syntax of child start".to_owned());
            }
        }

        let mut node_children = None;
        if has_children {
            let mut child_input: Option<PeekableTokenStream> = None;
            if let Some(TokenTree::Group(t)) = input.next() {
                child_input = Some(t.stream().into_iter().peekable());
            }

            if let Some(mut child_input) = child_input {
                let mut children = vec![];
                loop {
                    if let None = child_input.peek() {
                        break;
                    } else {
                        let (n, i) = Node::parse_view_node(child_input)?;
                        child_input = i;
                        children.push(n)
                    }
                }
                node_children = Some(children)
            } else {
                return Err("I have no idea how you'd get here".to_owned());
            }
        }
        Ok((node_children, input))
    }

    fn parse_view_node(
        mut input: PeekableTokenStream,
    ) -> Result<(Node, PeekableTokenStream), String> {
        let mut node_name = None;
        if let Some(TokenTree::Ident(name)) = input.next() {
            node_name = Some(name.to_string());
        }

        if let Some(name) = node_name {
            let (children, i) = Node::parse_view_node_children(input).unwrap();
            input = i;
            let e = Node {
                name: name,
                node_type: NodeType::Empty,
                //modifiers: None,
                children: children,
            };
            return Ok((e, input));
        } else {
            Err("unexpected start of view node".to_owned())
        }
    }

    fn from(input: PeekableTokenStream) -> Result<Node, String> {
        let (n, _) = Node::parse_view_node(input)?;
        Ok(n)
    }

    fn compile_children(&self) -> String {
        if let Some(c) = &self.children {
            if c.len() == 0 {
                return "None".to_owned();
            }
            let compiled_children = c
                .iter()
                .map(|x| format!("c.push({});\n", x.compile()).to_owned())
                .collect::<Vec<String>>()
                .join("\n");
            return format!(
                r#"{{
                    let mut c = Vec::<View>::new();
                    {}
                    Some(c)
                }}"#,
                compiled_children
            )
            .to_owned();
        }
        "None".to_owned()
    }

    fn compile_empty(&self) -> String {
        let compiled_children = self.compile_children();
        format!(
            r#"{{
                let mut o = {}{{..Default::default()}};
                o.construct({});
                View::{}(o)
            }}"#,
            self.name, compiled_children, self.name
        )
        .to_owned()
    }

    fn compile(&self) -> String {
        match &self.node_type {
            _ => self.compile_empty(),
            // _ => panic!("cannot handle"),
        }
    }
}

#[proc_macro_hack]
pub fn view(input: TokenStream) -> TokenStream {
    let tokens = input.into_iter().peekable();
    let e = Node::from(tokens).expect("invalid syntax");
    let s = e.compile();
    TokenStream::from_str(&s).unwrap()
}
