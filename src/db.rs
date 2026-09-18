use serde_json::Value;
use wasm_bindgen::JsValue;
use worker::*;

fn config(env: &Env) -> Result<(String, String)> {
    let base = env.var("SUPABASE_URL")?.to_string();
    let key = env.secret("SUPABASE_SERVICE_ROLE_KEY")?.to_string();
    if !base.starts_with("https://") || key.is_empty() {
        return Err("invalid Supabase configuration".into());
    }
    Ok((base.trim_end_matches('/').to_string(), key))
}

pub async fn select(env: &Env, resource: &str, query: &str) -> Result<Value> {
    let (base, key) = config(env)?;
    let url = format!("{base}/rest/v1/{resource}?{query}");
    let mut request = Request::new(&url, Method::Get)?;
    request.headers_mut()?.set("apikey", &key)?;
    request
        .headers_mut()?
        .set("Authorization", &format!("Bearer {key}"))?;
    request.headers_mut()?.set("Accept", "application/json")?;
    let mut response = Fetch::Request(request).send().await?;
    let status = response.status_code();
    let body = response.text().await?;
    if !(200..300).contains(&status) {
        return Err(format!("Supabase request failed with status {status}").into());
    }
    serde_json::from_str(&body).map_err(Into::into)
}

pub async fn reachable(env: &Env) -> bool {
    select(env, "site_settings", "select=key&limit=1")
        .await
        .is_ok()
}

pub async fn cached_json(
    env: &Env,
    key: &str,
    ttl: u64,
    load: impl std::future::Future<Output = Result<Value>>,
) -> Result<Value> {
    let kv = env.kv("CACHE_KV")?;
    if let Some(cached) = kv.get(key).json::<Value>().await? {
        return Ok(cached);
    }
    let value = load.await?;
    kv.put(key, serde_json::to_string(&value)?)?
        .expiration_ttl(ttl)
        .execute()
        .await?;
    Ok(value)
}

#[allow(dead_code)]
fn _json_body(value: &Value) -> JsValue {
    JsValue::from_str(&value.to_string())
}
