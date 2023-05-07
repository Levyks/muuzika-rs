use const_format::concatcp;

pub struct ExternalUrls {
    pub spotify: String,
}

impl ExternalUrls {
    pub const FIELDS: &'static str = "spotify";
}

pub struct Artist {
    pub id: String,
    pub name: String,
    pub external_urls: ExternalUrls,
}

impl Artist {
    pub const FIELDS: &'static str = concatcp!("id,name,external_urls(", ExternalUrls::FIELDS, ")");
}

pub struct Image {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub const FIELDS: &'static str = "url,width,height";
}

pub struct Album {
    pub images: Vec<Image>,
}

impl Album {
    pub const FIELDS: &'static str = concatcp!("images(", Image::FIELDS, ")");
}

pub struct Track {
    pub id: String,
    pub name: String,
    pub preview_url: Option<String>,
    pub external_urls: ExternalUrls,
    pub artists: Vec<Artist>,
    pub album: Album,
}

impl Track {
    pub const FIELDS: &'static str = concatcp!("id,name,preview_url,external_urls(", ExternalUrls::FIELDS, "),artists(", Artist::FIELDS, "),album(", Album::FIELDS, ")");
}

pub struct PlaylistItem {
    pub track: Track,
}

impl PlaylistItem {
    pub const FIELDS: &'static str = concatcp!("track(", Track::FIELDS, ")");
}

pub struct PlaylistItemPagination {
    pub total: u32,
    pub items: Vec<PlaylistItem>,
}

impl PlaylistItemPagination {
    pub const FIELDS: &'static str = concatcp!("total,items(", PlaylistItem::FIELDS, ")");
}

pub struct PlaylistOwner {
    pub display_name: String,
}

impl PlaylistOwner {
    pub const FIELDS: &'static str = "display_name";
}

pub struct PlaylistInfoWithTracks {
    pub id: String,
    pub name: String,
    pub owner: PlaylistOwner,
    pub external_urls: ExternalUrls,
    pub images: Vec<Image>,
    pub tracks: PlaylistItemPagination,
}

impl PlaylistInfoWithTracks {
    pub const FIELDS: &'static str = concatcp!("id,name,owner(", PlaylistOwner::FIELDS, "),external_urls(", ExternalUrls::FIELDS, "),images(", Image::FIELDS, "),tracks(", PlaylistItemPagination::FIELDS, ")");
}

pub enum TokenType {
    Bearer,
}

pub struct AccessToken {
    pub access_token: String,
    pub token_type: TokenType,
    pub expires_in: u32,
}
