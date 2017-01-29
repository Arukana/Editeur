pub use ::tuple::Tuple;
pub use ::emotion::Emotion;
pub use ::sheet::Sheet;
pub use ::sprite::texel::part::Part;

pub fn init<T>(e: Option<T>, v: Vec<T>) -> Vec<T> {
    match e {
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}
