#[derive(Debug, Default)]
pub struct CaptchaTextNotProvided;

#[derive(Debug)]
pub struct CaptchaText(pub(super) String);
