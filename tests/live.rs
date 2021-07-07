
// https://developers.google.com/recaptcha/docs/faq
// This test key is valid for recaptcha v2 and will accept any user response.
const V2_TEST_SECRET: &str = "6LeIxAcTAAAAAGG-vFI1TnRWxMZNFuojJ4WifJWe";
const INVALID_SECRET: &str = "invalid-secret";
const INVALID_RESPONSE: &str = "invalid-response";
const VALID_RESPONSE: &str = "valid-response";

#[tokio::test]
async fn test_invalid_secret() {
    use recaptcha::error::Error::*;
    use recaptcha::error::Code::*;
    let response = recaptcha::verify(INVALID_SECRET, INVALID_RESPONSE, None).await;

    assert!(
        matches!(&response, Err(Codes(errors)) if errors.contains(&InvalidSecret)),
        "Expected invalid `InvalidSecret` in response: {:#?}", response,
    );
}

#[tokio::test]
async fn test_valid_response() {
    let response = recaptcha::verify(V2_TEST_SECRET, VALID_RESPONSE, None).await;

    assert!(
        matches!(&response, Ok(())),
        "Expected valid response in response: {:#?}", response,
    );
}
