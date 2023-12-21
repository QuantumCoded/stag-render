#![allow(dead_code)]

use part::*;
use petgraph::{dot::Dot, Graph};

mod part;

fn main() {
    let mut part_graph: Graph<Part, Link> = Graph::new();

    let bar = part_graph.add_node(Part::ClauseBar(bar::clause::ClauseBar));
    let sub = part_graph.add_node(Part::LeftWord(word::left::LeftWord("It".into())));
    let vrb = part_graph.add_node(Part::RightWord(word::right::RightWord("works".into())));
    /* let art = part_graph.add_node(Part::DiagonalWord(word::diagonal::DiagonalWord(
        "The".into(),
    ))); */

    part_graph.add_edge(
        bar,
        sub,
        Link::Connection(Connection::ClauseBar(bar::clause::Connection::Origin)),
    );

    part_graph.add_edge(
        bar,
        vrb,
        Link::Connection(Connection::ClauseBar(bar::clause::Connection::Origin)),
    );

    /*
        part_graph.add_edge(
            sub,
            art,
            Link::Attachment(Attachment::Word(word::Attachment::Under)),
        );
    */

    let out = Dot::new(&part_graph);
    println!("{out:?}");
}
