pub struct Graph<Node, NodeIterator>
where
    NodeIterator: Iterator<Item = Node>,
{
    nodes: fn() -> [Node],
    adjacent_nodes: fn(&Node) -> NodeIterator,
    node_compare: fn(&Node, &Node) -> bool,
}
