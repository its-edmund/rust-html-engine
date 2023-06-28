use core::fmt;
use std::collections::HashMap;

enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
pub struct Node {
    node_type: NodeType,
    children: Vec<Node>,
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

pub type AttrMap = HashMap<String, String>;

pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn element(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
    }
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeType::Text(s) => write!(f, "{}", s),
            NodeType::Element(e) => write!(f, "{}\n{:?}", e.tag_name, e.attributes),
        }
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeType::Text(s) => write!(f, "{}", s),
            NodeType::Element(e) => write!(f, "{}", e.tag_name),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
