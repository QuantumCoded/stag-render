pub mod bar;
pub mod word;

#[derive(Debug)]
pub enum Part {
    ClauseBar(bar::clause::ClauseBar),
    ComplementBar(bar::complement::ComplementBar),
    ObjectBar(bar::object::ObjectBar),

    DiagonalWord(word::diagonal::DiagonalWord),
    LeftWord(word::left::LeftWord),
    RightWord(word::right::RightWord),
}

#[derive(Debug)]
pub enum Attachment {
    ClauseBar(bar::clause::Attachment),
    ComplementBar(bar::complement::Attachment),
    ObjectBar(bar::object::Attachment),

    DiagonalWord(word::diagonal::Attachment),
    LeftWord(word::left::Attachment),
    RightWord(word::right::Attachment),
}

#[derive(Debug)]
pub enum Connection {
    ClauseBar(bar::clause::Connection),
    ComplementBar(bar::complement::Connection),
    ObjectBar(bar::object::Connection),

    DiagonalWord(word::diagonal::Connection),
    LeftWord(word::left::Connection),
    RightWord(word::right::Connection),
}

#[derive(Debug)]
pub enum Link {
    Attachment(Attachment),
    Connection(Connection),
}
