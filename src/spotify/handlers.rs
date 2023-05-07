use std::pin::Pin;
use std::time::{Duration, SystemTime};
use actix::prelude::*;
use crate::errors::{MuuzikaError, MuuzikaFutureResult, UserFacingError};
use crate::spotify::{
    fetcher::SpotifyFetcher,
    messages::*,
    dtos::{PlaylistInfoWithTracks, AccessToken}
};

impl Handler<GetAccessToken> for SpotifyFetcher {
    type Result = MuuzikaFutureResult<String>;

    fn handle(&mut self, _msg: GetAccessToken, ctx: &mut Context<Self>) -> Self::Result {
        let addr = ctx.address();
        let preexistent_token_result = self.get_access_token_if_not_expired();
        
        Box::pin(async move {
            let token = match preexistent_token_result? {
                Some(token) => token.clone(),
                None => addr.send(GenerateAccessToken).await??,
            };
            Ok(token.clone())
        })
    }
}

impl Handler<GenerateAccessToken> for SpotifyFetcher {
    type Result = MuuzikaFutureResult<String>;

    fn handle(&mut self, _msg: GenerateAccessToken, ctx: &mut Context<Self>) -> Self::Result {
        Box::pin(async move {
            let token = "TODO: generate access token";
            let expires_on = Duration::from_secs(3600);
            
            self.access_token = Some(token.to_string());
            self.access_token_expires_at = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs() + expires_on.as_secs();
            
           Ok("TODO: generate access token".to_string())
        })
    }
}

impl Handler<FetchPlaylist> for SpotifyFetcher {
    type Result = MuuzikaFutureResult<PlaylistInfoWithTracks>;

    fn handle(&mut self, msg: FetchPlaylist, _ctx: &mut Context<Self>) -> Self::Result {
        todo!("Fetch playlist from Spotify")
    }
}