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
    fn new(id: &str) -> Self {
        Self { id: id.to_owned() }
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

pub(crate) struct Station {
    nodes: HashMap<Rc<Node>, Vec<Edge>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StationInput {
    start: String,
    end: String,
}

impl Station {
    pub fn new(inputs: Vec<StationInput>) -> Self {
        let mut nodes = HashMap::new();
        for StationInput { start, end } in inputs {
            // if End node is not in the graph insert
            let end_node = Rc::new(Node::new(&end));
            let edge = Edge::new(Rc::downgrade(&end_node));
            if !nodes.contains_key(&end_node) {
                nodes.insert(end_node, vec![]);
            }

            let start_node = Rc::new(Node::new(&start));
            if let Some(v) = nodes.get_mut(&start_node) {
                v.push(edge);
            } else {
                nodes.insert(start_node, vec![edge]);
            }
        }
        Self { nodes }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_station_creation() {
        let raw_json = r#"
            [
                {"start": "Station West", "end": "Entry Signal West" },
                {"start": "Entry Signal West", "end": "Point 0" },
                {"start": "Point 0", "end": "Exit Signal West 1" },
                {"start": "Point 0", "end": "Exit Signal West 2" }
            ]
        "#;
        let values: Vec<StationInput> = serde_json::from_str(raw_json).unwrap();

        let station = Station::new(values);

        let station_west = Node::new("Station West");
        let point_zero = Node::new("Point 0");
        let exit_signal_west1 = Node::new("Exit Signal West 2");

        assert!(station.nodes.contains_key(&station_west));
        assert!(station.nodes.contains_key(&point_zero));
        assert!(station.nodes.contains_key(&exit_signal_west1));

        let edges_station_west = station.nodes.get(&station_west).unwrap();
        let edges_point_zero = station.nodes.get(&point_zero).unwrap();
        let edges_exit_signal_west1 = station.nodes.get(&exit_signal_west1).unwrap();

        assert_eq!(edges_station_west.len(), 1);
        assert_eq!(edges_point_zero.len(), 2);
        assert_eq!(edges_exit_signal_west1.len(), 0);
    }
}

