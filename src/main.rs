use fontdue::Font;

#[allow(dead_code)]
struct Glyph {
    rle_start: u16,
    rle_len: u8,
    c: char,
    xmin: i8,
    ymin: i8,
    width: u8,
    height: u8,
}

static _FONT_RLE_DATA: &'static [(i8, u8)] = &[(1, 255), (0, 0), (-10, 0), (3, 128)];

static _FONT_METRICS: &'static [Glyph] = &[
    Glyph {
        c: 'a',
        xmin: 0,
        ymin: 0,
        width: 10,
        height: 10,
        rle_start: 4,
        rle_len: 3,
    },
    Glyph {
        c: 'b',
        xmin: 0,
        ymin: 0,
        width: 10,
        height: 10,
        rle_start: 4,
        rle_len: 3,
    },
];

static _FONT_GLYPH_INDEX: &'static [u8] = &[0, 0, 0, 23, 22];

fn main() {
    // Load the font data from a file
    let font_data = include_bytes!("../fonts/NotoSans-Regular.ttf") as &[u8];

    // Parse the font
    let font = Font::from_bytes(font_data, fontdue::FontSettings::default()).unwrap();

    for (c, index) in font.chars() {
        // Iterate over all glyphs
        if (*c > '~') || (*c < ' ') {
            continue;
        }
        println!("Glyph {:?}: {:?}", index, c);
    }

    // Use the font (e.g., to get the metrics of a character)
    let (metrics, bitmap) = font.rasterize('g', 12.0);
    println!("Character metrics: {:?}", metrics);

    // Print the bitmap
    for y in 0..metrics.height {
        for x in 0..metrics.width {
            print!(
                "{}",
                if bitmap[y * metrics.width + x] > 50 {
                    "#"
                } else {
                    " "
                }
            );
        }
        println!();
    }
}
