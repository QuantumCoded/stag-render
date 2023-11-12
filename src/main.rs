#![allow(dead_code)]

use part::*;
use petgraph::{dot::Dot, graph::NodeIndex, Graph};
use std::collections::HashMap;

mod part;

fn walk_phase_1(
    idx: NodeIndex,
    part_graph: &Graph<Part, Link>,
    svg_graph: &mut Graph<String, Vec2>,
    idx_map: &mut HashMap<NodeIndex, NodeIndex>,
) {
    // postorder transversal
    for idx in part_graph.neighbors(idx) {
        walk_phase_1(idx, part_graph, svg_graph, idx_map)
    }

    // postorder beyond this point
    
    let part = part_graph.node_weight(idx).expect("missing node weight in part graph");

    println!("{part:?}");

    // collect all the part's attachments
    let attachments: Vec<_> = part_graph.edges(idx).filter_map(|edge| {
        let link = edge.weight();

        match link {
            Link::Attachment(attachment) => Some(attachment),
            _ => None,
        }
    })
    .collect();

    let svg = part.svg(attachments);
    println!("{svg:?}");
    let svg_idx = svg_graph.add_node(svg);

    // add indices to the index map
    idx_map.insert(idx, svg_idx);
}

fn main() {
    let mut part_graph: Graph<Part, Link> = Graph::new();

    let bar = part_graph.add_node(Part::Bar(bar::Bar::Subject));
    let sub = part_graph.add_node(Part::Word(word::Word::Left("It".into())));
    let vrb = part_graph.add_node(Part::Word(word::Word::Right("works".into())));
    // let art = graph.add_node(Part::Word(word::Word::Diagonal("The".into())));

    part_graph.add_edge(
        bar,
        sub,
        Link::Connection(Connection::Bar(bar::Connection::Origin)),
    );

    part_graph.add_edge(
        bar,
        vrb,
        Link::Connection(Connection::Bar(bar::Connection::Origin)),
    );

    /* graph.add_edge(
        sub,
        art,
        Link::Attachment(Attachment::Word(word::Attachment::Under)),
    ); */

    let out = Dot::new(&part_graph);
    println!("{out:?}");

    let mut svg_graph = Graph::new();
    let mut idx_map = HashMap::new();

    walk_phase_1(bar, &part_graph, &mut svg_graph, &mut idx_map);
}
