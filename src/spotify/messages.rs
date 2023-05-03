use actix::prelude::*;
use crate::spotify::dtos::PlaylistInfoWithTracks;
use crate::errors::MuuzikaResult;

#[derive(Message)]
#[rtype(result = "MuuzikaResult<PlaylistInfoWithTracks>")]
pub struct FetchPlaylist {
    pub id: String
}

#[derive(Message)]
#[rtype(result = "MuuzikaResult<String>")]
pub struct GetAccessToken;