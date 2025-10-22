use crate::io::{
    VGA_WIDTH,
    console::{Pos, tests::buffer_test::make_console},
};

#[test]
fn test_move_cursor_underflow_detected() {
    let mut console = make_console();
    console.cursor = Pos::new(0, 5);
    console.offset = 10;
    console.move_cursor();

    assert!(
        console.writer.underflow_detected,
        "Underflow not detected !"
    );
    assert!(console.writer.last_pos.is_none(), "No pos should be saved");
}

#[test]
fn test_move_cursor_normal_position() {
    let mut console = make_console();

    console.cursor = Pos::new(5, 12);
    console.offset = 10;
    console.move_cursor();

    assert!(!console.writer.underflow_detected);
    assert_eq!(
        console.writer.last_pos,
        Some(((12 - 10) * VGA_WIDTH + 5) as u16)
    );
}
