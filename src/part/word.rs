#[derive(Debug)]
pub enum Word {
    Diagonal(String),
    Left(String),
    Right(String),
}

#[derive(Debug)]
pub enum Attachment {
    Under,
}

#[derive(Debug)]
pub enum Connection {
    Left,
    Right,
}

