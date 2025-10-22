use crate::io::{
    VGA_HEIGHT,
    console::{
        Cell,
        colors::{Color, ColorPair},
        tests::buffer_test::make_console,
    },
};

#[test]
fn test_parse_csi_up_arrow() {
    let mut console = make_console();

    console.cursor.y = 5;
    console.write_string(b"\x1B[A");

    assert_eq!(console.cursor.y, 4);
    assert_eq!(console.cursor.x, 0);
}

#[test]
fn test_parse_csi_up_arrow_line_0() {
    let mut console = make_console();

    console.cursor.y = 0;
    console.write_string(b"\x1B[A");

    assert_eq!(console.cursor.y, 0);
    assert_eq!(console.cursor.x, 0);
}

#[test]
fn test_parse_csi_down_arrow_no_scroll() {
    let mut console = make_console();
    console.cursor.y = 0;

    console.write_string(b"\x1B[B");

    assert_eq!(console.cursor.y, 1, "cursor should scroll down");
    assert_eq!(console.offset, 0, "offset dont should be change here");
}

#[test]
fn test_parse_csi_down_arrow_scroll() {
    let mut console = make_console();
    console.cursor.y = VGA_HEIGHT;
    console.buffer[console.cursor.y + 1][0] = Cell::new(b'A', ColorPair::default());

    console.write_string(b"\x1B[B");

    assert_eq!(
        console.cursor.y,
        VGA_HEIGHT + 1,
        "cursor should scroll down"
    );
    assert_eq!(console.offset, 1, "offset dont should be change here");
}

#[test]
fn test_parse_csi_down_arrow_no_scroll_empty_line() {
    let mut console = make_console();
    console.cursor.y = VGA_HEIGHT;

    console.write_string(b"\x1B[B");

    assert_eq!(console.cursor.y, VGA_HEIGHT, "cursor should dont move");
    assert_eq!(console.offset, 0, "offset dont should be change here");
}

#[test]
fn test_parse_csi_left_and_right_arrow() {
    let mut console = make_console();

    console.replace_byte(b'A');
    console.cursor.x = 1;
    console.replace_byte(b'A');
    console.write_string(b"\x1B[D");
    assert_eq!(console.cursor.x, 0);

    console.write_string(b"\x1B[C");
    assert_eq!(console.cursor.x, 1);

    console.write_string(b"\x1B[D");
    console.write_string(b"\x1B[D");
    assert_eq!(console.cursor.x, 0, "dont should be under 0");
}

//------------------
//     COLORS
//------------------

#[test]
fn test_parse_csi_color_change() {
    let mut console = make_console();
    console.write_string(b"\x1B[31m");

    assert_eq!(console.color.foreground, Color::Red);
}

#[test]
fn test_parse_csi_color_reset() {
    let mut console = make_console();
    console.write_string(b"\x1B[31m");
    assert_eq!(console.color.foreground, Color::Red);
    console.write_string(b"\x1B[0m");
    assert_eq!(console.color.foreground, Color::White);
}

#[test]
fn test_parse_csi_page_up_down() {
    let mut console = make_console();

    console.offset = 5;
    console.cursor.y = 10;

    console.write_string(b"\x1B[5~");
    assert_eq!(console.offset, 4);

    console.write_string(b"\x1B[6~");
    assert_eq!(console.offset, 5);
}
