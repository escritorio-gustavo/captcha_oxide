mod solution;
mod task;

pub use solution::*;
pub use task::*;

#[cfg(test)]
mod test {
    use std::env;

    use crate::{captcha_types::mt_captcha::MtCaptcha, CaptchaSolver, CaptchaTask, Error};

    #[tokio::test]
    async fn mt_captcha() -> Result<(), Error> {
        dotenv::dotenv().unwrap();
        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let captcha = MtCaptcha::builder()
            .website_url(
                "https://service.mtcaptcha.com/mtcv1/demo/index.html?sitekey=MTPublic-DemoKey9M",
            )
            .website_key("MTPublic-DemoKey9M")
            .build()?;

        let solution = solver.solve(captcha).await?.unwrap().solution;

        assert_ne!(solution.token, "");
        Ok(())
    }
}
