#[cfg(test)]
mod derive_tests {
    use traq_bot_http::*;
    use ParseError::*;

    const PARSE_ERROR_VARIANTS: [ParseError; 11] = [
        ContentTypeNotFound,
        ReadContentTypeFailed,
        ContentTypeMismatch,
        BotTokenNotFound,
        ReadBotTokenFailed,
        BotTokenMismatch,
        BotEventNotFound,
        ReadBotEventFailed,
        BotEventMismatch,
        ReadBodyFailed,
        ParseBodyFailed,
    ];

    #[test]
    fn request_parser() {
        let verification_token = "verification_token";
        let parser = RequestParser::new(verification_token);
        println!("{:?}", parser);
    }

    #[test]
    fn parse_error() {
        for variant in PARSE_ERROR_VARIANTS.iter() {
            let error = variant.clone();
            assert_eq!(variant, &error);
            assert_eq!(format!("{:?}", variant), format!("{:?}", error));
        }
    }
}
