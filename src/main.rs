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

    let mut glyphs: [Glyph; 128] = [Glyph {
        rle_start: 0,
        c: '\0',
        xmin: 0,
        ymin: 0,
        width: 0,
        height: 0,
    }; 128];

    println!("static FONT_RLE_DATA: &'static [(i8, u8)] = &[(0, 0),");
    let mut rle_index = 1;

    for (c, _index) in font.chars() {
        // Iterate over all glyphs
        if (*c > '~') || (*c < ' ') {
            continue;
        }

        // todo rmeove
        if true {
            if (*c > '2') || (*c < '1') {
                continue;
            }
        }
    //println!("Glyph {:?}: {:?}", index, c);

        // Use the font (e.g., to get the metrics of a character)
        let (metrics, bitmap) = font.rasterize(*c, 20.0);
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

    let mut glyph_index: [u8; 128] = [0; 128];

    println!("static FONT_METRICS: &'static [Glyph] = &[");
    let mut index = 0;
    for g in glyphs.iter() {
        if g.rle_start != 0 {

            let mut s = (g.c).to_string();
            if s.as_str() == "\\" {
                s = String::from( "\\\\" );
            }
            if s.as_str() ==  "'"{
                s = String::from( "\\'" );
            }

            println!(
                "Glyph {{ rle_start: {}, c: '{}', xmin: {}, ymin: {}, width: {}, height: {} }},",
                g.rle_start, s , g.xmin, g.ymin, g.width, g.height
            );
            glyph_index[g.c as usize] = index as u8;
            index += 1;
        }
    }
    println!("];");
    println!("");

    println!("static FONT_GLYPH_INDEX: &'static [u8] = &[");

    for i in 0..128 {
     println!("{},", glyph_index[i]);
    }
    println!("];");
}
