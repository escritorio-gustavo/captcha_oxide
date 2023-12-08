mod solution;
mod task;

pub use solution::*;
pub use task::*;

#[cfg(test)]
mod test {
    use std::env;

    use crate::{captcha_types::h_captcha::HCaptcha, CaptchaSolver, CaptchaTask};

    #[tokio::test]
    async fn h_captcha() -> Result<(), crate::Error> {
        dotenv::dotenv().unwrap();
        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let captcha = <HCaptcha>::builder()
            .website_url("https://2captcha.com/demo/hcaptcha")
            .website_key("f7de0da3-3303-44e8-ab48-fa32ff8ccc7b")
            .build()?;

        let solution = solver
            .solve(captcha)
            .await?
            .expect("This cannot return `None`, because `Solver` was not given a `callback_url`")
            .token;

        assert!(!solution.is_empty());

        Ok(())
    }
}
