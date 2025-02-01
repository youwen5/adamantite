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
    Heading { title: String },
}

#[derive(Debug)]
pub enum LiteralNodes {
    Text,
}

#[derive(Debug)]
pub struct Ast {
    nodes: Vec<Node>,
}

struct Marker {
    begin: &'static str,
    end: &'static str,
}

struct SemanticObject {
    marker: Marker,
    variant: Node,
}

const HEADING = SemanticObject {
    marker: Marker {
        begin: r#"\n#"#,
        end: r#"\n"#
    },
    variant: Node::Parent 
}

impl Ast {
    pub fn from_string(md_str: String) -> Ast {
        let mut ast: Ast = Ast { nodes: vec![] };
        for c in md_str.chars() {}
        //for l in md_str.lines() {
        //    let mut chars = l.chars();
        //    match chars.nth(0) {
        //        Some('#') => {
        //            let header_title = chars.collect();
        //            let node = Node::Parent {
        //                variant: ParentNodes::Heading {
        //                    title: header_title,
        //                },
        //                children: vec![header_title],
        //            };
        //            ast.nodes.push(node);
        //        }
        //        _ => {
        //            ast.nodes.push(Node::Literal {
        //                variant: LiteralNodes::Text,
        //                value: l.to_string(),
        //            });
        //        }
        //    }
        //}

        ast
    }

    pub fn from_file(file: &mut File) -> Ast {
        let mut buf: String = String::new();
        let _ = file.read_to_string(&mut buf);

        Ast::from_string(buf)
    }
}
