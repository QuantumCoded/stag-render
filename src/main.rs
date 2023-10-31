#![allow(dead_code)]

use part::*;
use petgraph::{dot::Dot, Graph};

mod part;

fn main() {
    let mut graph: Graph<part::Part, part::Link> = Graph::new();

    let bar = graph.add_node(Part::Bar(bar::Bar::Clause));
    let sub = graph.add_node(Part::Word(word::Word::Left("She".into())));
    let vrb = graph.add_node(Part::Word(word::Word::Right("ran".into())));

    graph.add_edge(bar, sub, Link::Connection(Connection::Bar(bar::Connection::Origin)));
    graph.add_edge(bar, vrb, Link::Connection(Connection::Bar(bar::Connection::Origin)));

    let out = Dot::new(&graph);

    println!("{out:?}");
}

