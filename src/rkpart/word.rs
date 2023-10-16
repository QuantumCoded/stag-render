use super::{RKNode, SizeVec2};
use std::f32::consts::FRAC_1_SQRT_2;

pub enum Word {
    DiagWord(String),
    LeftWord(String),
    RightWord(String),
}

pub enum Attachment {
    Left,
    Right,
    Under,
}

impl RKNode for Word {
    type Attachment = Attachment;

    fn attachment(&self, attachment: Self::Attachment) -> SizeVec2 {
        match (self, attachment) {
            (Self::DiagWord(_), Self::Attachment::Left) => SizeVec2::default(),
            (Self::DiagWord(word), Self::Attachment::Right) => {
                let dist = FRAC_1_SQRT_2 * (word.len() + 4) as f32;
                SizeVec2 {
                    height_ch: dist,
                    width_ch: dist,
                    ..Default::default()
                }
            }
            (Self::DiagWord(_), Self::Attachment::Under) => {
                let dist = FRAC_1_SQRT_2 * 2.;
                SizeVec2 {
                    height_ch: dist,
                    width_ch: dist,
                    ..Default::default()
                }
            }

            (Self::LeftWord(word), Self::Attachment::Left) => SizeVec2 {
                width_ch: -((word.len() + 4) as f32),
                ..Default::default()
            },
            (Self::LeftWord(_), Self::Attachment::Right) => SizeVec2::default(),
            (Self::LeftWord(word), Self::Attachment::Under) => SizeVec2 {
                width_ch: -((word.len() + 2) as f32),
                ..Default::default()
            },

            (Self::RightWord(_), Self::Attachment::Left) => SizeVec2::default(),
            (Self::RightWord(word), Self::Attachment::Right) => SizeVec2 {
                width_ch: (word.len() + 2) as f32,
                ..Default::default()
            },
            (Self::RightWord(_), Self::Attachment::Under) => SizeVec2 {
                width_ch: 2.,
                ..Default::default()
            },
        }
    }

    fn offset(&self) -> SizeVec2 {
        match self {
            Self::DiagWord(_) => SizeVec2::default(),
            Self::LeftWord(word) => SizeVec2 {
                height_em: 1.5,
                width_ch: (word.len() + 4) as f32,
                ..Default::default()
            },
            Self::RightWord(_) => SizeVec2 {
                height_em: 1.5,
                ..Default::default()
            },
        }
    }

    fn size(&self) -> SizeVec2 {
        match self {
            Self::DiagWord(word) => {
                let dist = FRAC_1_SQRT_2 * (word.len() + 4) as f32;
                SizeVec2 {
                    height_ch: dist,
                    width_ch: dist,
                    ..Default::default()
                }
            }

            Self::LeftWord(word) | Self::RightWord(word) => SizeVec2 {
                height_em: 1.5,
                width_ch: (word.len() + 4) as f32,
                ..Default::default()
            },
        }
    }
}
