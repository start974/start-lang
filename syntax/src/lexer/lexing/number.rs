use super::*;

/**
lex ascii digits
```ebnf
DIGIT = (0-9)
```
*/
pub fn digit<'src>() -> impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>> {
    any()
        .filter(|c: &char| c.is_ascii_digit())
        .labelled("digit")
}

/**
lex hexadecimal digits
DIGIT_HEX = (0-9 | a-f | A-F)
*/
pub fn digit_hex<'src>() -> impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>> {
    any()
        .filter(|c: &char| c.is_ascii_hexdigit())
        .labelled("digit_hex")
}

/**
lex octal digits
DIGIT_OCT = (0-7)
*/
pub fn digit_oct<'src>() -> impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>> {
    digit()
        .filter(|c: &char| *c != '8' && *c != '9')
        .labelled("digit_oct")
}

/**
lex binary digits
DIGIT_BIN = (0 | 1)
*/
pub fn digit_bin<'src>() -> impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>> {
    any()
        .filter(|c: &char| *c == '0' || *c == '1')
        .labelled("digit_bin")
}

/**
lexer number with given [`radix`] and [`digit`] parser
```ebnf
NUMBER_F(digit) = digit ( "_"* digit )*
```
*/
fn number_f<'src>(
    radix: u32,
    digit: impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>>,
) -> impl Parser<'src, &'src str, BigUint, Err<ErrorChumsky<'src>>> {
    let digit = Rc::new(digit);
    let underscores = just('_').repeated();

    digit
        .clone()
        .then(
            underscores
                .ignore_then(digit.clone())
                .repeated()
                .collect::<String>(),
        )
        .map(move |(digit1, digits2)| {
            let number_str = format!("{digit1}{digits2}");
            BigUint::parse_bytes(number_str.as_bytes(), radix).expect("Failed to parse number")
        })
}

/**
lex number with base prefix
with given [`prefix_lower`], [`prefix_upper`], [`radix`] and [`digit`] parser
```ebnf
NUMBER_BASE_PREFIXED(prefix_lower, prefix_upper, digit) = "0" ( prefix_lower | prefix_upper ) NUMBER_F(digit)
```
*/
fn number_base_prefixed<'src>(
    prefix_lower: char,
    prefix_upper: char,
    radix: u32,
    digit: impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>>,
) -> impl Parser<'src, &'src str, BigUint, Err<ErrorChumsky<'src>>> {
    let prefix = just("0").then(just(prefix_lower).or(just(prefix_upper)));
    prefix.ignore_then(number_f(radix, digit))
}

/**
lex number
```ebnf
NUMBER_DEC = digit ( digit | _)*
```
*/
pub fn number_dec<'src>() -> impl Parser<'src, &'src str, BigUint, Err<ErrorChumsky<'src>>> {
    number_f(10, digit()).labelled("number_dec")
}

/**
lex hexadecimal number
```ebnf
NUMBER_HEX = "0" ("x" | "X") digit_hex ( digit_hex | _ )*
```
*/
pub fn number_hex<'src>() -> impl Parser<'src, &'src str, BigUint, Err<ErrorChumsky<'src>>> {
    number_base_prefixed('x', 'X', 16, digit_hex()).labelled("number_hex")
}

/**
lex octal number
```ebnf
NUMBER_OCT = "0" ("o" | "O") digit_oct ( digit_oct | _ )*
```
*/
pub fn number_oct<'src>() -> impl Parser<'src, &'src str, BigUint, Err<ErrorChumsky<'src>>> {
    number_base_prefixed('o', 'O', 8, digit_oct()).labelled("number_oct")
}

/**
lex binary number
```ebnf
NUMBER_BIN = "0" ("b" | "B") digit_bin ( digit_bin | _ )*
```
*/
pub fn number_bin<'src>() -> impl Parser<'src, &'src str, BigUint, Err<ErrorChumsky<'src>>> {
    number_base_prefixed('b', 'B', 2, digit_bin()).labelled("number_bin")
}

/**
lex number: decimal, hexadecimal, octal or binary
```ebnf
NUMBER = NUMBER_HEX | NUMBER_OCT | NUMBER_BIN | NUMBER_DEC
```
*/
pub fn number<'src>() -> impl Parser<'src, &'src str, BigUint, Err<ErrorChumsky<'src>>> {
    // lex decimal number or hexadecimal or octal or binary
    choice((number_hex(), number_oct(), number_bin(), number_dec())).labelled("number")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lex_number_hex() {
        let parser = number_hex();

        let result = parser.parse("0x1A3F_FF").into_result();
        assert_eq!(result.unwrap(), BigUint::from(0x1A3FFFu32));
    }

    #[test]
    fn lex_number_oct() {
        let parser = number_oct();

        let result = parser.parse("0o765_4321").into_result();
        assert_eq!(result.unwrap(), BigUint::from(0o7654321u32));
    }

    #[test]
    fn lex_number_bin() {
        let parser = number_bin();

        let result = parser.parse("0b1101_0010").into_result();
        assert_eq!(result.unwrap(), BigUint::from(0b11010010u32));
    }

    #[test]
    fn lex_number_dec() {
        let parser = number_dec();

        let result = parser.parse("123_4_5_6_789").into_result();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BigUint::from(123456789u32));
    }

    #[test]
    fn lex_number() {
        let parser = number();

        let result = parser.parse("0xFF").into_result();
        assert_eq!(result.unwrap(), BigUint::from(255u8));

        let result = parser.parse("0o777").into_result();
        assert_eq!(result.unwrap(), BigUint::from(0o777u16));

        let result = parser.parse("0b101010").into_result();
        assert_eq!(result.unwrap(), BigUint::from(42u8));

        let result = parser.parse("12345").into_result();
        assert_eq!(result.unwrap(), BigUint::from(12345u16));
    }
}
