use std::fmt;
use crate::common::span::Span;

/// Represents a note attached to a Syntax error,
/// i.e. a location in source code with an optional
/// specific hint or tip.
#[derive(Debug, PartialEq, Eq)]
pub struct Note {
    span: Span,
    hint: Option<String>,
}

/// Represents a static error (syntax, semantics, etc.) found at compile time.
/// Ideally, each note included should have a distinct `Span` and hint.
/// Usually, one `Note` for an error is enough.
#[derive(Debug, PartialEq, Eq)]
pub struct Syntax {
    pub reason: String,
    pub notes:  Vec<Note>,
}

impl Syntax {
    /// Creates a new static error, with
    pub fn error(reason: &str, span: &Span) -> Syntax {
        Syntax::error_with_note(reason, Note { span: span.clone(), hint: None })
    }

    /// Creates a new static error, but with an added hint.
    pub fn error_with_note(reason: &str, note: Note) -> Syntax {
        Syntax {
            reason: reason.to_string(),
            notes:  vec![note],
        }
    }

    pub fn add_note(&mut self, note: Note) {
        self.notes.push(note)
    }
}

impl fmt::Display for Syntax {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for note in self.notes {
            // TODO: include note when generating span, after ^^^
            // like:              something wrong!
            //                              ^^^^^ hint: do it right!
            // The way span is formatted is a bit jank,
            // should be composable methods to build up string.
            if !note.span.is_empty() { fmt::Display::fmt(&self.span, f)? };
        }
        write!(f, "Syntax Error: {}", self.message)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::source::Source;
    use std::rc::Rc;

    #[test]
    fn error() {
        // This is just a demo to check formatting
        // might not coincide with an actual Passerine error
        let source = Rc::new(Source::source("x = \"Hello, world\" -> y + 1"));
        let error = Syntax::error(
            "Unexpected token '\"Hello, world!\"'",
            &Span::new(&source, 4, 14),
        );

        let target = "In ./source:1:5
   |
 1 | x = \"Hello, world\" -> y + 1
   |     ^^^^^^^^^^^^^^
   |
Syntax Error: Unexpected token '\"Hello, world!\"'\
";

        let result = format!("{}", error);
        assert_eq!(result, target);
    }
}
