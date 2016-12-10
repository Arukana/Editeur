#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Tuple {
    pub part: super::sprite::texel::part::Part,
    pub emotion: super::emotion::Emotion,
}
