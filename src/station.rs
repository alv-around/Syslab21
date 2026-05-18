use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::{Rc, Weak};

// TODO: implement NodeType
// enum NodeType {
//  Side,
//  EntrySignal,
//  ExitSignal,
//  Point,
// }

// TODO: add field `type: NodeType,`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Node {
    id: String,
}

impl Node {
    fn new(id: String) -> Self {
        Self { id }
    }
}

// #[derive(Debug, Deserialize, Serialize)]
pub struct Edge {
    to: Weak<Node>,
    occupied: bool,
}

impl Edge {
    fn new(to: Weak<Node>) -> Self {
        Self {
            to,
            occupied: false,
        }
    }
}

struct Station {
    nodes: HashMap<Rc<Node>, Vec<Edge>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StationInput {
    start: String,
    end: String,
}

impl Station {
    pub fn new(inputs: Vec<StationInput>) -> Self {
        let mut nodes = HashMap::new();
        for StationInput { start, end } in inputs {
            // if End node is not in the graph insert
            let end_node = Rc::new(Node::new(end));
            let edge = Edge::new(Rc::downgrade(&end_node));
            if !nodes.contains_key(&end_node) {
                nodes.insert(end_node, vec![]);
            }

            let start_node = Rc::new(Node::new(start));
            if let Some(v) = nodes.get_mut(&start_node) {
                v.push(edge);
            } else {
                nodes.insert(start_node, vec![edge]);
            }
        }
        Self { nodes }
    }
}

