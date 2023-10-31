#![allow(dead_code)]

use part::*;
use petgraph::{dot::Dot, graph::NodeIndex, Graph};

mod part;

fn walk_graph(graph: &Graph<Part, Link>, root: NodeIndex) {
    for node in graph.neighbors(root) {
        walk_graph(graph, node);
    }

    println!("{:?}", graph.node_weight(root));
}

fn main() {
    let mut graph: Graph<Part, Link> = Graph::new();

    let bar = graph.add_node(Part::Bar(bar::Bar::Clause));
    let sub = graph.add_node(Part::Word(word::Word::Left("She".into())));
    let vrb = graph.add_node(Part::Word(word::Word::Right("ran".into())));

    graph.add_edge(
        bar,
        sub,
        Link::Connection(Connection::Bar(bar::Connection::Origin)),
    );
    graph.add_edge(
        bar,
        vrb,
        Link::Connection(Connection::Bar(bar::Connection::Origin)),
    );

    let out = Dot::new(&graph);
    println!("{out:?}");

    walk_graph(&graph, bar);
}
