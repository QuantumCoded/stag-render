pub mod bar;
pub mod word;

#[derive(Clone, Debug)]
pub enum Part {
    Bar(bar::Bar),
    Word(word::Word),
}

#[derive(Clone, Debug)]
pub enum Link {
    Attachment(Attachment),
    Connection(Connection),
}

#[derive(Clone, Debug)]
pub enum Attachment {
    Bar(bar::Attachment),
    Word(word::Attachment),
}

#[derive(Clone, Debug)]
pub enum Connection {
    Bar(bar::Connection),
    Word(word::Connection),
}
