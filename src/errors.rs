use ariadne::{Cache, Color, Label, Report, ReportKind, Source};

// To see about ParsingErrors:
use super::parser::error::AspenError;

// To see about evaluation errors:
use super::evaluate::error::EvaluateError;

pub fn build_error(source: &str, err: AspenError, file_name: &str) {
    let err_string = err.to_string();
    let (message, note, offset, length, code) = match err {
        AspenError::IoError(_e) => {
            return;
        }
        AspenError::Lexing { start, length, .. } => (
            &err_string,
            "Remove unsupported or invalid ASCII Chars".to_owned(),
            start,
            length,
            0,
        ),
        AspenError::Eof => (
            &err_string,
            "add a missing <expr> or statement end".to_owned(),
            source.len() - 1,
            0,
            1,
        ),
        AspenError::Evaluate {
            start,
            length,
            note,
            ..
        } => (&err_string, note, start, length, 6),
        AspenError::Expected { start, length, .. } => (
            &err_string,
            "Add what's expected, it's not difficult, right ?".to_owned(),
            start,
            length,
            2,
        ),
        AspenError::ExpectedSpace { start, length, .. } => (
            &err_string,
            "Add a space at the indicated offset".to_owned(),
            start,
            length,
            4,
        ),
        AspenError::ExpectedNewline { start, length, .. } => (
            &err_string,
            "Add a newline at the indicated offset".to_owned(),
            start,
            length,
            5,
        ),
        AspenError::Unknown { start, length, .. } => (
            &err_string,
            "Just remove the unexpected token!".to_owned(),
            start,
            length,
            3,
        ),
    };

    let red = Color::Red;

    Report::build(ReportKind::Error, file_name, offset)
        .with_code(code)
        .with_message(message)
        .with_label(Label::new((file_name, offset..offset + length)).with_color(red))
        .with_help(note)
        .finish()
        .print((file_name, Source::from(source)))
        .unwrap();
}
