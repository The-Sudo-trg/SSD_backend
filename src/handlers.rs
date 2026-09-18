use crate::db;
use crate::error::json_error;
use serde_json::json;
use worker::*;

pub async fn health(env: &Env) -> Result<Response> {
    let supabase_status = if db::reachable(env).await {
        "ok"
    } else {
        "error"
    };
    let kv_status = env.kv("CACHE_KV").map(|_| "ok").unwrap_or("error");

    Response::from_json(&json!({
        "status": if supabase_status == "ok" && kv_status == "ok" { "ok" } else { "degraded" },
        "checks": { "supabase": supabase_status, "kv": kv_status }
    }))
}

pub async fn public_settings(env: &Env) -> Result<Response> {
    let result = db::cached_json(
        env,
        "public:settings",
        60,
        db::select(
            env,
            "site_settings",
            "select=key,value&key=in.(stats,footer,socials)",
        ),
    )
    .await?;
    Response::from_json(&json!({ "data": result }))
}

pub async fn posts(req: Request, env: &Env) -> Result<Response> {
    if req.method() != Method::Get {
        return crate::error::method_not_allowed();
    }
    let result = db::cached_json(env, "public:posts", 30, db::select(
        env, "posts", "select=id,kind,slug,title,excerpt,cover_image_key,category,tags,published_at&status=eq.published&order=published_at.desc&limit=50"
    )).await?;
    Response::from_json(&json!({ "data": result }))
}

pub async fn not_implemented() -> Result<Response> {
    json_error(501, "NOT_IMPLEMENTED", "this endpoint is not enabled yet")
}
