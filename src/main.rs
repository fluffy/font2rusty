use fontdue::Font;


fn main() {
    // Load the font data from a file
    let font_data = include_bytes!("../fonts/NotoSans-Regular.ttf") as &[u8];

    // Parse the font
    let font = Font::from_bytes(font_data, fontdue::FontSettings::default()).unwrap();

    for (c,index) in font.chars() { // Iterate over all glyphs
       if (*c > '~') || ( *c < ' ') {
           continue;
       }
        println!("Glyph {:?}: {:?}", index, c );
    }


    // Use the font (e.g., to get the metrics of a character)
    let (metrics, bitmap) = font.rasterize('g', 12.0);
    println!("Character metrics: {:?}", metrics);

    // Print the bitmap
    for y in 0..metrics.height {
        for x in 0..metrics.width {
            print!("{}", if bitmap[y * metrics.width + x] > 50 { "#" } else { " " });
        }
        println!();
    }
}
