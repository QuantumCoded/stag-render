use super::{length::Len, vec2::Vec2};

#[derive(Clone, Debug)]
pub enum Bar {
    Complement,
    Object,
    Subject,
}

#[derive(Clone, Debug)]
pub enum Attachment {}

#[derive(Clone, Debug)]
pub enum Connection {
    Origin,
}

impl Bar {
    pub fn svg(&self, _attachments: Vec<&Attachment>) -> String {
        match self {
            Bar::Complement => String::from(
                r#"\
                    <line x1="0" x2="-2ch" y1="0" y2="-1.5em" stroke="black" />\n\
                    <line x1="0" x2="-2ch" y1="0" y2="0" stroke="black" />\
                "#,
            ),

            Bar::Object => {
                String::from(r#"<line x1="0" x2="0" y1="0" y2="-1.5em" stroke="black" />"#)
            }

            Bar::Subject => {
                String::from(r#"<line x1="0" x2="0" y1="0" y2="3em" stroke="black" />"#)
            }
        }
    }

    pub fn size(&self, _attachments: Vec<&Attachment>) -> Vec2 {
        match self {
            Bar::Complement => Vec2(
                Len {
                    ch: 2.0,
                    ..Default::default()
                },
                Len {
                    em: 1.5,
                    ..Default::default()
                },
            ),

            Bar::Object => Vec2(
                Len::default(),
                Len {
                    em: 1.5,
                    ..Default::default()
                },
            ),

            Bar::Subject => Vec2(
                Len::default(),
                Len {
                    em: 3.0,
                    ..Default::default()
                },
            ),
        }
    }

    pub fn offset(&self, _attachments: Vec<&Attachment>) -> Vec2 {
        match self {
            Bar::Complement => Vec2(
                Len {
                    ch: 2.0,
                    ..Default::default()
                },
                Len {
                    em: 1.5,
                    ..Default::default()
                },
            ),

            Bar::Object => Vec2(
                Len::default(),
                Len {
                    em: 1.5,
                    ..Default::default()
                },
            ),

            Bar::Subject => Vec2(
                Len::default(),
                Len {
                    ch: 1.5,
                    ..Default::default()
                },
            ),
        }
    }
}
