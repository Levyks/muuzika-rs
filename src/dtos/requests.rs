use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrJoinRoomRequest {
    pub username: String,
    pub captcha_token: String
}