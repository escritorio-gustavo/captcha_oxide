# captcha_oxide

This is a rust library for solving captcha puzzles with the 2Captcha API

## Usage

```rust
use captcha_oxide::solver::CaptchaSolver;
use captcha_oxide::captcha_arguments::RecaptchaV3;
use captcha_oxide::response::RequestContent;

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
