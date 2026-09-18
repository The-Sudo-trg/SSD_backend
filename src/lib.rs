mod db;
mod error;
mod handlers;
mod router;

use router::route;
use worker::*;

#[event(fetch)]
pub async fn fetch(req: Request, env: Env, ctx: Context) -> Result<Response> {
    route(req, env, ctx).await
}
