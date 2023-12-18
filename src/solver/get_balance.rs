#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct GetBalanceRequest<'a> {
    pub client_key: &'a str,
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(super) enum GetBalanceResponse {
    #[serde(rename_all = "camelCase")]
    TaskCreated { balance: f32 },

    #[serde(rename_all = "camelCase")]
    Error { error_code: Box<str> },
}
