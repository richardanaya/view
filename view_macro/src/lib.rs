extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use proc_macro_hack::proc_macro_hack;
use std::str::FromStr;
use std::collections::HashMap;
use std::iter::Peekable;

type PeekableTokenStream =  Peekable<proc_macro::token_stream::IntoIter>;

enum NodeType {
    Empty,
    Simple(Vec<String>),
    Complex(HashMap<String,String>),
    Iterable
}

struct NodeModifier {
    function:String,
    parameters:String
}

struct Node {
    name: String,
    node_type : NodeType,
    modifiers: Vec<NodeModifier>,
    children: Vec<Node>
}

impl Node {
    fn parse_view_node(mut input:PeekableTokenStream) -> Result<Node,String> {
        if let Some(TokenTree::Ident(name)) = input.next() {
            let e = Node {
                name: name.to_string(),
                node_type: NodeType::Empty,
                modifiers: vec![],
                children: vec![]
            };
            return Ok(e);
        }
        Err("unexpected start of view node".to_owned())
    }

    fn from(input:PeekableTokenStream) -> Result<Node,String> {
        Node::parse_view_node(input)
    }

    fn compile_empty(&self) -> String {
        (self.name.clone()+"{..Default::default()}").to_owned()
    }

    fn compile(&self) -> String {
        match &self.node_type {
            Empty => self.compile_empty(),
            _ => panic!("cannot handle")
        }
    }
}

#[proc_macro_hack]
pub fn view(input: TokenStream) -> TokenStream {
    let tokens = input.into_iter().peekable();
    let e = Node::from(tokens).expect("invalid syntax");
    TokenStream::from_str(&e.compile()).unwrap()
}
