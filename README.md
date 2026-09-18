# SSD Delhi backend

Rust/Wasm JSON API for Cloudflare Workers. Supabase Postgres is the system of record
through its REST API, KV provides short-lived response caching and token primitives,
R2 stores media, and Queues are reserved for asynchronous email.

## Local setup

```sh
rustup target add wasm32-unknown-unknown
cargo install worker-build
cp .env.example .dev.vars
# Replace placeholders locally; never paste production credentials into source control.
npx wrangler dev
```

Run `supabase/migrations/0001_init.sql` in the Supabase SQL editor or with the Supabase
CLI. Set `SUPABASE_SERVICE_ROLE_KEY`, `JWT_SECRET`, `TURNSTILE_SECRET`, and
`RESEND_API_KEY` with `wrangler secret put`. The service-role key must remain server-side
and must never be exposed to the browser. Rotate any credentials that were pasted into
chat or terminal history before using this project.

The initial integration exposes real Supabase/KV health checks, cached public settings,
and cached published post reads. Protected mutations must add custom authentication,
Turnstile verification, validation, rate limiting, and audit logging before being enabled.

# SSD_backend
