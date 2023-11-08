mod builder;
mod task;

pub use task::*;

#[cfg(test)]
mod test {
    use std::env;

    use crate::{
        captcha_types::turnstile_captcha::TurnstileStandaloneCaptcha, CaptchaTask, Error, Solver,
    };

    #[tokio::test]
    async fn turnstile_standalone_captcha() -> Result<(), Error> {
        dotenv::dotenv().unwrap();

        let solver = Solver::new(env::var("API_KEY").unwrap());

        let captcha = TurnstileStandaloneCaptcha::builder()
            .website_url("https://2captcha.com/demo/cloudflare-turnstile")?
            .website_key("0x4AAAAAAAC3DHQFLr1GavRN")
            .build();

        let solution = solver
            .solve(captcha)
            .await?
            .expect("Should never be none without a `callback_url`");

        assert!(!solution.token.is_empty(), "Empty?");

        Ok(())
    }
}
