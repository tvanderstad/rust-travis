use std::collections;
use std::vec;

pub trait Graph<Node, NodeIterator>
where
    NodeIterator: Iterator<Item = Node>,
{
    fn nodes(&self) -> NodeIterator;
    fn adjacent_nodes(&self, node: Node) -> NodeIterator;
    fn equal(node_a: Node, node_b: Node) -> bool;
}

pub struct GraphByValue<Node> {
    nodes: vec::Vec<Node>,
    adjacent_nodes: collections::HashMap<Node, vec::Vec<Node>>,
    node_compare: fn(&Node, &Node) -> bool, // todo: change to be a templated comparator
}
