#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Tuple {
    part: super::sprite::texel::part::Part,
    emotion: super::emotion::Emotion,
}
