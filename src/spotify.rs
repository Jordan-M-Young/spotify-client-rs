use reqwest;

pub async fn get_tracks(token: String, num_tracks: u16) -> Result<String, reqwest::Error> {
    let bearer = format!("Bearer {}", token);
    let endpoint = format!(
        "v1/me/top/tracks?time_range=short_term&limit={}",
        num_tracks
    );
    let url = format!("https://api.spotify.com/{}", endpoint);
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("Authorization", bearer)
        .send()
        .await?;

    let body = res.text().await?;

    Ok(body)
}
