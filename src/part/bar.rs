#[derive(Clone, Debug)]
pub enum Bar {
    Clause,
    Complement,
    Object,
}

#[derive(Clone, Debug)]
pub enum Attachment {}

#[derive(Clone, Debug)]
pub enum Connection {
    Origin,
}
