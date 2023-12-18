mod task;

pub use task::*;

#[cfg(test)]
mod test {
    use std::env;

    use crate::{
        captcha_types::turnstile_captcha::TurnstileStandaloneCaptcha, CaptchaSolver, CaptchaTask,
        Error,
    };

    #[tokio::test]
    async fn turnstile_standalone_captcha() -> Result<(), Error> {
        dotenv::dotenv().unwrap();

        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let captcha = TurnstileStandaloneCaptcha::builder()
            .website_url("https://2captcha.com/demo/cloudflare-turnstile")
            .website_key("0x4AAAAAAAC3DHQFLr1GavRN")
            .build()?;

        let solution = solver
            .solve(captcha)
            .await?
            .expect("Should never be none without a `callback_url`")
            .solution;

        assert!(!solution.token.is_empty(), "Empty?");

        Ok(())
    }
}

// {
//     "errorId": 0,
//     "status": "ready",
//     "solution": {
//         "token":"0.SSfSf9E-da3Ow94buwIaJiMFuB9AVltL1klC8JEYAHldcCY30s7SvJ_HHj0fTErxOu840k1PfGQf8yr_454g76cPYaOkz7sbJ8xsrNrJUVe4I4cSyApYgIsbdUTivn6FjjH7VJLf4_MGmd8I9BE4I9jrIcQG4BPRjqU1K1Ow51TfTrFeWi1iWbTZohrYJ-z-IQXPcI-V25uwcJFmyh-p4-ulpaHcQHybdZKwp83D_o19vns7Cyg4dj39scvk0ngCFpcS_hszrMbNYyDcDHpc_YlJCoUEM2-qcOByOPT38ha42UwPOLUAvZY5wZAX8tWcTlBC3NCLeETDhNg3wd4yZV2E3u9p3oj5BrDs2WWkYgNoh8V8jCL1zVEj8d4OAFmilSMayTvA1oqvJjrQl_kxNdb2FJwXVrtHHzWly9YUnV41HiQlccFW95dKZGEs2m8I.u5ympAaCikXyQp_4rMG7Gw.84fc4e4fd67d9b3c0b99f1abd495b742de920e716dfcc4fbd203426f1484662b",
//         "userAgent":"Mozilla\/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit\/537.36 (KHTML, like Gecko) Chrome\/116.0.0.0 Safari\/537.36"
//     },
//     "cost":"0.00145",
//     "ip":"186.0.146.41",
//     "createTime":1702901966,
//     "endTime":1702901979,
//     "solveCount":1
// }
