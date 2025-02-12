mod font;

#[test]
fn test_digit1() {
    assert_ne!(font::GLYPH_INDEX['1' as usize], 0xFF);

    let index = font::GLYPH_INDEX['1' as usize] as usize;
    assert_eq!(font::GLYPH_METRICS[index].c, '1');
}
