// Text editing helpers for prompt input.

pub(crate) fn insert_char(buffer: &mut String, cursor: &mut usize, ch: char) {
    buffer.insert(*cursor, ch);
    *cursor += ch.len_utf8();
}

pub(crate) fn delete_prev_char(buffer: &mut String, cursor: &mut usize) {
    if *cursor == 0 {
        return;
    }
    if let Some((index, ch)) = buffer[..*cursor].char_indices().last() {
        buffer.drain(index..index + ch.len_utf8());
        *cursor = index;
    }
}

pub(crate) fn delete_next_char(buffer: &mut String, cursor: &mut usize) {
    if *cursor >= buffer.len() {
        return;
    }
    if let Some(ch) = buffer[*cursor..].chars().next() {
        buffer.drain(*cursor..*cursor + ch.len_utf8());
    }
}

pub(crate) fn move_cursor_left(buffer: &str, cursor: &mut usize) {
    if *cursor == 0 {
        return;
    }
    if let Some((index, _)) = buffer[..*cursor].char_indices().last() {
        *cursor = index;
    }
}

pub(crate) fn move_cursor_right(buffer: &str, cursor: &mut usize) {
    if *cursor >= buffer.len() {
        return;
    }
    if let Some(ch) = buffer[*cursor..].chars().next() {
        *cursor += ch.len_utf8();
    }
}
