#![allow(dead_code)]

use part::*;
use petgraph::{dot::Dot, graph::NodeIndex, Graph};
use std::collections::HashMap;

mod part;

fn walk_graph(graph: &Graph<Part, Link>, root: NodeIndex, other: &mut Graph<Part, Link>) {
    for node in graph.neighbors(root) {
        walk_graph(graph, node, other);
    }

    // this point on will always be postorder
    let node = graph.node_weight(root);

    println!("{node:?}");

    // if the current node is alreay on the second graph then we just
    // need to do its parents. use the map for checking for nodes.

    // add the node from the first graph to the second one
    other.add_node(node.unwrap().clone());

    // copy the parents to the other graph

    // form the edges between these two, possibly needs a map
}

fn main() {
    let mut graph: Graph<Part, Link> = Graph::new();

    let bar = graph.add_node(Part::Bar(bar::Bar::Clause));
    let sub = graph.add_node(Part::Word(word::Word::Left("girl".into())));
    let vrb = graph.add_node(Part::Word(word::Word::Right("ran".into())));
    let art = graph.add_node(Part::Word(word::Word::Diagonal("The".into())));

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

    graph.add_edge(
        sub,
        art,
        Link::Attachment(Attachment::Word(word::Attachment::Under)),
    );

    let out = Dot::new(&graph);
    println!("{out:?}");

    let mut graph2: Graph<Part, Link> = Graph::new();

    walk_graph(&graph, bar, &mut graph2);
}
