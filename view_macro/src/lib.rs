extern crate proc_macro;
use proc_macro_hack::proc_macro_hack;
use proc_macro::{Delimiter, TokenStream, TokenTree};
//use std::collections::HashMap;
use std::iter::Peekable;
use std::str::FromStr;

type PeekableTokenStream = Peekable<proc_macro::token_stream::IntoIter>;

#[derive(Debug)]
enum NodeType {
    Empty,
    If(String),
    For(String),
    Simple(String),
    Params(String),
}

#[derive(Debug)]
struct NodeModifier {
    function: String,
    parameters: String,
}

#[derive(Debug)]
struct Node {
    name: String,
    node_type: NodeType,
    modifiers: Option<Vec<NodeModifier>>,
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

    fn parse_view_node_args(
        mut input: PeekableTokenStream,
    ) -> Result<(Option<String>, bool, PeekableTokenStream), String> {
        let mut is_structured = true;
        let mut has_args = false;
        if let Some(TokenTree::Group(t)) = input.peek() {
            if t.delimiter() == Delimiter::Parenthesis {
                has_args = true;
            }
        }

        let mut args = None;
        if has_args {
            if let Some(TokenTree::Group(t)) = input.next() {
                {
                    let mut s = t.stream().into_iter();
                    {
                        if let Some(TokenTree::Ident(_)) = s.next() {
                        } else {
                            is_structured = false;
                        }
                    }
                    if is_structured {
                        if let Some(TokenTree::Punct(p)) = s.next() {
                            if p.as_char() != ':' {
                                is_structured = false;
                            }
                        } else {
                            is_structured = false;
                        }
                    }
                }
                args = Some(t.stream().to_string());
            }
        } else {
            is_structured = false;
        }
        Ok((args, is_structured, input))
    }

    fn parse_view_node_mods(
        mut input: PeekableTokenStream,
    ) -> Result<(Option<Vec<NodeModifier>>, PeekableTokenStream), String> {
        let mut mods = vec![];
        loop {
            if let Some(TokenTree::Punct(p)) = input.peek() {
                if p.as_char() != '.' {
                    break;
                }
            } else {
                break;
            }
            input.next();
            let mut name = None;
            if let Some(TokenTree::Ident(i)) = input.next() {
                name = Some(i.to_string());
            }
            if let Some(function_name) = name {
                if let Some(TokenTree::Group(g)) = input.next() {
                    if g.delimiter() == Delimiter::Parenthesis {
                        mods.push(NodeModifier {
                            function: function_name,
                            parameters: g.stream().to_string(),
                        })
                    }
                }
            } else {
                return Err("unexpected function name after .".to_owned());
            }
        }
        if mods.len() == 0 {
            Ok((None, input))
        } else {
            Ok((Some(mods), input))
        }
    }

    fn parse_if_view(
        mut input: PeekableTokenStream,
    ) -> Result<(Node, PeekableTokenStream), String> {
        let (args, _, i) = Node::parse_view_node_args(input)?;
        input = i;
        if args.is_none() {
            return Err("if requires arguments".to_string());
        }
        let (children, i) = Node::parse_view_node_children(input)?;
        input = i;
        if children.is_none() {
            return Err("if requires children".to_string());
        }
        let e = Node {
            name: "If".to_string(),
            node_type: NodeType::If(args.unwrap()),
            modifiers: None,
            children: children,
        };
        Ok((e, input))
    }

    fn parse_for_view(
        mut input: PeekableTokenStream,
    ) -> Result<(Node, PeekableTokenStream), String> {
        let (args, _, i) = Node::parse_view_node_args(input)?;
        input = i;
        if args.is_none() {
            return Err("for requires arguments".to_string());
        }
        let (children, i) = Node::parse_view_node_children(input)?;
        input = i;
        if children.is_none() {
            return Err("for requires children".to_string());
        }
        let e = Node {
            name: "For".to_string(),
            node_type: NodeType::For(args.unwrap()),
            modifiers: None,
            children: children,
        };
        Ok((e, input))
    }

    fn parse_user_view(
        name: String,
        mut input: PeekableTokenStream,
    ) -> Result<(Node, PeekableTokenStream), String> {
        let (args, structed_args, i) = Node::parse_view_node_args(input)?;
        input = i;
        let (mods, i) = Node::parse_view_node_mods(input)?;
        input = i;
        let (children, i) = Node::parse_view_node_children(input)?;
        input = i;
        if args.is_none() {
            let e = Node {
                name: name,
                node_type: NodeType::Empty,
                modifiers: mods,
                children: children,
            };
            Ok((e, input))
        } else {
            if structed_args {
                let e = Node {
                    name: name,
                    node_type: NodeType::Params(args.unwrap()),
                    modifiers: mods,
                    children: children,
                };
                Ok((e, input))
            } else {
                let e = Node {
                    name: name,
                    node_type: NodeType::Simple(args.unwrap()),
                    modifiers: mods,
                    children: children,
                };
                Ok((e, input))
            }
        }
    }

    fn parse_view_node(
        mut input: PeekableTokenStream,
    ) -> Result<(Node, PeekableTokenStream), String> {
        let mut node_name = None;
        if let Some(TokenTree::Ident(name)) = input.next() {
            node_name = Some(name.to_string());
        }
        if let Some(name) = node_name {
            if name == "If" {
                Node::parse_if_view(input)
            } else if name == "For" {
                Node::parse_for_view(input)
            } else {
                Node::parse_user_view(name, input)
            }
        } else {
            Err("unexpected start of view node".to_owned())
        }
    }

    fn from(input: PeekableTokenStream) -> Result<Node, String> {
        let (n, _) = Node::parse_view_node(input)?;
        Ok(n)
    }

    fn compile_mods(&self) -> String {
        if let Some(m) = &self.modifiers {
            if m.len() == 0 {
                "".to_owned()
            } else {
                m.iter()
                    .map(|x| format!(r#"o.{}({});"#, x.function, x.parameters).to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            }
        } else {
            "".to_owned()
        }
    }

    fn compile_children(&self) -> String {
        if let Some(c) = &self.children {
            if c.len() == 0 {
                return "None".to_owned();
            }
            let compiled_children = c
                .iter()
                .map(|x| match &x.node_type {
                    NodeType::If(args) => {
                        let if_children = x.compile_children();
                        format!(
                            r#"if {} {{ 
                            c.append(&mut ({}).unwrap())
                        }}"#,
                            args, if_children
                        )
                        .to_string()
                    }
                    NodeType::For(args) => {
                        let if_children = x.compile_children();
                        format!(
                            r#"for {} {{ 
                            c.append(&mut ({}).unwrap());
                        }}"#,
                            args, if_children
                        )
                        .to_string()
                    }
                    _ => format!("c.push({});\n", x.compile()).to_owned(),
                })
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

    fn compile_user_node(&self) -> String {
        let mods = self.compile_mods();
        let compiled_children = self.compile_children();
        match &self.node_type {
            NodeType::Empty => format!(
                r#"{{
                        let mut o = {}{{..Default::default()}};
                        {}
                        o.construct({});
                        View::{}(o)
                    }}"#,
                self.name, mods, compiled_children, self.name
            )
            .to_owned(),
            NodeType::Params(args) => format!(
                r#"{{
                        let mut o = {}{{ {},..Default::default()}};
                        {}
                        o.construct({});
                        View::{}(o)
                    }}"#,
                self.name, args, mods, compiled_children, self.name
            )
            .to_owned(),
            NodeType::Simple(args) => format!(
                r#"{{
                        let mut o = {}::new({});
                        {}
                        o.construct({});
                        View::{}(o)
                    }}"#,
                self.name, args, mods, compiled_children, self.name
            )
            .to_owned(),
            _ => panic!("cannot start with non-user view"),
        }
    }

    fn compile(&self) -> String {
        match &self.node_type {
            NodeType::Empty => self.compile_user_node(),
            NodeType::Simple(_) => self.compile_user_node(),
            NodeType::Params(_) => self.compile_user_node(),
            _ => panic!("cannot start with non-user view"),
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
