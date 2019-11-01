extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use proc_macro_hack::proc_macro_hack;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::FromStr;

type PeekableTokenStream = Peekable<proc_macro::token_stream::IntoIter>;

enum NodeType {
    Empty,
    Simple(Vec<String>),
    Complex(HashMap<String, String>),
    Iterable,
}

struct NodeModifier {
    function: String,
    parameters: String,
}

struct Node {
    name: String,
    node_type: NodeType,
    modifiers: Option<Vec<NodeModifier>>,
    children: Option<Vec<Node>>,
}

impl Node {
    fn parse_view_node(mut input: PeekableTokenStream) -> Result<Node, String> {
        if let Some(TokenTree::Ident(name)) = input.next() {
            let e = Node {
                name: name.to_string(),
                node_type: NodeType::Empty,
                modifiers: None,
                children: None,
            };
            return Ok(e);
        }
        Err("unexpected start of view node".to_owned())
    }

    fn from(input: PeekableTokenStream) -> Result<Node, String> {
        Node::parse_view_node(input)
    }

    fn compile_children(&self) -> String {
        if let Some(c) = &self.children {
            return format!(
                r#"{{
                let c = Vec<View>::new();
                Some(c)
            }}"#
            )
            .to_owned();
        }
        "None".to_owned()
    }

    fn compile_empty(&self) -> String {
        let compiled_children = "None".to_owned();
        format!(
            r#"{{
            let o = {}{{..Default::default()}};
            o.construct({});
            View::{}(o)
        }}"#,
            self.name, compiled_children, self.name
        )
        .to_owned()
    }

    fn compile(&self) -> String {
        match &self.node_type {
            Empty => self.compile_empty(),
            _ => panic!("cannot handle"),
        }
    }
}

#[proc_macro_hack]
pub fn view(input: TokenStream) -> TokenStream {
    let tokens = input.into_iter().peekable();
    let e = Node::from(tokens).expect("invalid syntax");
    let s = e.compile();
    println!("{:?}", s);
    TokenStream::from_str(&s).unwrap()
}
