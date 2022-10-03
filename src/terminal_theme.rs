#![allow(dead_code)]

use lazy_static::lazy_static;

use crate::color_triplet::ColorTriplet;
use crate::palette::Palette;

/// A color theme used when exporting console content.
#[derive(Debug)]
pub(crate) struct TerminalTheme {
    pub(crate) background_color: ColorTriplet,
    pub(crate) foreground_color: ColorTriplet,
    pub(crate) ansi_colors: Palette<16>,
}

impl TerminalTheme {
    /// Get a new [`TerminalTheme`].
    ///
    /// # Arguments
    /// * `background` - The background color.
    /// * `foreground` - The foreground (text) color.
    /// * `normal` - A list of 8 normal intensity colors.
    /// * `bright`- A list of 8 bright colors, or none to repeat normal
    ///   intensity.
    pub(crate) fn new(
        background_color: (u8, u8, u8),
        foreground_color: (u8, u8, u8),
        mut normal: Vec<(u8, u8, u8)>,
        bright: Option<Vec<(u8, u8, u8)>>,
    ) -> Self {
        normal.append(&mut bright.unwrap_or_else(|| normal.clone()));

        Self {
            background_color: ColorTriplet::new(
                background_color.0,
                background_color.1,
                background_color.2,
            ),
            foreground_color: ColorTriplet::new(
                foreground_color.0,
                foreground_color.1,
                foreground_color.2,
            ),
            ansi_colors: Palette::new(normal.try_into().unwrap()),
        }
    }
}

lazy_static! {
    static ref DEFAULT_TERMINAL_THEME: TerminalTheme = TerminalTheme::new(
        (255, 255, 255),
        (0, 0, 0),
        vec![
            (0, 0, 0),
            (128, 0, 0),
            (0, 128, 0),
            (128, 128, 0),
            (0, 0, 128),
            (128, 0, 128),
            (0, 128, 128),
            (192, 192, 192),
        ],
        Some(vec![
            (128, 128, 128),
            (255, 0, 0),
            (0, 255, 0),
            (255, 255, 0),
            (0, 0, 255),
            (255, 0, 255),
            (0, 255, 255),
            (255, 255, 255),
        ]),
    );
}

lazy_static! {
    static ref MONOKAI: TerminalTheme = TerminalTheme::new(
        (12, 12, 12),
        (217, 217, 217),
        vec![
            (26, 26, 26),
            (244, 0, 95),
            (152, 224, 36),
            (253, 151, 31),
            (157, 101, 255),
            (244, 0, 95),
            (88, 209, 235),
            (196, 197, 181),
            (98, 94, 76),
        ],
        Some(vec![
            (244, 0, 95),
            (152, 224, 36),
            (224, 213, 97),
            (157, 101, 255),
            (244, 0, 95),
            (88, 209, 235),
            (246, 246, 239),
        ]),
    );
}

lazy_static! {
    static ref DIMMED_MONOKAI: TerminalTheme = TerminalTheme::new(
        (25, 25, 25),
        (185, 188, 186),
        vec![
            (58, 61, 67),
            (190, 63, 72),
            (135, 154, 59),
            (197, 166, 53),
            (79, 118, 161),
            (133, 92, 141),
            (87, 143, 164),
            (185, 188, 186),
            (136, 137, 135),
        ],
        Some(vec![
            (251, 0, 31),
            (15, 114, 47),
            (196, 112, 51),
            (24, 109, 227),
            (251, 0, 103),
            (46, 112, 109),
            (253, 255, 185),
        ]),
    );
}

lazy_static! {
    static ref NIGHT_OWLISH: TerminalTheme = TerminalTheme::new(
        (255, 255, 255),
        (64, 63, 83),
        vec![
            (1, 22, 39),
            (211, 66, 62),
            (42, 162, 152),
            (218, 170, 1),
            (72, 118, 214),
            (64, 63, 83),
            (8, 145, 106),
            (122, 129, 129),
            (122, 129, 129),
        ],
        Some(vec![
            (247, 110, 110),
            (73, 208, 197),
            (218, 194, 107),
            (92, 167, 228),
            (105, 112, 152),
            (0, 201, 144),
            (152, 159, 177),
        ]),
    );
}

lazy_static! {
    static ref DRACULA: TerminalTheme = TerminalTheme::new(
        (40, 42, 54),
        (248, 248, 242),
        vec![
            (33, 34, 44),
            (255, 85, 85),
            (152, 224, 36),
            (253, 151, 31),
            (157, 101, 255),
            (244, 0, 95),
            (88, 209, 235),
            (196, 197, 181),
        ],
        Some(vec![
            (98, 114, 164),
            (255, 110, 110),
            (105, 255, 148),
            (255, 255, 165),
            (214, 172, 255),
            (255, 146, 223),
            (164, 255, 255),
            (255, 255, 255),
        ]),
    );
}

lazy_static! {
    static ref MATERIAL_OCEAN: TerminalTheme = TerminalTheme::new(
        (38, 50, 56),
        (176, 190, 197),
        vec![
            (0, 0, 0),
            (220, 96, 104),
            (171, 207, 118),
            (230, 180, 85),
            (110, 152, 235),
            (180, 128, 214),
            (113, 198, 231),
            (238, 255, 255),
        ],
        Some(vec![
            (0, 0, 0),
            (240, 113, 120),
            (195, 232, 141),
            (255, 203, 107),
            (130, 170, 255),
            (199, 146, 234),
            (137, 221, 255),
            (238, 255, 255),
        ]),
    );
}

lazy_static! {
    static ref MATERIAL_DEEP_OCEAN: TerminalTheme = TerminalTheme::new(
        (9, 11, 16),
        (166, 172, 205),
        vec![
            (0, 0, 0),
            (220, 96, 104),
            (171, 207, 118),
            (230, 180, 85),
            (110, 152, 235),
            (180, 128, 214),
            (113, 198, 231),
            (238, 255, 255),
        ],
        Some(vec![
            (0, 0, 0),
            (240, 113, 120),
            (195, 232, 141),
            (255, 203, 107),
            (130, 170, 255),
            (199, 146, 234),
            (137, 221, 255),
            (238, 255, 255),
        ]),
    );
}

lazy_static! {
    static ref MATERIAL_PALENIGHT: TerminalTheme = TerminalTheme::new(
        (41, 45, 62),
        (166, 172, 205),
        vec![
            (0, 0, 0),
            (220, 96, 104),
            (171, 207, 118),
            (230, 180, 85),
            (110, 152, 235),
            (180, 128, 214),
            (113, 198, 231),
            (238, 255, 255),
        ],
        Some(vec![
            (0, 0, 0),
            (240, 113, 120),
            (195, 232, 141),
            (255, 203, 107),
            (130, 170, 255),
            (199, 146, 234),
            (137, 221, 255),
            (238, 255, 255),
        ]),
    );
}

lazy_static! {
    static ref MATERIAL_LIGHTER: TerminalTheme = TerminalTheme::new(
        (250, 250, 250),
        (84, 110, 122),
        vec![
            (0, 0, 0),
            (220, 96, 104),
            (171, 207, 118),
            (230, 180, 85),
            (110, 152, 235),
            (180, 128, 214),
            (113, 198, 231),
            (255, 255, 255),
        ],
        Some(vec![
            (0, 0, 0),
            (229, 57, 53),
            (145, 184, 89),
            (246, 164, 52),
            (97, 130, 184),
            (124, 77, 255),
            (57, 173, 181),
            (255, 255, 255),
        ]),
    );
}

lazy_static! {
    static ref MATERIAL_DARKER: TerminalTheme = TerminalTheme::new(
        (33, 33, 33),
        (176, 190, 197),
        vec![
            (0, 0, 0),
            (220, 96, 104),
            (171, 207, 118),
            (230, 180, 85),
            (110, 152, 235),
            (180, 128, 214),
            (113, 198, 231),
            (238, 255, 255),
        ],
        Some(vec![
            (0, 0, 0),
            (240, 113, 120),
            (195, 232, 141),
            (255, 203, 107),
            (130, 170, 255),
            (199, 146, 234),
            (137, 221, 255),
            (238, 255, 255),
        ]),
    );
}

lazy_static! {
    static ref SVG_EXPORT_THEME: TerminalTheme = TerminalTheme::new(
        (41, 41, 41),
        (197, 200, 198),
        vec![
            (75, 78, 85),
            (204, 85, 90),
            (152, 168, 75),
            (208, 179, 68),
            (96, 138, 177),
            (152, 114, 159),
            (104, 160, 179),
            (197, 200, 198),
            (154, 155, 153),
        ],
        Some(vec![
            (255, 38, 39),
            (0, 130, 61),
            (208, 132, 66),
            (25, 132, 233),
            (255, 44, 122),
            (57, 130, 128),
            (253, 253, 197),
        ]),
    );
}
