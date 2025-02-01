use std::{fs::File, io::Read};

#[derive(Debug)]
pub enum Node {
    Parent {
        variant: ParentNodes,
        children: Vec<Node>,
    },
    Literal {
        variant: LiteralNodes,
        value: String,
    },
}

#[derive(Debug)]
pub enum ParentNodes {
    Paragraph,
    Heading,
}

#[derive(Debug)]
pub enum LiteralNodes {
    Text,
}

#[derive(Debug)]
pub struct Ast {
    nodes: Vec<Node>,
}

impl Ast {
    pub fn from_string(md_str: String) -> Ast {
        let mut ast: Ast = Ast { nodes: vec![] };
        for l in md_str.lines() {
            let mut chars = l.chars();
            match chars.nth(0) {
                Some('#') => {
                    let header_content = Node::Literal {
                        variant: LiteralNodes::Text,
                        value: chars.collect(),
                    };
                    let node = Node::Parent {
                        variant: ParentNodes::Heading,
                        children: vec![header_content],
                    };
                    ast.nodes.push(node);
                }
                _ => {
                    ast.nodes.push(Node::Literal {
                        variant: LiteralNodes::Text,
                        value: l.to_string(),
                    });
                }
            }
        }

        ast
    }

    pub fn from_file(file: &mut File) -> Ast {
        let mut buf: String = String::new();
        let _ = file.read_to_string(&mut buf);

        Ast::from_string(buf)
    }
}
