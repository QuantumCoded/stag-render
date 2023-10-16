use crate::rkpart::{RKNode, SizeVec2};

pub enum Bar {
    ClauseBar,
    ComplementBar,
    ObjectBar,
}
pub enum Attachment
{
    Origin,
}

impl RKNode for Bar {
    type Attachment = Attachment;

    fn attachment(&self, attachment: Self::Attachment) -> SizeVec2 {
        match attachment {
            Attachment::Origin => SizeVec2::default(),
        }
    }

    fn offset(&self) -> SizeVec2 {
        match self {
            Self::ClauseBar => SizeVec2 {
                height_em: 1.5,
                ..Default::default()
            },

            Self::ComplementBar => SizeVec2 {
                height_em: 1.5,
                width_ch: 1.,
                ..Default::default()
            },

            Self::ObjectBar => SizeVec2 {
                height_em: 1.5,
                ..Default::default()
            },
        }
    }

    fn size(&self) -> super::SizeVec2 {
        match self {
            Self::ClauseBar => SizeVec2 {
                height_em: 3.,
                ..Default::default()
            },

            Self::ComplementBar => SizeVec2 {
                height_em: 1.5,
                width_ch: 1.,
                ..Default::default()
            },

            Self::ObjectBar => SizeVec2 {
                height_em: 1.5,
                ..Default::default()
            },
        }
    }
}
