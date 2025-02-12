mod font;

#[test]
fn test_digit1() {
    assert_ne!(font::GLYPH_INDEX['1' as usize], 0xFF);

    let index = font::GLYPH_INDEX['1' as usize] as usize;
    assert_eq!(font::GLYPH_METRICS[index].c, '1');
}


#[test]
fn test_rle() {
    let index = font::GLYPH_INDEX['a' as usize] as usize;
    let glyph = &font::GLYPH_METRICS[index];

    let rle_start = glyph.rle_start as usize;
    let rle_end = font::RLE_DATA.len();

    let width = glyph.width as i32;

    let mut rle_index = rle_start;
    let mut row: i32 = 0;
    let mut col : i32 = 0;

    let max_row = font::METRICS.ascent as i32 - font::METRICS.descent as i32;
    let max_cal = glyph.width as i32;

    assert!( font::METRICS.min_width <= font::METRICS.max_width);
    assert!( font::METRICS.rle_bytes > 0 );
    assert!( font::METRICS.line_gap <= 2 );

    assert!( glyph.xmin <= 5 );
    assert!( glyph.xmin >= 0 );

    let ymax = glyph.ymin + glyph.height as i8;

    for _  in ymax ..  font::METRICS.ascent as i8 {
        println!("_");
        row += 1;
        assert!( row <= max_row);
    }

    while rle_index < rle_end {
        let (count,val) = font::RLE_DATA[rle_index];
        if count == 0 && val == 0 {
            break;
        }
        if col == 0 {
            for _ in 0 .. glyph.xmin {
                print!("_");
                col += 1;
                assert!( col <= max_cal);
            }
        }
        if count == -1 {
            for _ in col ..width {
                print!("_");
                col += 1;
                assert!( col <= max_cal);
            }
            println!("");
            row += 1;
            assert!( row <= max_row);
            col = 0;
        }
        if  count > 0 {
            for _ in 0..count {
                if val > 128 {
                    print!("_");
                    col += 1;
                    assert!( col <= max_cal);
                } else {
                    print!("*");
                    col += 1;
                    assert!( col <= max_cal);
                }
            }
        }

        rle_index += 1;
    }

    for _ in font::METRICS.descent ..  glyph.ymin {
        println!("_");
        row += 1;
        assert!( row <= max_row);
    }
}