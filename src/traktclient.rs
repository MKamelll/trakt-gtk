use gtk4::{
    glib::{KeyFile, KeyFileFlags},
    show_uri,
};
use reqwest::Url;
use tiny_http::{Response, Server};

#[derive(serde::Deserialize, serde::Serialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
    created_at: i64,
}

#[derive(Clone, Debug)]
pub struct TraktClient {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    base_uri: &'static str,
    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_at: Option<i64>,
    client: reqwest::Client,
}

impl TraktClient {
    pub fn new() -> Self {
        let f = KeyFile::new();
        f.load_from_file("config.ini", KeyFileFlags::NONE)
            .expect("couldn't load a config.ini file");

        let client_id = f
            .string("app", "client_id")
            .expect("config.ini must have a client_id");

        let client_secret = f
            .string("app", "client_secret")
            .expect("config.ini must have a client_secret");

        let redirect_uri = f
            .string("app", "redirect_uri")
            .expect("config.ini must have a redirect_uri");

        let access_token = match f.string("tokens", "access_token") {
            Ok(t) => Some(t.to_string()),
            Err(_) => None,
        };

        let refresh_token = match f.string("tokens", "refresh_token") {
            Ok(t) => Some(t.to_string()),
            Err(_) => None,
        };

        let expires_at = match f.string("tokens", "expires_at") {
            Ok(t) => Some(
                t.to_string()
                    .parse::<i64>()
                    .expect("expires_at is supposed to be an int"),
            ),
            Err(_) => None,
        };

        Self {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.to_string(),
            base_uri: "https://api.trakt.tv",
            access_token: access_token,
            refresh_token: refresh_token,
            expires_at: expires_at,
            client: reqwest::Client::new(),
        }
    }

    pub async fn login(&mut self) {
        let url = Url::parse_with_params(
            &format!("{}/oauth/authorize", self.base_uri),
            &[
                ("response_type", "code"),
                ("client_id", &self.client_id),
                ("redirect_uri", &self.redirect_uri),
            ],
        )
        .expect("couldn't create the login url")
        .to_string();

        show_uri(
            None::<&gtk4::Window>,
            &url,
            chrono::Utc::now().timestamp() as u32,
        );

        let redirect_uri = self.redirect_uri.clone();
        let code = tokio::task::spawn_blocking(move || {
            let server_uri = redirect_uri
                .trim_start_matches("http://")
                .trim_start_matches("https://")
                .trim_end_matches(|c| c != '/')
                .trim_end_matches('/');

            let server = Server::http(server_uri).expect("couldn't start a server");
            let req = server.recv().expect("couldn't recieve a request");
            let url = req.url();

            if url.contains("/callback") {
                let code = url
                    .split('&')
                    .filter(|s| s.contains("code="))
                    .nth(0)
                    .and_then(|s| s.split('=').nth(1))
                    .and_then(|s| Some(s.to_string()));

                req.respond(Response::from_string("You can close the tab now"))
                    .expect("couldn't send a success response");
                code
            } else {
                req.respond(Response::from_string("Login failed"))
                    .expect("couldn't send a fail response");
                None
            }
        })
        .await
        .expect("starting the server to recieve the code failedd");

        match code {
            Some(c) => self.get_access_token(c).await,
            None => eprintln!("failed to get a code for the token"),
        }
    }

    async fn get_access_token(&mut self, code: String) {
        let url = format!("{}/oauth/token", self.base_uri);
        let res = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("User-Agent", "tracktor/1.0")
            .header("trakt-api-key", &self.client_id)
            .header("trakt-api-version", "2")
            .json(&serde_json::json!({
                "code": code,
                "client_id": &self.client_id,
                "client_secret": &self.client_secret,
                "redirect_uri": &self.redirect_uri,
                "grant_type": "authorization_code"
            }))
            .send()
            .await
            .expect("failed to send the access token request")
            .json::<TokenResponse>()
            .await
            .expect("failed to parse the response as json");

        self.access_token = Some(res.access_token);
        self.refresh_token = Some(res.refresh_token);
        self.expires_at = Some(res.created_at + res.expires_in);

        self.save_tokens();
    }

    fn save_tokens(&self) {
        if let (Some(t), Some(r), Some(e)) =
            (&self.access_token, &self.refresh_token, &self.expires_at)
        {
            let f = KeyFile::new();
            f.load_from_file("config.ini", KeyFileFlags::NONE)
                .expect("couldn't open the config.ini file to write the tokens");

            f.set_string("tokens", "access_token", t);
            f.set_string("tokens", "refresh_token", r);
            f.set_int64("tokens", "expires_at", *e);

            f.save_to_file("config.ini")
                .expect("couldn't save the config.ini file after login");
        }
    }

    pub fn is_logged_in(&self) -> bool {
        match &self.access_token {
            Some(_) => true,
            None => false,
        }
    }
}
