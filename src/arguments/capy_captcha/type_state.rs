use crate::arguments::capy_captcha::CapyVersion;

#[derive(Default, Debug)]
pub struct VersionNotProvided;

#[derive(Debug)]
pub struct Version(pub CapyVersion);
