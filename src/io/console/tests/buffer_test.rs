use crate::io::console::{
    Cell, Console, ERASE_BYTE, Pos, State,
    colors::{Color, ColorPair},
    writer::mocker::MockWriter,
};

const TEST_WIDTH: usize = 80;
// const TEST_HEIGHT: usize = 3;
const TEST_HISTORY: usize = 100;

fn make_console() -> Console<MockWriter> {
    Console {
        buffer: [[Cell::blank(); TEST_WIDTH]; TEST_HISTORY],
        cursor: Pos::blank(),
        offset: 0,
        writer: MockWriter {},
        color: ColorPair {
            foreground: Color::White,
            background: Color::Black,
        },
        state: State::Default,
    }
}

#[test]
fn test_store_byte_advances_cursor() {
    let mut console = make_console();
    console.store_byte(b'A');
    assert_eq!(console.buffer[0][0].byte, b'A');
    assert_eq!(console.cursor.x, 1);
    assert_eq!(console.cursor.y, 0);

    console.store_byte(b'B');
    assert_eq!(console.buffer[0][1].byte, b'B');
    assert_eq!(console.cursor.x, 2);
    assert_eq!(console.cursor.y, 0);
}

#[test]
fn test_newline_resets_x() {
    let mut console = make_console();
    console.cursor.x = 3;
    console.cursor.y = 1;
    console.nl();
    assert_eq!(console.cursor.x, 0);
    assert_eq!(console.cursor.y, 2);
}

#[test]
fn test_backspace() {
    let mut console = make_console();
    console.store_byte(b'A');
    console.store_byte(b'B');
    console.back_space();
    // doit effacer 'B'
    assert_eq!(console.buffer[0][1].byte, ERASE_BYTE);
    assert_eq!(console.cursor.x, 1);
}
