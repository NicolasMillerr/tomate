use ratatui::text::Line;

const ROWS: usize = 5;

#[rustfmt::skip]
pub const COLON: &[&str] = &[
    "   ",
    " █ ",
    "   ",
    " █ ",
    "   ",
];

#[rustfmt::skip]
pub const ZERO: &[&str] = &[
    " ███ ",
    "█   █",
    "█   █",
    "█   █",
    " ███ ",
];

#[rustfmt::skip]
pub const ONE: &[&str] = &[
    "  █  ",
    " ██  ",
    "  █  ",
    "  █  ",
    " ███ ",
];

#[rustfmt::skip]
pub const TWO: &[&str] = &[
    " ███ ",
    "    █",
    " ███ ",
    "█    ",
    " ███ ",
];

#[rustfmt::skip]
pub const THREE: &[&str] = &[
    " ███ ",
    "    █",
    " ███ ",
    "    █",
    " ███ ",
];

#[rustfmt::skip]
pub const FOUR: &[&str] = &[
    "█   █",
    "█   █",
    " ███ ",
    "    █",
    "    █",
];

#[rustfmt::skip]
pub const FIVE: &[&str] = &[
    " ███ ",
    "█    ",
    " ███ ",
    "    █",
    " ███ ",
];

#[rustfmt::skip]
pub const SIX: &[&str] = &[
    " ███ ",
    "█    ",
    " ███ ",
    "█   █",
    " ███ ",
];

#[rustfmt::skip]
pub const SEVEN: &[&str] = &[
    " ███ ",
    "    █",
    "   █ ",
    "  █  ",
    " ███ ",
];

#[rustfmt::skip]
pub const EIGHT: &[&str] = &[
    " ███ ",
    "█   █",
    " ███ ",
    "█   █",
    " ███ ",
];

#[rustfmt::skip]
pub const NINE: &[&str] = &[
    " ███ ",
    "█   █",
    " ███ ",
    "    █",
    " ███ ",
];

const NUMBER_MAP: [&[&str]; 10] = [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

pub fn draw_time<'a>(m1: u64, m2: u64, s1: u64, s2: u64) -> Vec<Line<'a>> {
    let mut lines = Vec::with_capacity(ROWS);
    for i in 0..(ROWS) {
        lines.push(Line::from(format!(
            "{} {} {} {} {}",
            NUMBER_MAP[m1 as usize][i],
            NUMBER_MAP[m2 as usize][i],
            COLON[i],
            NUMBER_MAP[s1 as usize][i],
            NUMBER_MAP[s2 as usize][i]
        )));
    }
    lines
}
