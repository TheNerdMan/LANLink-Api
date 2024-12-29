use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EditUserPayloadDto {
    pub(crate) new_username: String,
    pub(crate) new_first_name: String,
    pub(crate) new_last_name: String,
    pub(crate) new_discord_username: String,
    pub(crate) new_steam_url: String,
}