use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::zip;
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

#[derive(Debug)]
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
    // TODO: change Vector<Edge> to HashMap
    nodes: HashMap<Rc<Node>, Vec<Edge>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StationEdge {
    start: String,
    end: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StationEdgeState {
    start: String,
    end: String,
    occupied: bool,
}

impl Station {
    pub fn new(inputs: Vec<StationEdge>) -> Self {
        let mut nodes: HashMap<Rc<Node>, Vec<Edge>> = HashMap::new();
        for StationEdge { start, end } in inputs {
            let start_node = Node::new(&start);
            let end_node = Node::new(&end);

            nodes.entry(Rc::new(start_node.clone())).or_default();
            nodes.entry(Rc::new(end_node.clone())).or_default();

            let start_rc = nodes.get_key_value(&start_node).unwrap().0.clone();
            let end_rc = nodes.get_key_value(&end_node).unwrap().0.clone();

            let edge = Edge::new(Rc::downgrade(&end_rc));
            let reverse_edge = Edge::new(Rc::downgrade(&start_rc));

            nodes.get_mut(&start_rc).unwrap().push(edge);
            nodes.get_mut(&end_rc).unwrap().push(reverse_edge);
        }
        Self { nodes }
    }

    fn bread_first_search(&self, start: Rc<Node>, end: Rc<Node>) -> Vec<Rc<Node>> {
        tracing::debug!("calculating path between: {start:?} - {end:?}");
        let mut path = vec![];
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent_map = HashMap::new();

        queue.push_back(start);

        while let Some(node) = queue.pop_front()
            && let Some(edges) = self.nodes.get(&node)
        {
            visited.insert(Rc::clone(&node));
            println!("Pop Node to the queue: {node:?}");

            if node == end {
                let mut parent = Some(end);
                while let Some(parent_node) = parent {
                    path.push(Rc::clone(&parent_node));
                    parent = parent_map.get(&parent_node).cloned();
                }
                path.reverse();
                println!("path found: {path:?}");
                return path;
            }

            for edge in edges {
                println!("edge: {edge:?}");
                if let Some(upgraded) = edge.to.upgrade()
                    && !visited.contains(&upgraded)
                {
                    queue.push_back(Rc::clone(&upgraded));
                    parent_map.insert(Rc::clone(&upgraded), Rc::clone(&node));
                    println!("Adding Node: {upgraded:?}  and updating parent to: {node:?}");
                }
            }
        }
        tracing::debug!("path found: {path:?}");
        path
    }

    pub fn update_occupancy(&mut self, inputs: Vec<StationEdgeState>) {
        for StationEdgeState {
            start,
            end,
            occupied,
        } in inputs
        {
            if occupied {
                let (start_node, _) = self.nodes.get_key_value(&Node::new(&start)).unwrap();
                let (end_node, _) = self.nodes.get_key_value(&Node::new(&end)).unwrap();
                let path = self.bread_first_search(Rc::clone(start_node), Rc::clone(end_node));
                println!("{path:?}");
                for (x, y) in zip(path.iter(), path.iter().skip(1)) {
                    self.update_edge(x, y);
                }
            }
        }
    }

    fn update_edge(&mut self, start: &Rc<Node>, end: &Rc<Node>) {
        if let Some(edges) = self.nodes.get_mut(start) {
            for edge in edges {
                let is_equal = edge.to.upgrade().is_some_and(|rc| rc == *end);
                if is_equal {
                    tracing::debug!("updating edge between: {start:?}-{end:?}");
                    edge.occupied = true;
                }
            }

            if let Some(edges) = self.nodes.get_mut(end) {
                for edge in edges {
                    let is_equal = edge.to.upgrade().is_some_and(|rc| rc == *start);
                    if is_equal {
                        tracing::debug!("updating edge between: {end:?}-{start:?}");
                        edge.occupied = true;
                    }
                }
            }
        }
    }

    pub fn check_route(&self, StationEdge { start, end }: StationEdge) -> bool {
        let (start_node, _) = self.nodes.get_key_value(&Node::new(&start)).unwrap();
        let (end_node, _) = self.nodes.get_key_value(&Node::new(&end)).unwrap();
        let path = self.bread_first_search(Rc::clone(start_node), Rc::clone(end_node));
        let edges = zip(path.iter(), path.iter().skip(1)).map(|(x, y)| self.get_edge(x, y));
        edges
            .map(|edge| edge.is_none_or(|x| x.occupied))
            .all(|x| !x)
    }

    fn get_edge(&self, node: &Rc<Node>, to: &Rc<Node>) -> Option<&Edge> {
        let mut edge = None;
        if let Some(edges) = self.nodes.get(node) {
            edge = edges
                .iter()
                .find(|x| x.to.upgrade().is_some_and(|rc| rc == *to))
        }
        edge
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_station() -> Station {
        let raw_json = r#"
            [
                {"start": "Station West", "end": "Entry Signal West" },
                {"start": "Entry Signal West", "end": "Point 0" },
                {"start": "Point 0", "end": "Exit Signal West 1" },
                {"start": "Point 0", "end": "Exit Signal West 2" }
            ]
        "#;
        let values: Vec<StationEdge> = serde_json::from_str(raw_json).unwrap();

        Station::new(values)
    }

    #[test]
    fn test_station_creation() {
        let station = create_station();
        let entry_signal_west = Node::new("Entry Signal West");
        let point_zero = Node::new("Point 0");
        let exit_signal_west1 = Node::new("Exit Signal West 2");

        assert!(station.nodes.contains_key(&entry_signal_west));
        assert!(station.nodes.contains_key(&point_zero));
        assert!(station.nodes.contains_key(&exit_signal_west1));

        let edges_entry_signal_west = station.nodes.get(&entry_signal_west).unwrap();
        let edges_point_zero = station.nodes.get(&point_zero).unwrap();
        let edges_exit_signal_west1 = station.nodes.get(&exit_signal_west1).unwrap();

        assert_eq!(edges_entry_signal_west.len(), 2);
        assert_eq!(edges_point_zero.len(), 3);
        assert_eq!(edges_exit_signal_west1.len(), 1);
    }

    #[test]
    fn test_update_occupancy() {
        let raw_routes = r#"
        [
            {"start": "Entry Signal West", "end": "Exit Signal West 1", "occupied": false },
            {"start": "Entry Signal West", "end": "Exit Signal West 2", "occupied": true }
        ]
        "#;
        let inputs: Vec<StationEdgeState> = serde_json::from_str(raw_routes).unwrap();
        let mut station = create_station();
        station.update_occupancy(inputs);

        let entry_signal_west = Node::new("Entry Signal West");
        let edges_entry_signal_west = station.nodes.get(&entry_signal_west).unwrap();
        println!("{edges_entry_signal_west:?}");

        assert!(
            edges_entry_signal_west
                .iter()
                .find(|x| x.occupied)
                .is_some()
        )
    }

    #[test]
    fn test_bfs() {
        let station = create_station();
        let start = Rc::new(Node::new("Station West"));
        let end = Rc::new(Node::new("Exit Signal West 2"));
        let path = station.bread_first_search(Rc::clone(&start), Rc::clone(&end));

        assert!(!path.is_empty());
        assert_eq!(path.first().unwrap().id, "Station West");
        assert_eq!(path.last().unwrap().id, "Exit Signal West 2");
        assert_eq!(path.len(), 4);

        let path = station.bread_first_search(Rc::clone(&end), Rc::clone(&start));
        assert!(!path.is_empty());
        assert_eq!(path.last().unwrap().id, "Station West");
        assert_eq!(path.first().unwrap().id, "Exit Signal West 2");
        assert_eq!(path.len(), 4);
    }
}
