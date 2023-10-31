pub mod bar;
pub mod word;

#[derive(Debug)]
pub enum Part {
    Bar(bar::Bar),
    Word(word::Word),
}

#[derive(Debug)]
pub enum Link {
    Attachment(Attachment),
    Connection(Connection),
}

#[derive(Debug)]
pub enum Attachment {
    Bar(bar::Attachment),
    Word(word::Attachment),
}

#[derive(Debug)]
pub enum Connection {
    Bar(bar::Connection),
    Word(word::Connection),
}
