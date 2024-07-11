use dotenv::dotenv;
use serde::Deserialize;
use tauri::State;
use std::{collections::HashMap, env::var, fmt::format, sync::{Mutex, MutexGuard, PoisonError}, time::{Duration, SystemTime}};
use reqwest;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error("the mutex was poisoned")]
    PoisonError(String),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}
// Implement From<PoisonError> for Error to convert it to something we have set up serialization for
impl<T> From<PoisonError<T>> for Error {
    fn from(err: PoisonError<T>) -> Self {
        // We "just" convert the error to a string here
        Error::PoisonError(err.to_string())
    }
}

#[derive(Deserialize, Debug)]
struct AuthResponse {
    access_token: String,
    expires_in: usize,
    refresh_token: String,
    user: AuthResponseUser,
}
#[derive(Deserialize, Debug)]
struct AuthResponseUser {
    id: String,
    email: String,
    email_confirmed_at: String,
}

async fn store_tokens(auth_state_mutex: State<'_, Mutex<AuthState>>, response: reqwest::Response) -> Result<(), Error>  {
    if 200 != response.status() {
        return Ok(())
    }

    let auth_response = response.json::<AuthResponse>().await?;
    let mut auth_state = auth_state_mutex.lock().unwrap();
    auth_state.access_token = Some(auth_response.access_token);
    auth_state.refresh_token = Some(auth_response.refresh_token);
    auth_state.expires = SystemTime::now()
        .checked_add(Duration::new(auth_response.expires_in as u64, 0));

    Ok(())
}

#[tauri::command]
async fn sign_up(email: String, password: String, auth_state_mutex: State<'_, Mutex<AuthState>>) -> Result<(), Error> {
    dotenv().ok();

    let url = format!("{}/signup", var("GO_TRUE").expect("GO_TRUE url not set"));
    let api_key = var("ANON_KEY").expect("ANON_KEY not set");

    let mut body = HashMap::new();
    body.insert("email", &email);
    body.insert("password", &password);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("apikey", reqwest::header::HeaderValue::from_str(&api_key).unwrap());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let response = client
        .post(url)
        .json(&body)
        .send()
        .await?;

    store_tokens(auth_state_mutex, response).await?;

    Ok(())
}

#[tauri::command]
async fn login(email: String, password: String, auth_state_mutex: State<'_, Mutex<AuthState>>) -> Result<(), Error> {
    dotenv().ok();

    let url = format!("{}/token?grant_type=password", var("GO_TRUE").expect("GO_TRUE url not set"));
    let api_key = var("ANON_KEY").expect("ANON_KEY not set");

    let mut body = HashMap::new();
    body.insert("email", &email);
    body.insert("password", &password);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("apikey", reqwest::header::HeaderValue::from_str(&api_key).unwrap());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let response = client
        .post(url)
        .json(&body)
        .send()
        .await?;

    store_tokens(auth_state_mutex, response).await?;

    Ok(())
}

#[tauri::command]
async fn logout(auth_state_mutex: State<'_, Mutex<AuthState>>) -> Result<(), Error> {
    dotenv().ok();

    let url = format!("{}/logout", var("GO_TRUE").expect("GO_TRUE url not set"));
    let api_key = var("ANON_KEY").expect("ANON_KEY not set");

    let mut headers = reqwest::header::HeaderMap::new();

    {

        let mut auth_state = auth_state_mutex.lock().unwrap();

        if auth_state.expires.is_none()  {
            return Ok(())
        }

        let  expires = auth_state.expires.clone().unwrap();

        if SystemTime::now() > expires {
            return Ok(())
        }

        if auth_state.access_token.is_none()  {
            return Ok(())
        }

        let access_token = auth_state.access_token.clone().unwrap();
        let access_token = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", &access_token)).unwrap();
        headers.insert("Authorization", access_token);

        let api_key = reqwest::header::HeaderValue::from_str(&api_key).unwrap();
        headers.insert("apikey", api_key);


        auth_state.access_token = None;
        auth_state.refresh_token = None;
        auth_state.expires = None;

    }

    reqwest::Client::builder()
        .default_headers(headers)
        .build()?
        .post(url)
        .send()
        .await?;

    Ok(())
}

#[derive(serde::Serialize, Default, Clone)]
pub(crate) struct AuthState {
    #[serde(skip_serializing)]
    access_token: Option<String>,
    #[serde(skip_serializing)]
    refresh_token: Option<String>,
    expires: Option<SystemTime>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(Mutex::new(AuthState::default()))
        .invoke_handler(tauri::generate_handler![sign_up, login, logout])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
