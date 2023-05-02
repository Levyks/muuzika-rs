use actix::prelude::*;
use crate::spotify::dtos::PlaylistInfoWithTracks;
use crate::errors::{UserFacingError};

#[derive(Message)]
#[rtype(result = "Result<PlaylistInfoWithTracks, UserFacingError>")]
pub struct FetchPlaylist {
    pub id: String
}

#[derive(Message)]
#[rtype(result = "Result<String, UserFacingError>")]
pub struct GetAccessToken;