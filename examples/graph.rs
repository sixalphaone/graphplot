use std::env;

use graphplot::{Edge, Layout, Multigraph, Node, Options};

fn main() {
    let mut graph = Multigraph::new();

    // 1. define a multigraph:
    for (from, to) in [
        ("A", "B"),
        ("A", "C"),
        ("A", "D"),
        ("B", "E"),
        ("B", "C"),
        ("C", "E"),
        ("C", "F"),
        ("G", "G"),
        ("G", "H"),
        ("G", "H"),
        ("H", "G"),
    ] {
        // add node
        let from_id = graph.add_node(Node::from(from), false);
        let to_id = graph.add_node(Node::from(to), false);

        // add edge
        graph.add_edge(
            Edge::from(from_id, to_id).typst(format!("$ {from} arrow.r {to} $ ")),
            true,
        );
    }

    // 2. highlight some nodes & edges
    graph.highlight_edges([3, 4]);
    graph.highlight_nodes([1, 2, 4]);

    // 3. export plot
    let config = Options::default().layout(Layout::Radial);
    let api_key = env::var("GP_API_KEY").expect("No Graphplot API env variable");
    graph
        .save("graph.svg", api_key, Some(config))
        .expect("Error plotting with Graphplot API")
}
