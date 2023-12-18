# captcha_oxide

![Build](https://github.com/escritorio-gustavo/captcha_oxide/workflows/Continuous%20integration/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/captcha_oxide.svg)](https://crates.io/crates/captcha_oxide)
[![Documentation](https://docs.rs/captcha_oxide/badge.svg)](https://docs.rs/captcha_oxide)

This is a rust library for solving captcha puzzles with the 2Captcha API

This library is now feature complete and stable, so unless there is a breaking
change on the 2captcha API, there will no longer be any breaking changes in the library starting with version 5.0.0!

Contributions are very welcome, especially documentation and examples, feel free
to submit a PR.

## Usage

```rust
use captcha_oxide::{
  CaptchaSolver,
  catcha_types::recaptcha::RecaptchaV3,
  Error,
  CaptchaTask
};

use url::Url;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let solver = CaptchaSolver::new("YOUR TWOCAPTCHA API KEY");

  let args = RecaptchaV3::builder()
    .website_url("https://someurl.com")
    .website_key("SITE_KEY")
    .build()?;

  let solution = solver
    .solve(args)
    .await?
    .unwrap()
    .solution
    .g_recaptcha_response;

  assert!(!solution.is_empty());

  Ok(())
}
```
