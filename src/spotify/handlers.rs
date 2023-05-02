use actix::prelude::*;
use crate::errors::UserFacingError;
use crate::spotify::{
    fetcher::SpotifyFetcher,
    messages::*,
    dtos::{PlaylistInfoWithTracks, AccessToken}
};

impl Handler<GetAccessToken> for SpotifyFetcher {
    type Result = ResponseFuture<Result<String, ()>>;

    fn handle(&mut self, _msg: GetAccessToken, _ctx: &mut Context<Self>) -> Self::Result {
        
        if let Some(access_token) = &self.get_valid_access_token() {
            Box::pin(async move { Ok(access_token.clone()) })
        } else {
            Box::pin(async move { 
                todo!("Get access token from Spotify")
            })
        }
    }
}

impl Handler<FetchPlaylist> for SpotifyFetcher {
    type Result = Result<PlaylistInfoWithTracks, UserFacingError>;

    fn handle(&mut self, msg: FetchPlaylist, _ctx: &mut Context<Self>) -> Self::Result {
        todo!("Fetch playlist from Spotify")
    }
}