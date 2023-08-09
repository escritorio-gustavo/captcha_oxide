use crate::arguments::normal_captcha::ImageInstructions;

#[derive(Default, Debug)]
pub struct ImgInstructionsNotProvided;

#[derive(Debug)]
pub struct ImgInstructions(pub ImageInstructions);
