mod solution;
mod task;

pub use solution::*;
pub use task::*;

#[cfg(test)]
mod test {
    use std::env;

    use crate::{captcha_types::lemin_captcha::LeminCaptcha, CaptchaSolver, CaptchaTask, Error};

    #[tokio::test]
    async fn lemin_captcha() -> Result<(), Error> {
        dotenv::dotenv().unwrap();
        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let captcha = LeminCaptcha::builder()
            .website_url("https://2captcha.com/demo/lemin")
            .captcha_id("CROPPED_3dfdd5c_d1872b526b794d83ba3b365eb15a200b")
            .div_id("lemin-cropped-captcha")
            .lemin_api_server_subdomain(Some("api.leminnow.com"))
            .build()?;

        let solution = solver.solve(captcha).await?.unwrap().solution;

        assert_ne!(solution.answer, "");

        Ok(())
    }
}
