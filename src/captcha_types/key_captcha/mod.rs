mod solution;
mod task;

pub use solution::*;
pub use task::*;

#[cfg(test)]
mod test {
    use std::env;

    use crate::{captcha_types::key_captcha::KeyCaptcha, CaptchaSolver, CaptchaTask, Error};

    #[tokio::test]
    async fn key_captcha() -> Result<(), Error> {
        dotenv::dotenv().unwrap();
        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let captcha = KeyCaptcha::builder()
            .website_url("https://2captcha.com/demo/keycaptcha")
            .user_id(184015_u32)
            .session_id("8510374722aa3f99a7199d306865afb2")
            .web_server_sign("bed1536559a1cab72ecd0e28e89c431c")
            .web_server_sign2("104ac902450db8362ce5fc11e841ee47")
            .build()?;

        let solution = solver.solve(captcha).await?.unwrap().solution;

        assert_ne!(solution.token, "");
        Ok(())
    }
}
