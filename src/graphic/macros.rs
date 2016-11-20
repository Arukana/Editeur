
#[macro_export]
#[cfg(feature = "clipboard")]
macro_rules! editeur_new {
    ($graphic: expr, $output: expr) => ({
        use std::io;
        use ::clipboard::ClipboardContext;
        Editeur {
            graphic: $graphic,
            output: $output,
            input: io::stdin().events(),
            kopimism: ClipboardContext::new().unwrap(),
            menu: Menu::default(),
        }
    });
}

#[macro_export]
macro_rules! format_cell {
    ($cell: expr, $target: expr, $x: expr, $y: expr) => ({
        if $target.eq(&($x, $y)) {
            format!("{}\u{0332}", $cell)
        } else {
            format!("{}", $cell)
        }
    });
    ($cell: expr, $target: expr, $part_by_emotion: expr) => ({
        if $target.eq(&$part_by_emotion) {
            format!("[{}]", $cell)
        } else {
            format!("{}", $cell)
        }
    });
}
