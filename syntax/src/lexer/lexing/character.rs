use super::*;

/**
char with number with [`digit`] parser, number of digit [`number_digit`] and [`radix`]
```ebfn
ESCAPE_NUMBER_CHAR(digit, number_digit) := digit{number_digit}
```
*/
fn escape_number_char<'src>(
    digit: impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>>,
    number_digit: usize,
    radix: u32,
) -> impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>> {
    let digits = digit.repeated().exactly(number_digit).collect::<String>();

    digits.try_map(move |digits, span| {
        u8::from_str_radix(&digits, radix)
            .map(|b| b as char)
            .map_err(|_| Rich::custom(span, "escape character"))
    })
}

/**
escape char with prefix, with [`digit`] parser, number of digit [`number_digit`] and [`radix`]
```ebnf
ESCAPE_NUMBER_CHAR_PREFIXED(prefix) := prefix ESCAPE_NUMBER_CHAR(digit, number_digit)
```
*/
fn escape_number_char_prefixed<'src>(
    prefix: char,
    digit: impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>>,
    number_digit: usize,
    radix: u32,
) -> impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>> {
    just(prefix).ignore_then(escape_number_char(digit, number_digit, radix))
}

/**
escape unicode char
```ebnf
ESCAPE_UNICODE_CHAR := "u" "{" DIGIT_HEX+ "}"
```
*/
fn escape_unicode_char<'src>() -> impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>> {
    just('u')
        .ignore_then(
            digit_hex()
                .repeated()
                .at_least(1)
                .collect::<String>()
                .delimited_by(just('{'), just('}')),
        )
        .try_map(|digits, span| {
            u32::from_str_radix(&digits, 16)
                .ok()
                .and_then(std::char::from_u32)
                .ok_or_else(|| Rich::custom(span, "unicode escape"))
        })
}

/**
escape char
```ebnf
ESCAPE_CHAR := "\"
   ("\\" | "\"" | "\'" | "n" | "r" | "t"
   | DIGIT{3} | "x" DIGIT_HEX{2} | "o" DIGIT_OCT{3}
   | "u{" DIGIT_HEX+ "}")
```
*/
fn escape_char<'src>() -> impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>> {
    just('\\').ignore_then(choice((
        just('\\').to('\\'),
        just('\"').to('\"'),
        just('\'').to('\''),
        just('n').to('\n'),
        just('r').to('\r'),
        just('t').to('\t'),
        escape_number_char(digit(), 3, 10),
        escape_number_char_prefixed('x', digit_hex(), 2, 16),
        escape_number_char_prefixed('o', digit_oct(), 3, 8),
        escape_unicode_char(),
    )))
}

/**
lex caracter
```ebnf
CHARACTER_LIT :=
    | ESCAPE_CHAR
    | [U+0000 .. U+D7FF]
    | [U+E000 .. U+10FFFF]
```
*/
fn character_lit<'src>() -> impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>> {
    choice((
        escape_char(),
        any().filter(|c: &char| {
            let cp = *c as u32;
            (cp <= 0xD7FF) || (0xE000..=0x10FFFF).contains(&cp)
        }),
    ))
}

/// lex character
/// ```ebnf
/// CHARACTER := "'" CHARACTER_LIT "'"
/// ```
pub fn character<'src>() -> impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>> {
    let quote = just('\'').labelled("'");
    character_lit()
        .delimited_by(quote, quote)
        .labelled("character")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn character_basic() {
        let parser = character();

        let result = parser.parse("'a'").into_result();
        assert_eq!(result.unwrap(), 'a');

        let result = parser.parse("'.'").into_result();
        assert_eq!(result.unwrap(), '.');

        let result = parser.parse("'δ'").into_result();
        assert_eq!(result.unwrap(), 'δ');

        let result = parser.parse("'α'").into_result();
        assert_eq!(result.unwrap(), 'α');

        let result = parser.parse("'😀'").into_result();
        assert_eq!(result.unwrap(), '😀');
    }

    #[test]
    fn character_escape() {
        let parser = character();

        let result = parser.parse(r"'\\'").into_result();
        assert_eq!(result.unwrap(), '\\');

        let result = parser.parse("'\"'").into_result();
        assert_eq!(result.unwrap(), '\"');

        let result = parser.parse(r"'\''").into_result();
        assert_eq!(result.unwrap(), '\'');

        let result = parser.parse(r"'\n'").into_result();
        assert_eq!(result.unwrap(), '\n');

        let result = parser.parse(r"'\r'").into_result();
        assert_eq!(result.unwrap(), '\r');

        let result = parser.parse(r"'\t'").into_result();
        assert_eq!(result.unwrap(), '\t');

        let result = parser.parse(r"'\065'").into_result();
        assert_eq!(result.unwrap(), 'A');

        let result = parser.parse(r"'\x41'").into_result();
        assert_eq!(result.unwrap(), 'A');

        let result = parser.parse(r"'\o101'").into_result();
        assert_eq!(result.unwrap(), 'A');

        let result = parser.parse(r"'\u{03B1}'").into_result();
        assert_eq!(result.unwrap(), 'α');
    }

    #[test]
    fn invalid_character_escape() {
        let parser = character();

        let result = parser.parse(r"'\999'").into_result();
        let errs = result.unwrap_err();
        let err = errs.first().unwrap();
        assert!(err.to_string().contains("escape character"));
    }

    #[test]
    fn invalid_character_unicode_escape() {
        let parser = character();

        let result = parser.parse(r"'\u{110000}'").into_result();
        let errs = result.unwrap_err();
        let err = errs.first().unwrap();
        assert!(err.to_string().contains("unicode escape"));
    }
}
