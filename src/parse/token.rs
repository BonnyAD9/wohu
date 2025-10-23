#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Token {
    Ident,

    Colon,  // :
    Assign, // =
    Comma,  // ,

    OpenBracket,  // {
    CloseBracket, // }
    OpenSq,       // [
    CloseSq,      // ]

    String,     // "blah"
    LineString, // l{blah}

    KwName,  // name
    KwOrder, // order

    DecLanguage, // #language

    Eof,
}
