use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub fn oauth_url(client_id: &str, state: &str, redirect_url: &str) -> String {
    return format!("https://daldal.so/oauth/authorize?response_type=code&client_id={}&state={}&redirect_uri={}", client_id, state, urlencoding::encode(redirect_url))
}

pub enum GrantType {
    AuthorizationCode,
    RefreshToken,
    Flush
}

impl GrantType {
    fn to_string(&self) -> &'static str {
        match self {
            GrantType::AuthorizationCode => "authorization_code",
            GrantType::RefreshToken => "refresh_token",
            GrantType::Flush => "flush"
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub enum TokenErrors {
    #[serde(alias = "code")]
    Code,
    #[serde(alias = "refresh_token")]
    RefreshToken,
    #[serde(alias = "grant_type")]
    GrantType
}

impl TokenErrors {
    pub fn to_string(&self) -> &'static str {
        match self {
            TokenErrors::Code => "허가 토큰이 올바르지 않습니다.",
            TokenErrors::RefreshToken => "갱신 토큰이 올바르지 않습니다.",
            TokenErrors::GrantType => "grant_type이 올바르지 않습니다."
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct OauthResponse {
    access_token: Option<String>,
    refresh_token: Option<String>,
    token_type: Option<String>,
    expires_in: Option<usize>,
    #[serde(default)]
    error: Option<TokenErrors>
}

impl OauthResponse {
    pub fn access_token(&self) -> Option<String> {
        self.access_token.clone()
    }
    pub fn refresh_token(&self) -> Option<String> {
        self.refresh_token.clone()
    }
    pub fn token_type(&self) -> Option<String> {
        self.token_type.clone()
    }
    pub fn expires_in(&self) -> Option<usize> {
        self.expires_in
    }
    pub fn error(&self) -> Option<TokenErrors> {
        self.error.clone()
    }
}

pub fn get_oauth_token(client_id: &'static str, client_secret: &'static str, grant_type: GrantType, code: Option<&str>) -> Option<OauthResponse> {
    let mut form = HashMap::new();
    form.insert("client_id", client_id);
    form.insert("client_secret", client_secret);
    form.insert("grant_type", grant_type.to_string());

    if (grant_type.to_string() == "authorization_code" || grant_type.to_string() == "refresh_token") && code.is_some()  {
        form.insert("code", code.unwrap());
    }

    let client = reqwest::blocking::Client::new();
    let res = client.post("https://daldal.so/oauth/token").form(&form)
        .header(reqwest::header::USER_AGENT, "AkiaCode/balbalso").send().unwrap().text().unwrap();
    let json = serde_json::from_str(&res);

    if json.is_ok() {
        Some(json.unwrap())
    } else {
        None
    }
}
#[derive(Serialize, Deserialize)]
pub struct MeResponse {
    key: String,
    name: Option<String>,
    account: Option<String>,
    libra: LibraOrStar,
    foveon: usize,
    profile: Profile
}

impl MeResponse {
    pub fn key(&self) -> String {
        self.key.clone()
    }
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }
    pub fn account(&self) -> Option<String> {
        self.account.clone()
    }
    pub fn libra(&self) -> LibraOrStar {
        self.libra.clone()
    }
    pub fn foveon(&self) -> usize {
        self.foveon
    }
    pub fn profile(&self) -> Profile {
        self.profile.clone()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Profile {
    image: String,
    text: String
}
impl Profile {
    pub fn image(&self) -> String {
        self.image.clone()
    }
    pub fn text(&self) -> String {
        self.text.clone()
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct LibraOrStar {
    level: usize,
    prev: usize,
    next: usize
}

impl LibraOrStar {
    pub fn level(&self) -> usize {
        self.level
    }
    pub fn prev(&self) -> usize {
        self.prev
    }
    pub fn next(&self) -> usize {
        self.next
    }
}

pub fn me(token: &str) -> Option<MeResponse> {
    let res = request("me", token);
    let json = serde_json::from_str(&res);

    if json.is_ok() {
        Some(json.unwrap())
    } else {
        None
    }
}
#[derive(Serialize, Deserialize)]
pub struct InventoryResponse {
    #[serde(alias = "static")]
    item: Item,
    seq: usize,
    star: LibraOrStar,
    quantity: usize,
    mileage: usize,
    minutes: Vec<String>,
}

impl InventoryResponse {
    pub fn item(&self) -> Item {
        self.item.clone()
    }
    pub fn seq(&self) -> usize {
        self.seq
    }
    pub fn star(&self) -> LibraOrStar {
        self.star.clone()
    }
    pub fn quantity(&self) -> usize {
        self.quantity
    }
    pub fn mileage(&self) -> usize {
        self.mileage
    }
    pub fn minutes(&self) -> Vec<String> {
        self.minutes.clone()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    id: String,
    name: String,
    desc: String
}

impl Item {
    pub fn id(&self) -> String {
        self.id.clone()
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn desc(&self) -> String {
        self.desc.clone()
    }
}

pub fn inventory(token: &str) -> Option<Vec<InventoryResponse>> {
    let res = request("me", token);
    let json = serde_json::from_str(&res);

    if json.is_ok() {
        Some(json.unwrap())
    } else {
        None
    }
}


pub fn moon(token: &str) -> Option<serde_json::Value> {
    let res = request("moon", token);
    let json = serde_json::from_str(&res);

    if json.is_ok() {
        Some(json.unwrap())
    } else {
        None
    }
}

pub fn transceivosome(token: &str) -> Option<serde_json::Value> {
    let res = request("transceivosome", token);
    let json = serde_json::from_str(&res);

    if json.is_ok() {
        Some(json.unwrap())
    } else {
        None
    }
}

fn request(api: &str, token: &str) -> String {
    let url = format!("https://daldal.so/oauth/api/{}", api);

    let client = reqwest::blocking::Client::new();
    let res = client.get(url).header("Authorization", format!("Bearer {}", token))
        .header(reqwest::header::USER_AGENT, "AkiaCode/balbalso").send().unwrap().text().unwrap();
    return res;
}