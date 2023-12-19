use super::{requests::create_task::CreateTaskResponse, requests::get_balance::GetBalanceResponse};

/// Represents all the errors that can be returned by the 2captcha API
#[derive(thiserror::Error, Debug)]
pub enum SolveError {
    #[error("Your API key is incorrect. Make sure you set the key correctly and copied it from the dashboard in Customer or Developer mode")]
    InvalidApiKey,

    #[error("Your bid is too low for the captcha you submit or the queue of your captchas is loo long and the API temporarily does not accept more captchas from you")]
    NoSlotAvailable,

    #[error("Image size is smaller than 100 bytes")]
    ImageTooSmall,

    #[error("Image is larger than 100kB or bigger than 600px on any side")]
    ImageTooBig,

    #[error("You don't have funds on your account")]
    ZeroBalance,

    #[error("The request was sent from an IP that is not in your list of trusted IPs")]
    IpNotAllowed,

    #[error("Unable to solve captcha - three workers were unable solve it. The captcha price is automatically returned to your balance")]
    UnsolvableCaptcha,

    #[error("The error is returned when the 100% accuracy feature is enabled. The error means that the max numbers of attempts was reached but the min number of matches was not found")]
    BadDuplicates,

    #[error("Request made to an API route that does not exist")]
    NoSuchMethod,

    #[error("The image can not be processed due to an incorrect format or size, or the image is corrupted. Please check the image in your request payload")]
    UnsupportedImageType,

    #[error("You've provided an incorrect captcha ID in the request")]
    CaptchaIdNotFound,

    #[error("Your IP address is banned due to improper use of the API")]
    IpBlocked,

    #[error("The `task` property is missing in your call to `createTask`")]
    TaskNotProvided,

    #[error("The `task` property in your call to `createTask` contains a type of task that is not supported the API")]
    TaskNotSupported,

    #[error("The `sitekey` value provided in your request is not valid")]
    InvalidSiteKey,

    #[error("Your API access was blocked for improper use of the API. Please contact support to resolve the issue")]
    AccountSuspended,

    #[error("Unable to establish connection through the proxy")]
    BadProxy,

    #[error("Could not connect to proxy")]
    ProxyConnectionFailed,

    #[error("The required captcha parameters in your request are missing or incorrect")]
    BadParameters,

    #[error("The error is returned in cases when `imgInstructions` contains an unsupported file type, corrupted file or the size of the image is over the limits. The limits are described in the corresponding task type specification.")]
    BadImageInstructions,
}

impl From<&str> for SolveError {
    fn from(value: &str) -> Self {
        match value {
            "ERROR_KEY_DOES_NOT_EXIST" => SolveError::InvalidApiKey,
            "ERROR_NO_SLOT_AVAILABLE" => SolveError::NoSlotAvailable,
            "ERROR_ZERO_CAPTCHA_FILESIZE" => SolveError::ImageTooSmall,
            "ERROR_TOO_BIG_CAPTCHA_FILESIZE" => SolveError::ImageTooBig,
            "ERROR_ZERO_BALANCE" => SolveError::ZeroBalance,
            "ERROR_IP_NOT_ALLOWED" => SolveError::IpNotAllowed,
            "ERROR_CAPTCHA_UNSOLVABLE" => SolveError::UnsolvableCaptcha,
            "ERROR_BAD_DUPLICATES" => SolveError::BadDuplicates,
            "ERROR_NO_SUCH_METHOD" => SolveError::NoSuchMethod,
            "ERROR_IMAGE_TYPE_NOT_SUPPORTED" => SolveError::UnsupportedImageType,
            "ERROR_NO_SUCH_CAPCHA_ID" => SolveError::CaptchaIdNotFound,
            "ERROR_IP_BLOCKED" => SolveError::IpBlocked,
            "ERROR_TASK_ABSENT" => SolveError::TaskNotProvided,
            "ERROR_TASK_NOT_SUPPORTED" => SolveError::TaskNotSupported,
            "ERROR_RECAPTCHA_INVALID_SITEKEY" => SolveError::InvalidSiteKey,
            "ERROR_ACCOUNT_SUSPENDED" => SolveError::AccountSuspended,
            "ERROR_BAD_PROXY" => SolveError::BadProxy,
            "ERROR_PROXY_CONNECTION_FAILED" | "ERR_PROXY_CONNECTION_FAILED" => {
                SolveError::ProxyConnectionFailed
            }
            "ERROR_BAD_PARAMETERS" => SolveError::BadParameters,
            "ERROR_BAD_IMGINSTRUCTIONS" => SolveError::BadImageInstructions,
            x => unreachable!("Unreachable 2captcha error: {}", x),
        }
    }
}

impl From<CreateTaskResponse> for Result<u64, SolveError> {
    fn from(val: CreateTaskResponse) -> Self {
        match val {
            CreateTaskResponse::TaskCreated { task_id } => Ok(task_id),
            CreateTaskResponse::Error { error_code, .. } => {
                let error_code = error_code.as_ref();

                Err(error_code.into())
            }
        }
    }
}

impl From<GetBalanceResponse> for Result<f32, SolveError> {
    fn from(value: GetBalanceResponse) -> Self {
        match value {
            GetBalanceResponse::TaskCreated { balance } => Ok(balance),
            GetBalanceResponse::Error { error_code } => Err(error_code.as_ref().into()),
        }
    }
}
