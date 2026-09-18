use crate::handlers;
use worker::*;

pub async fn route(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let origin = env
        .var("FRONTEND_ORIGIN")
        .map(|v| v.to_string())
        .unwrap_or_default();
    let path = req.path();
    let response = match (req.method(), path.as_str()) {
        (Method::Get, "/api/v1/health") => handlers::health(&env).await,
        (Method::Get, "/api/v1/settings/public") => handlers::public_settings(&env).await,
        (Method::Get, "/api/v1/posts") => handlers::posts(req, &env).await,
        _ => handlers::not_implemented().await,
    }?;

    let mut response = response;
    response
        .headers_mut()
        .set("X-Content-Type-Options", "nosniff")?;
    response
        .headers_mut()
        .set("Referrer-Policy", "strict-origin-when-cross-origin")?;
    response.headers_mut().set(
        "Content-Security-Policy",
        "default-src 'none'; frame-ancestors 'none'",
    )?;
    response
        .headers_mut()
        .set("Access-Control-Allow-Origin", &origin)?;
    response
        .headers_mut()
        .set("Access-Control-Allow-Credentials", "true")?;
    Ok(response)
}
