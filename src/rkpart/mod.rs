use std::ops::Deref;

use petgraph::{self, Graph};

pub mod bar;
pub mod word;

pub trait RKNode {
    type Attachment;

    fn attachment(&self, attachment: Self::Attachment) -> SizeVec2;
    fn offset(&self) -> SizeVec2;
    fn size(&self) -> SizeVec2;
}

#[derive(Default)]
pub struct SizeVec2 {
    height_ch: f32,
    height_em: f32,
    height_px: f32,
    width_ch: f32,
    width_em: f32,
    width_px: f32,
}

impl SizeVec2 {
    fn as_px_tuple(&self, font_size: f32, font_ratio: f32) -> (f32, f32) {
        (
            self.width_ch * font_size * font_ratio + self.width_em * font_size + self.width_px,
            self.height_ch * font_size * font_ratio + self.width_em * font_size + self.width_px,
        )
    }

    fn to_px_tuple(self, font_size: f32, font_ratio: f32) -> (f32, f32) {
        (
            self.width_ch * font_size * font_ratio + self.width_em * font_size + self.width_px,
            self.height_ch * font_size * font_ratio + self.width_em * font_size + self.width_px,
        )
    }
}

pub enum RKPart {
    Bar(bar::Bar),
    Word(word::Word),
}

impl RKPart {
    
}

pub type RKGraph = Graph<RKPart, SizeVec2>;
