use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct SpotifyResponse{
    href: String,
    items: Vec<Album>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Album{
    album: AlbumInfo
}

#[derive(Serialize, Deserialize, Debug)]
struct AlbumInfo{
    release_date: String,
    album_type: String,
    id: String,
    uri: String,
    total_tracks: u16,
    r#type: String,
    name: String,
    available_markets: Vec<Value>,
    artists: Vec<Value>,
    external_urls: Value,
    href: String,
    release_date_precision: String,
    images: Vec<Value>

}


struct Artists{
    external_urls: Value,
    href: String,
    id: String,
    name: String,
    r#type: String,
    uril: String
}
