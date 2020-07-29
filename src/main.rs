use pancurses::{endwin, initscr, noecho, raw, Input::*, A_BOLD};

mod scrollback_buffer;
use scrollback_buffer::ScrollbackBuffer;

fn main() {
    let window = initscr();
    window.keypad(true);
    raw();
    noecho();

    let height = window.get_max_y() as usize;
    let mut buffer = ScrollbackBuffer::new(height - 2);

    window.attron(A_BOLD);
    window.printw("Press ^C to quit.\n");
    window.attroff(A_BOLD);

    while let Some(input) = window.getch() {
        if input == Character('\u{3}') {
            break;
        }
        // Push new line onto buffer
        buffer.push_back(format!("{:?}\n", input));
        // Reprint the whole buffer
        window.mv(1, 0);
        for line in buffer.iter() {
            window.printw(line);
        }
    }

    endwin();
}
