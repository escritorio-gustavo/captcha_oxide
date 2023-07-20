# captcha_oxide

![Build](https://github.com/escritorio-gustavo/captcha_oxide/workflows/Continuous%20integration/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/captcha_oxide.svg)](https://crates.io/crates/captcha_oxide)
[![Documentation](https://docs.rs/captcha_oxide/badge.svg)](https://docs.rs/captcha_oxide)

This is a rust library for solving captcha puzzles with the 2Captcha API

## Usage

```rust
use captcha_oxide::{
  solver::CaptchaSolver,
  captcha_arguments::RecaptchaV3,
  response::RequestContent
};


#[tokio::main]
async fn main() {
  let solver = CaptchaSolver::new("YOUR TWOCAPTCHA API KEY");

  let args = RecaptchaV3 {
    page_url: String::from("https://someurl.com"),
    site_key: String::from("SITE_KEY"),
    ..Default.default()
  };

  match solver.solve(args).await {
    Ok(solution) => {
      // If there isn't a variant named after your captcha type,
      // it's because it only returns a token, so you should use
      // ths String variant
      match solution.solution {
        RequestContent::String(plain_text_solution) => {
          todo!("Use the solution");
        },
        _ => unreachable!()
      }
    },
    Err(e) => {
      todo!("Handle your error");
    }
  };
}
```
