use super::token::Token;


/// Remove comments from the input.
/// 
/// The comment is a line starting with a (#) character. end with a semicolon (;) or newline character.
pub(super) fn remove_comment(input: &str) -> String {
    let mut result = String::with_capacity(input.len() + 1024);

    let mut is_comment = false;

    for c in input.chars() {
        if c == '#' {
            is_comment = true;
        }

        if is_comment == false {
            result.push(c);
        }

        if c == '\n' || c == '\r' || c == ';' {
            is_comment = false;
        }
    }

    result
}

/// Remove empty lines from the input.
/// 
/// The empty line is a line that contains only whitespace characters.
pub(super) fn remove_empty_line(input: &str) -> String {
    let mut result = String::with_capacity(input.len() + 1024);

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        result.push_str(line);
        result.push('\n');
    }

    result
}

/// Replace the newline character with a semicolon.
pub(super) fn replace_line_with_semicolon(input: &str) -> String {
    input.replace("\n", ";")
}

/// Split the tokens into multiple parts, the split position is the place token.
pub(super) fn recursion_split(place: &Token, tokens: &[Token]) -> Vec<Vec<Token>> {
    let mut result = Vec::new();

    let (left, right) = split(place, tokens);

    if right.is_empty() {
        result.push(left);
    } else {
        result.push(left);
        result.append(&mut recursion_split(place, &right));
    }

    result
}

/// Split the tokens into two parts, the first part contains the token before the place, and the second part contains the token after the place.
///
/// For example,
///
/// consider a token vec `[Token::String("A".to_string()), Token::Semicolon, Token::String("B".to_string())]`.
///
/// If the split position is Token::Semicolon, the "Left" will be `[Token::String("A")]` and the "Right" will be `[Token::String("B")]`.
///  
/// The split position "Token::Semicolon" itself is not included. Additionally, if there are multiple "Token::Semicolons", the leftmost one will be the target.
///
///
///
/// If the split position Token::Semicolon does not exist, the split operation cannot be performed.
///
/// In this case, all elements of the input tokens will be reflected in the left side of the tuple (Vec<Token>, Vec<Token>) returned.
///
/// This means that the resulting left side will contain the entire input token sequence.
///
/// **!Please refer to the unit tests for details.!**
///
pub(super) fn split(place: &Token, tokens: &[Token]) -> (Vec<Token>, Vec<Token>) {
    if tokens.contains(place) == false {
        return (tokens.to_vec(), Vec::default());
    }

    let mut left = Vec::with_capacity(tokens.len());

    let mut right = Vec::with_capacity(tokens.len());

    for (i, token) in tokens.iter().enumerate() {
        if token == place {
            let (l, r) = tokens.split_at(i);
            left = l.to_vec();
            right = r[1..].to_vec();
            break;
        }
    }

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_line_with_semicolon() {
        let input = "a\nb\nc";
        let expected = "a;b;c";

        assert_eq!(replace_line_with_semicolon(input), expected);
    }

    #[test]
    fn test_split() {
        let tokens = vec![
            Token::String("A".to_string()),
            Token::Semicolon,
            Token::String("B".to_string()),
        ];

        let place = &Token::Semicolon;

        let (left, right) = split(place, &tokens);

        assert_eq!(left, vec![Token::String("A".to_string())]);
        assert_eq!(right, vec![Token::String("B".to_string())]);
    }

    #[test]
    fn test_split_not_found() {
        let tokens = vec![
            Token::String("A".to_string()),
            Token::String("B".to_string()),
        ];

        let place = &Token::Semicolon;

        let (left, right) = split(place, &tokens);

        assert_eq!(
            left,
            vec![
                Token::String("A".to_string()),
                Token::String("B".to_string())
            ]
        );
        assert_eq!(right, Vec::default());
    }

    #[test]
    fn test_recursion_split() {
        let tokens = vec![
            Token::String("A".to_string()),
            Token::Semicolon,
            Token::String("B".to_string()),
            Token::Semicolon,
            Token::String("C".to_string()),
        ];

        let place = &Token::Semicolon;

        let result = recursion_split(place, &tokens);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], vec![Token::String("A".to_string())]);
        assert_eq!(result[1], vec![Token::String("B".to_string())]);
        assert_eq!(result[2], vec![Token::String("C".to_string())]);
    }

    #[test]
    fn test_remove_comment() {
        let input = "hello world;\n# this is a comment\nhello world;";

        let expected = "hello world;\nhello world;";

        assert_eq!(remove_comment(input), expected);
    }

    #[test]
    fn test_remove_empty_line() {
        let input = "hello world;\n\nhello world;\n";

        let expected_1 = "hello world;\nhello world;\n";

        assert_eq!(remove_empty_line(input), expected_1);
    }
}
