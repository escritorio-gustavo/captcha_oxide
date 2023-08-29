# captcha_oxide

![Build](https://github.com/escritorio-gustavo/captcha_oxide/workflows/Continuous%20integration/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/captcha_oxide.svg)](https://crates.io/crates/captcha_oxide)
[![Documentation](https://docs.rs/captcha_oxide/badge.svg)](https://docs.rs/captcha_oxide)

This is a rust library for solving captcha puzzles with the 2Captcha API

## Usage

```rust
use captcha_oxide::{
  CaptchaSolver,
  Solution,
  Error,
  arguments::RecaptchaV3,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
  let solver = CaptchaSolver::new("YOUR TWOCAPTCHA API KEY");

  let args = RecaptchaV3::builder()
    .page_url("https://someurl.com")
    .site_key("SITE_KEY")
    .build();

  let solution = solver.solve(args).await?.unwrap().solution;

  // If there isn't a variant named after your captcha type,
  // it's because it only returns a token, so you should use
  // the Token variant
  let Solution::Token(solution) = solution else {
    unreachable!()
  };

  assert_ne!(solution, "");

  Ok(())
}
```
