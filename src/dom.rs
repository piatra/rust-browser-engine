use std::collections::HashMap;
use std::fmt::Show;
use std::fmt::Formatter;
use std::fmt;

#[allow(dead_code)]
pub struct Node {
    children: Vec<Node>,

    node_type: NodeType,
}

#[allow(dead_code)]
enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String)
}

#[allow(dead_code)]
struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

impl Show for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Text(ref str) => write!(f, "{}", str),
            Comment(ref str) => write!(f, "<!-- {} -->", str),
            Element(ref elem_data) => write!(f, "<{}>", elem_data.tag_name)
        }
    }
}

impl Show for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = write!(f, "{}", self.node_type);
        for i in range(0u, self.children.len()) {
            write!(f, "{}", self.children[i]);
        }
        match self.node_type {
            Element(ref elem_data) => write!(f, "</{}>", elem_data.tag_name),
            _ => write!(f, "")
        };
        res
    }
}

pub type AttrMap = HashMap<String, String>;

#[allow(dead_code)]
pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: Text(data)
    }
}

pub fn comment(textNode: Node) -> Node {
    match textNode.node_type {
        Text(content) => Node {
            children: Vec::new(),
            node_type: Comment(content)
        },
        _ => Node {
            children: Vec::new(),
            node_type: Text("comment".to_string())
        }
    }
}

#[allow(dead_code)]
pub fn elem(name: String, attr: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: Element(ElementData{
            tag_name: name,
            attributes: attr,
        })
    }
}
