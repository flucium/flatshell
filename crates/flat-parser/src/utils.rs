use crate::token::Token;

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
    fn test_recursion_split() {
        let tokens = vec![
            Token::String("a".to_string()),
            Token::String("b".to_string()),
            Token::Semicolon,
            Token::String("c".to_string()),
            Token::String("d".to_string()),
            Token::Semicolon,
            Token::String("e".to_string()),
            Token::String("f".to_string()),
        ];

        let place = Token::Semicolon;

        let result = recursion_split(&place, &tokens);

        assert_eq!(
            result,
            vec![
                vec![
                    Token::String("a".to_string()),
                    Token::String("b".to_string())
                ],
                vec![
                    Token::String("c".to_string()),
                    Token::String("d".to_string())
                ],
                vec![
                    Token::String("e".to_string()),
                    Token::String("f".to_string())
                ]
            ]
        );
    }

    #[test]
    fn test_recursion_split_no_place() {
        let tokens = vec![
            Token::String("a".to_string()),
            Token::String("b".to_string()),
            Token::String("c".to_string()),
            Token::String("d".to_string()),
        ];

        let place = Token::Semicolon;

        let result = recursion_split(&place, &tokens);

        assert_eq!(
            result,
            vec![vec![
                Token::String("a".to_string()),
                Token::String("b".to_string()),
                Token::String("c".to_string()),
                Token::String("d".to_string())
            ]]
        );
    }

    #[test]
    fn test_split() {
        let tokens = vec![
            Token::String("a".to_string()),
            Token::String("b".to_string()),
            Token::Semicolon,
            Token::String("c".to_string()),
            Token::String("d".to_string()),
        ];

        let place = Token::Semicolon;

        let (left, right) = split(&place, &tokens);

        assert_eq!(
            left,
            vec![
                Token::String("a".to_string()),
                Token::String("b".to_string())
            ]
        );

        assert_eq!(
            right,
            vec![
                Token::String("c".to_string()),
                Token::String("d".to_string())
            ]
        );
    }

    #[test]
    fn test_split_no_place() {
        let tokens = vec![
            Token::String("a".to_string()),
            Token::String("b".to_string()),
            Token::String("c".to_string()),
            Token::String("d".to_string()),
        ];

        let place = Token::Semicolon;

        let (left, right) = split(&place, &tokens);

        assert_eq!(
            left,
            vec![
                Token::String("a".to_string()),
                Token::String("b".to_string()),
                Token::String("c".to_string()),
                Token::String("d".to_string())
            ]
        );

        assert_eq!(right, Vec::default());
    }

    #[test]
    fn test_split_no_place_left_and_none_right() {
        let tokens = vec![
            Token::String("a".to_string()),
            Token::String("b".to_string()),
            Token::String("c".to_string()),
            Token::String("d".to_string()),
        ];

        let place = Token::Semicolon;

        let (left, right) = split(&place, &tokens);

        assert_eq!(
            left,
            vec![
                Token::String("a".to_string()),
                Token::String("b".to_string()),
                Token::String("c".to_string()),
                Token::String("d".to_string())
            ]
        );

        assert_eq!(right, Vec::default());
    }
}
