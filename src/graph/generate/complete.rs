use super::super::graph::Graph;

pub fn complete_graph<Node, NodeIterator>(
    node_count: usize,
    node_generator: NodeIterator,
    node_compare: fn(&Node, &Node) -> bool,
) -> Graph<Node, NodeIterator>
where
    NodeIterator: Iterator<Node>,
{
    let nodes = node_generator.take(node_count);
    let adjacent_nodes = |node| {
        node_generator
            .take(node_count)
            .filter(|adjacent_node| !node_compare(adjacent_node, node))
    };
    Graph {
        nodes: nodes,
        adjacent_nodes: adjacent_nodes,
    }
}
