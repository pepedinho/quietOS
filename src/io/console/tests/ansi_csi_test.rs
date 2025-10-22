use crate::io::{
    VGA_HEIGHT,
    console::{
        CONSOLE_HISTORY, CSI, Cell, State,
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

#[test]
fn test_parse_csi_long_number() {
    let mut console = make_console();
    console.write_string(b"\x1B[123m");

    // 123 is not a supported color, nothing should be broken
    assert_eq!(console.color, ColorPair::default());
    assert_eq!(console.state, State::Default);
}

#[test]
fn test_incomplete_escape_sequence_does_not_panic() {
    let mut console = make_console();

    console.write_string(b"\x1B[");

    // State should be CSI(None), without panic
    match console.state {
        State::CSI(CSI::None) => {}
        _ => panic!("State not stay on CSI::None"),
    }
}

#[test]
fn test_cursor_position_after_scroll() {
    let mut console = make_console();
    console.offset = CONSOLE_HISTORY - VGA_HEIGHT - 1;
    console.cursor.y = VGA_HEIGHT - 1;

    console.scroll_offset_down();

    assert_eq!(console.offset, CONSOLE_HISTORY - VGA_HEIGHT);
    assert!(console.cursor.y >= console.offset);
}

#[test]
fn test_tab_inserts_spaces() {
    let mut console = make_console();
    console.write_string(b"\t");

    assert_eq!(console.buffer[0][0].byte, b' ');
    assert_eq!(console.buffer[0][3].byte, b' ');
    assert_eq!(console.cursor.x, 4);
}
