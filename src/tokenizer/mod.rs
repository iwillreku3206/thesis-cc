pub mod preprocess_token;
pub mod token;
pub mod tokenizer;

#[derive(Debug)]
enum TokenizationIssue {
    UnknownEscapeSequence(String),
    UnterminatedString,
    OutOfRange,
    MissingHexadecimalDigits,
    HexadecimalFloatWithoutExponent,
}
