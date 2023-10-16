use crate::rkpart::{bar::Bar, word::Word, RKGraph, RKPart, RKNode};

mod rkpart;

fn main() {
    let mut rkgraph = RKGraph::new();

    let clause = rkgraph.add_node(RKPart::Bar(Bar::ClauseBar));
    let subject = rkgraph.add_node(RKPart::Word(Word::LeftWord("king".into())));
    let subj_mod = rkgraph.add_node(RKPart::Word(Word::DiagWord("The".into())));
    let verb = rkgraph.add_node(RKPart::Word(Word::RightWord("ran".into())));
    let prep = rkgraph.add_node(RKPart::Word(Word::DiagWord("to".into())));
    let prep_obj = rkgraph.add_node(RKPart::Word(Word::RightWord("throne".into())));
    let prep_obj_mod = rkgraph.add_node(RKPart::Word(Word::DiagWord("his".into())));

    rkgraph.add_edge(clause, subject, rkgraph.node_weight(clause).unwrap()
    .attach())

    println!("Hello, world!");
}
