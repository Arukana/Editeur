use super::sprite::texel::part::Part;
use super::emotion::Emotion;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Tuple {
    pub part: Part,
    pub emotion: Emotion,
}

impl From<(Part, Emotion)> for Tuple {

    /// The constructor `from` returns a tuple of Part and Emotion
    /// according to the argumentation.
    fn from((part, emotion): (Part, Emotion)) -> Self {
        Tuple {
            part: part,
            emotion: emotion,
        }
    }
}
