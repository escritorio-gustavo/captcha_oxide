mod builder;
mod task;

pub use task::*;

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use std::env;
    use std::str::FromStr;

    use crate::{captcha_types::RecaptchaV2, CaptchaTask, Error, Solver};

    #[tokio::test]
    async fn recaptcha_v2() -> Result<(), Error> {
        dotenv().unwrap();

        let data = RecaptchaV2::builder()
            .website_url(url::Url::from_str("https://patrickhlauke.github.io/recaptcha/").unwrap())
            .website_key("6Ld2sf4SAAAAAKSgzs0Q13IZhY02Pyo31S2jgOB5")
            .build();

        let solver = Solver::new(env::var("API_KEY").unwrap());

        let solution = solver
            .solve(data)
            .await?
            .expect("This can't be None since callback_url was not provided")
            .g_recaptcha_response;

        assert!(!solution.is_empty());
        Ok(())
    }
}
