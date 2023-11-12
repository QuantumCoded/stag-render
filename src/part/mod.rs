pub mod bar;
pub mod length;
pub mod vec2;
pub mod word;

pub use length::*;
pub use vec2::*;

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

impl Part {
    pub fn svg(&self, attachments: Vec<&Attachment>) -> String {
        match self {
            Part::Bar(bar) => {
                let attachments: Vec<_> = attachments
                    .into_iter()
                    .filter_map(|att| match att {
                        Attachment::Bar(att) => Some(att),
                        _ => None,
                    })
                    .collect();

                bar.svg(attachments)
            }

            Part::Word(word) => {
                let attachments: Vec<_> = attachments
                    .into_iter()
                    .filter_map(|att| match att {
                        Attachment::Word(att) => Some(att),
                        _ => None,
                    })
                    .collect();

                // word.svg(attachments)
                todo!()
            }
        }
    }

    pub fn size(&self, attachments: Vec<&Attachment>) -> Vec2 {
        match self {
            Part::Bar(bar) => {
                let attachments: Vec<_> = attachments
                    .into_iter()
                    .filter_map(|att| match att {
                        Attachment::Bar(att) => Some(att),
                        _ => None,
                    })
                    .collect();

                bar.size(attachments)
            }

            Part::Word(_word) => todo!(),
        }
    }

    pub fn offset(&self, attachments: Vec<&Attachment>) -> Vec2 {
        match self {
            Part::Bar(bar) => {
                let attachments: Vec<_> = attachments
                    .into_iter()
                    .filter_map(|att| match att {
                        Attachment::Bar(att) => Some(att),
                        _ => None,
                    })
                    .collect();

                bar.offset(attachments)
            }

            Part::Word(_word) => todo!(),
        }
    }
}
