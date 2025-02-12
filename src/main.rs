use fontdue::Font;

#[derive(Copy, Clone)]
struct Glyph {
    rle_start: u16,
    c: char,
    xmin: i8,
    ymin: i8,
    width: u8,
    height: u8,
}

fn main() {
    // Load the font data from a file
    let font_data = include_bytes!("../fonts/NotoSans-Regular.ttf") as &[u8];
    const FONT_SIZE: f32 = 20.0;

    println!("// Do not edit - generated with font2rusty");
    println!("// Font data for NotoSans-Regular.ttf at {}pt", FONT_SIZE);
    println!("");

    println!("pub struct Glyph {{ pub rle_start: u16,");
    println!("    pub c: char,pub xmin: i8, pub ymin: i8, pub width: u8, pub height: u8, }}");
    println!("");

    // Parse the font
    let font = Font::from_bytes(font_data, fontdue::FontSettings::default()).unwrap();

    let mut glyphs: [Glyph; 128] = [Glyph {
        rle_start: 0,
        c: '\0',
        xmin: 0,
        ymin: 0,
        width: 0,
        height: 0,
    }; 128];

    println!("pub static RLE_DATA: &'static [(i8, u8)] = &[(0, 0),");
    let mut rle_index = 1;

    let mut min_width = 255u8;
    let mut max_width = 0u8;

    for (c, _index) in font.chars() {
        // Iterate over all glyphs
        if (*c > '~') || (*c < ' ') {
            continue;
        }

        if true {
            if (*c > '2') || (*c < '1') {
                continue;
            }
        }
        //println!("Glyph {:?}: {:?}", index, c);

        let (metrics, bitmap) = font.rasterize(*c, FONT_SIZE);
        //println!("Character metrics: {:?}", metrics);

        let glyph_index = *c as usize;
        assert!(glyph_index < 128);

        let g: Glyph = Glyph {
            rle_start: rle_index,
            c: *c,
            xmin: metrics.xmin as i8,
            ymin: metrics.ymin as i8,
            width: metrics.width as u8,
            height: metrics.height as u8,
        };

        glyphs[glyph_index] = g;

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
                    print!("({}, {}),", count, 255 - prev_val);
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
    println!("");

    let mut glyph_index: [u8; 128] = [0xFF; 128];

    println!("pub static GLYPH_METRICS: &'static [Glyph] = &[");
    let mut index = 0;
    for g in glyphs.iter() {
        if g.rle_start != 0 {
            let mut s = (g.c).to_string();
            if s.as_str() == "\\" {
                s = String::from("\\\\");
            }
            if s.as_str() == "'" {
                s = String::from("\\'");
            }

            println!(
                "Glyph {{ rle_start: {}, c: '{}', xmin: {}, ymin: {}, width: {}, height: {} }},",
                g.rle_start, s, g.xmin, g.ymin, g.width, g.height
            );
            glyph_index[g.c as usize] = index as u8;
            index += 1;

            if g.width < min_width {
                min_width = g.width;
            }
            if g.width > max_width {
                max_width = g.width;
            }
        }
    }
    println!("];");
    println!("");

    println!("pub static GLYPH_INDEX: &'static [u8] = &[");

    for i in 0..128 {
        println!("{},", glyph_index[i]);
    }
    println!("];");
    println!("");

    println!("pub struct Metrics {{");
    println!("    pub ascent: u8, pub descent: i8, pub line_gap: u8, pub min_width: u8, pub max_width: u8 }}");
    println!("");
    let line_metrics = font.horizontal_line_metrics(FONT_SIZE).unwrap();
    println!(
        "pub static METRICS: &'static Metrics = &Metrics {{ \
        ascent: {}, descent: {}, line_gap: {}, min_width: {}, max_width: {} }};",
        line_metrics.ascent.ceil(),
        line_metrics.descent.floor(),
        line_metrics.line_gap.ceil(),
        min_width,
        max_width
    );
}
