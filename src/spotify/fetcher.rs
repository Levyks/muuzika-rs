use std::time::{SystemTime};
use actix::prelude::*;
use crate::errors::{MuuzikaError};

pub struct SpotifyFetcher {
    client_id: String,
    client_secret: String,
    pub access_token: Option<String>,
    pub access_token_expires_at: u64,
}

impl Actor for SpotifyFetcher {
    type Context = Context<Self>;
}

impl SpotifyFetcher {
    fn new(client_id: String, client_secret: String) -> SpotifyFetcher {
        SpotifyFetcher {
            client_id,
            client_secret,
            access_token: None,
            access_token_expires_at: 0,
        }
    }
    
    pub fn create_and_start(client_id: String, client_secret: String) -> Addr<SpotifyFetcher> {
        let arbiter = Arbiter::new();
        let fetcher = SpotifyFetcher::new(client_id, client_secret);
        SpotifyFetcher::start_in_arbiter(&arbiter.handle(), move |_| fetcher)
    }
    
    pub fn get_access_token_if_not_expired(&self) -> Result<Option<String>, MuuzikaError> {
        match self.access_token {
            Some(ref token) => {
                let now = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_secs();
                
                if now < self.access_token_expires_at {
                    return Ok(Some(token.clone()));
                }
                
                Ok(None)
            }
            None => Ok(None),
        }
    }
}