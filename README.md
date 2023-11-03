# captcha_oxide

![Build](https://github.com/escritorio-gustavo/captcha_oxide/workflows/Continuous%20integration/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/captcha_oxide.svg)](https://crates.io/crates/captcha_oxide)
[![Documentation](https://docs.rs/captcha_oxide/badge.svg)](https://docs.rs/captcha_oxide)

This is a rust library for solving captcha puzzles with the 2Captcha API

## Usage

```rust
use captcha_oxide::{
  Solver,
  RecaptchaV3,
  Error,
  CaptchaTask
};

use url::Url;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let solver = Solver::new("YOUR TWOCAPTCHA API KEY");

  let args = RecaptchaV3::builder()
    .website_url(Url::parse("https://someurl.com")?)
    .website_key("SITE_KEY")
    .build();

  let solution = solver
    .solve(args)
    .await?
    .unwrap()
    .g_recaptcha_response;

  assert!(!solution.is_empty());

  Ok(())
}
```
