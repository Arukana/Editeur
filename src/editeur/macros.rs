#[macro_export]
#[cfg(not(feature = "clipboard"))]
macro_rules! editeur_new {
    ($graphic: expr, $output: expr) => ({
        use std::io;
        Editeur {
            graphic: $graphic,
            output: $output,
            input: io::stdin().events(),
            menu: Menu::default(),
        }
    });
}
