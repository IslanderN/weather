use reqwest::Response;

pub async fn get_request(url: &str) -> Result<Response, String> {
    reqwest::get(url)
        .await
        .and_then(|r| {r.error_for_status()})
        .map_err(stringify_reqwest_err)
}

pub fn stringify_reqwest_err (err: reqwest::Error) -> String {
    err.to_string()
}