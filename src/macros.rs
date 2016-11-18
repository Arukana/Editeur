#[macro_export]
macro_rules! format_cell {
    ($cell: expr, $target: expr, $x: expr, $y: expr) => ({
        use ::termion::color;
        if $target.eq(&($x, $y)) {
            format!("{}{}{}",
                    color::Bg(color::Cyan),
                    $cell,
                    color::Bg(color::Reset))
        } else {
            format!("{}", $cell)
        }
    });
    ($cell: expr, $target: expr, $part_by_emotion: expr) => ({
        use ::termion::color;
        if $target.eq(&$part_by_emotion) {
            format!("{}{}{}",
                    color::Bg(color::Cyan),
                    $cell,
                    color::Bg(color::Reset))
        } else {
            format!("{}", $cell)
        }
    });
}
