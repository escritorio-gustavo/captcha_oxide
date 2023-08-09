use crate::arguments::{
    type_state::page_url::{PageUrl, PageUrlNotProvided},
    KeyCaptcha,
};

pub use super::type_state::{
    server_sign::{ServerSign, ServerSignNotProvided},
    server_sign2::{ServerSign2, ServerSign2NotProvided},
    session_id::{SessionId, SessionIdNotProvided},
    user_id::{UserId, UserIdNotProvided},
};

#[derive(Debug, Default, Clone)]
/// Builds a [`KeyCaptcha`] instance using the typestate pattern
/// to help avoid sending avoid inconsistent data to the
/// 2captcha API
///
/// # Example
/// ```
/// use captcha_oxide::arguments::KeyCaptcha;
///
/// let args = KeyCaptcha::builder()
///     .page_url("SOME_URL")
///     .user_id("DINAMICALLY_GENERATED")
///     .session_id("DINAMICALLY_GENERATED")
///     .server_sign("DINAMICALLY_GENERATED")
///     .server_sign2("DINAMICALLY_GENERATED")
///     .build();
/// ```
pub struct KeyCaptchaBuilder<T, U, V, W, X> {
    page_url: T,
    user_id: U,
    session_id: V,
    server_sign: W,
    server_sign2: X,
    pingback: Option<String>,
}

impl
    KeyCaptchaBuilder<
        PageUrlNotProvided,
        UserIdNotProvided,
        SessionIdNotProvided,
        ServerSignNotProvided,
        ServerSign2NotProvided,
    >
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl KeyCaptchaBuilder<PageUrl, UserId, SessionId, ServerSign, ServerSign2> {
    pub fn build(self) -> KeyCaptcha {
        KeyCaptcha {
            page_url: self.page_url.0,
            user_id: self.user_id.0,
            session_id: self.session_id.0,
            server_sign: self.server_sign.0,
            server_sign2: self.server_sign2.0,
            pingback: self.pingback,
        }
    }
}

impl<T, U, V, W, X> KeyCaptchaBuilder<T, U, V, W, X> {
    /// Full URL of the page where you see the captcha
    pub fn page_url(self, page_url: impl Into<String>) -> KeyCaptchaBuilder<PageUrl, U, V, W, X> {
        KeyCaptchaBuilder {
            page_url: PageUrl(page_url.into()),
            user_id: self.user_id,
            session_id: self.session_id,
            server_sign: self.server_sign,
            server_sign2: self.server_sign2,
            pingback: self.pingback,
        }
    }

    /// Value of the s_s_c_user_id parameter you found on the page's source code
    pub fn user_id(self, user_id: impl Into<String>) -> KeyCaptchaBuilder<T, UserId, V, W, X> {
        KeyCaptchaBuilder {
            page_url: self.page_url,
            user_id: UserId(user_id.into()),
            session_id: self.session_id,
            server_sign: self.server_sign,
            server_sign2: self.server_sign2,
            pingback: self.pingback,
        }
    }

    /// Value of the s_s_c_session_id parameter you found on the page's source code
    pub fn session_id(
        self,
        session_id: impl Into<String>,
    ) -> KeyCaptchaBuilder<T, U, SessionId, W, X> {
        KeyCaptchaBuilder {
            page_url: self.page_url,
            user_id: self.user_id,
            session_id: SessionId(session_id.into()),
            server_sign: self.server_sign,
            server_sign2: self.server_sign2,
            pingback: self.pingback,
        }
    }

    /// Value of the s_s_c_web_server_sign parameter you found on the page's source code
    pub fn server_sign(
        self,
        server_sign: impl Into<String>,
    ) -> KeyCaptchaBuilder<T, U, V, ServerSign, X> {
        KeyCaptchaBuilder {
            page_url: self.page_url,
            user_id: self.user_id,
            session_id: self.session_id,
            server_sign: ServerSign(server_sign.into()),
            server_sign2: self.server_sign2,
            pingback: self.pingback,
        }
    }

    /// Value of the s_s_c_web_server_sign2 parameter you found on the page's source code
    pub fn server_sign2(
        self,
        server_sign2: impl Into<String>,
    ) -> KeyCaptchaBuilder<T, U, V, W, ServerSign2> {
        KeyCaptchaBuilder {
            page_url: self.page_url,
            user_id: self.user_id,
            session_id: self.session_id,
            server_sign: self.server_sign,
            server_sign2: ServerSign2(server_sign2.into()),
            pingback: self.pingback,
        }
    }

    /// Callback URL where you wish to receive the response
    pub fn pingback(mut self, pingback: impl Into<String>) -> Self {
        self.pingback = Some(pingback.into());
        self
    }
}
