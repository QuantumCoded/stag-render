#[derive(Clone, Debug)]
pub enum Word {
    Diagonal(String),
    Left(String),
    Right(String),
}

#[derive(Clone, Debug)]
pub enum Attachment {
    Under,
}

#[derive(Clone, Debug)]
pub enum Connection {
    Left,
    Right,
}
