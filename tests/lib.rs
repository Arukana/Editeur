extern crate lalrpop_util;
extern crate graphic;

use graphic::input;
use graphic::util::{Part, Emotion, Posture, Tuple}; // synthesized by LALRPOP

#[test]
fn test_texel() {
    assert_eq!(input::parse_Texels(&"None('s'):None [Talk, Talk]"
                   .to_string()),
               Ok(vec![(Part::None,
                        "s",
                        Emotion::None,
                        vec![Posture::Talk, Posture::Talk])]));
    assert_eq!(input::parse_Texels(&"None(\"sa\"):None [Talk, Talk]"
                   .to_string()),
               Ok(vec![(Part::None,
                        "sa",
                        Emotion::None,
                        vec![Posture::Talk, Posture::Talk])]));
    assert_eq!(input::parse_Texels(&"None(\"sa\"):None [Talk]".to_string()),
               Ok(vec![(Part::None,
                        "sa",
                        Emotion::None,
                        vec![Posture::Talk])]));
}

#[test]
fn test_sprite() {
    let l: Result<Vec<(Posture, i64, Vec<Tuple>)>, lalrpop_util::ParseError<usize, (usize, &str), ()>> = input::parse_Draws(
        "Talk 200\n\
        Heart:None Heart:None Heart:None None:None None:None None:None None:None None:None None:None None:None \n\
        Heart:None Heart:None Heart:None None:None None:None None:None None:None None:None None:None None:None \n\
        None:None None:None None:None None:None Mouth:None None:None None:None None:None None:None None:None \n\
        None:None None:None None:None None:None None:None None:None None:None None:None None:None None:None \n\
        None:None None:None None:None None:None None:None None:None None:None None:None None:None None:None ;"
    );
    let r: Result<Vec<(Posture, i64, Vec<Tuple>)>, lalrpop_util::ParseError<usize, (usize, &str), ()>> = Ok(vec![(
            Posture::Talk,
            200,
            vec![Tuple { part: Part::Heart, emotion: Emotion::None }, Tuple { part: Part::Heart, emotion: Emotion::None }, Tuple { part: Part::Heart, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::Heart, emotion: Emotion::None }, Tuple { part: Part::Heart, emotion: Emotion::None }, Tuple { part: Part::Heart, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::Mouth, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }, Tuple { part: Part::None, emotion: Emotion::None }
            ]
        )]
    );
    assert!(l.unwrap().iter().zip(r.unwrap().iter()).all(|(&(l_posture, l_duration, ref l), &(r_posture, r_duration, ref r))| {
        assert_eq!(l_posture, r_posture);
        assert_eq!(l_duration, r_duration);
        l.iter().zip(r.iter()).all(|(&l_tuple, &r_tuple)| {
            assert_eq!(l_tuple, r_tuple);
            true
        })
    }));
}
