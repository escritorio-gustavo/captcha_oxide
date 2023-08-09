use crate::arguments::normal_captcha::NormalCaptchaMethods;

#[derive(Default, Debug)]
pub struct MethodNotProvided;

#[derive(Debug)]
pub struct Method(pub NormalCaptchaMethods);
