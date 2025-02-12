use fontdue::Font;

#[allow(dead_code)]
struct Glyph {
    rle_start: u16,
    c: char,
    xmin: i8,
    ymin: i8,
    width: u8,
    height: u8,
}

static _FONT_RLE_DATA: &'static [(i8, u8)] = &[(0, 0), (0, 0), (-10, 0), (3, 128)];

static _FONT_METRICS: &'static [Glyph] = &[
    Glyph {
        c: 'a',
        xmin: 0,
        ymin: 0,
        width: 10,
        height: 10,
        rle_start: 4,
    },
    Glyph {
        c: 'b',
        xmin: 0,
        ymin: 0,
        width: 10,
        height: 10,
        rle_start: 4,
    },
];

static _FONT_GLYPH_INDEX: &'static [u8] = &[0, 0, 0, 23, 22];

fn main() {
    // Load the font data from a file
    let font_data = include_bytes!("../fonts/NotoSans-Regular.ttf") as &[u8];

    println!("// Do not edit - generated with font2rusty");
    println!("// Font data for NotoSans-Regular.ttf");

    println!("struct Glyph {{ rle_start: u16,");
    println!("    c: char,xmin: i8,ymin: i8,width: u8,height: u8, }}");
    println!("");

    // Parse the font
    let font = Font::from_bytes(font_data, fontdue::FontSettings::default()).unwrap();

    println!("static FONT_RLE_DATA: &'static [(i8, u8)] = &[(0, 0),");
    let mut rle_index = 1;

    for (c, _index) in font.chars() {
        // Iterate over all glyphs
        if (*c > '~') || (*c < ' ') {
            continue;
        }
        //println!("Glyph {:?}: {:?}", index, c);

        // Use the font (e.g., to get the metrics of a character)
        let (metrics, bitmap) = font.rasterize(*c, 20.0);
        //println!("Character metrics: {:?}", metrics);

        println!("// Char {} at {}", *c, rle_index);

        // Run-length encode the bitmap
        for y in 0..metrics.height {
            let mut prev_val: u8 = 0;
            let mut count: u8 = 0;

            for x in 0..metrics.width {
                let val = bitmap[y * metrics.width + x];
                if val == prev_val {
                    count += 1;
                } else {
                    print!("({}, {}),", count, 255-prev_val);
                    rle_index += 1;

                    prev_val = val;
                    count = 1;
                }
            }

            println!("(-1, 255),");
            rle_index += 1;
        }
        println!("(0, 0),");
        rle_index += 1;
    }

    println!("];"); // end of FONT_RLE_DATA
}
