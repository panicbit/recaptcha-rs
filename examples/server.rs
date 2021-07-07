use rocket::{form::Form, response::{Redirect, content::Html}};

#[macro_use] extern crate rocket;

// https://developers.google.com/recaptcha/docs/faq
// Test keys are valid for recaptcha v2 and will accept any user response.
const V2_TEST_SITE_KEY: &str = "6LeIxAcTAAAAAJcZVRqyHh71UMIEGNQ_MXjiZKhI";
const V2_TEST_SECRET: &str = "6LeIxAcTAAAAAGG-vFI1TnRWxMZNFuojJ4WifJWe";

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![
            index,
            verify,
        ])
}

#[get("/")]
fn index() -> Html<String> {
    let site = format!(r#"
        <html>
            <head>
                <title>reCAPTCHA demo: Simple page</title>
                <script src="https://www.google.com/recaptcha/api.js" async defer></script>
            </head>
            <body>
                <form action="/verify" method="POST">
                <div class="g-recaptcha" data-sitekey="{site_key}"></div>
                <br/>
                <input type="submit" value="Submit">
                <br/>
                (Note: The test secret never fails verification)
                </form>
            </body>
        </html>
    "#, site_key = V2_TEST_SITE_KEY);

    Html(site)
}

#[post("/verify", data = "<recaptcha>")]
async fn verify(recaptcha: Form<RecaptchaForm<'_>>) -> Html<&'static str> {
    let result = recaptcha::verify(V2_TEST_SECRET, recaptcha.g_recaptcha_response, None).await;

    eprintln!("{:?}", result);

    let message = match result.is_ok() {
        true => "Verification successful!",
        false => "Verification failed!",
    };

    Html(message)
}

#[derive(FromForm)]
struct RecaptchaForm<'r> {
    #[field(name = "g-recaptcha-response")]
    g_recaptcha_response: &'r str,
}

#[catch(404)]
fn not_found() -> Redirect {
    Redirect::to("/")
}
