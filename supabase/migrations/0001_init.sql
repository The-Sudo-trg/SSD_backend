create extension if not exists pgcrypto;

create table users (
  id uuid primary key default gen_random_uuid(),
  email text not null unique,
  password_hash text not null,
  full_name text not null,
  role text not null check (role in ('super_admin', 'admin', 'editor')),
  is_active boolean not null default true,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table refresh_tokens (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references users(id) on delete cascade,
  token_hash text not null,
  expires_at timestamptz not null,
  revoked_at timestamptz,
  created_at timestamptz not null default now()
);

create table password_reset_tokens (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references users(id) on delete cascade,
  token_hash text not null,
  expires_at timestamptz not null,
  used_at timestamptz
);

create table posts (
  id uuid primary key default gen_random_uuid(),
  kind text not null check (kind in ('article', 'blog', 'news')),
  slug text not null unique,
  title text not null,
  excerpt text,
  body_markdown text not null,
  cover_image_key text,
  category text,
  tags jsonb not null default '[]'::jsonb,
  author_id uuid references users(id),
  status text not null default 'draft' check (status in ('draft', 'published', 'archived')),
  published_at timestamptz,
  seo_title text,
  seo_description text,
  view_count integer not null default 0,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);
create index posts_public_listing on posts (kind, status, published_at desc);

create table events (
  id uuid primary key default gen_random_uuid(),
  slug text not null unique,
  title text not null,
  description text,
  location text,
  starts_at timestamptz not null,
  ends_at timestamptz,
  cover_image_key text,
  capacity integer,
  status text not null default 'draft' check (status in ('draft', 'published', 'cancelled')),
  created_by uuid references users(id),
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table event_rsvps (
  id uuid primary key default gen_random_uuid(),
  event_id uuid not null references events(id) on delete cascade,
  name text not null,
  email text not null,
  phone text,
  created_at timestamptz not null default now(),
  unique (event_id, email)
);

create table gallery_albums (
  id uuid primary key default gen_random_uuid(),
  title text not null,
  description text,
  cover_image_key text,
  created_at timestamptz not null default now()
);

create table gallery_images (
  id uuid primary key default gen_random_uuid(),
  album_id uuid not null references gallery_albums(id) on delete cascade,
  r2_key text not null,
  caption text,
  sort_order integer not null default 0,
  created_at timestamptz not null default now()
);

create table pages (
  slug text primary key,
  title text not null,
  body_markdown text not null,
  updated_by uuid references users(id),
  updated_at timestamptz not null default now()
);

create table timeline_milestones (
  id uuid primary key default gen_random_uuid(),
  year integer not null,
  title text not null,
  description text not null,
  sort_order integer not null default 0
);

create table membership_applications (
  id uuid primary key default gen_random_uuid(),
  full_name text not null,
  email text not null,
  phone text,
  city text,
  state text,
  message text,
  status text not null default 'pending' check (status in ('pending', 'approved', 'rejected')),
  reviewed_by uuid references users(id),
  reviewed_at timestamptz,
  created_at timestamptz not null default now()
);

create table contact_messages (
  id uuid primary key default gen_random_uuid(),
  full_name text not null,
  email text not null,
  subject text,
  message text not null,
  status text not null default 'new' check (status in ('new', 'read', 'archived')),
  created_at timestamptz not null default now()
);

create table site_settings (
  key text primary key,
  value jsonb not null,
  updated_by uuid references users(id),
  updated_at timestamptz not null default now()
);

create table audit_logs (
  id uuid primary key default gen_random_uuid(),
  actor_id uuid references users(id),
  action text not null,
  entity_type text not null,
  entity_id uuid,
  metadata jsonb,
  created_at timestamptz not null default now()
);

alter table users enable row level security;
alter table refresh_tokens enable row level security;
alter table password_reset_tokens enable row level security;
alter table posts enable row level security;
alter table events enable row level security;
alter table event_rsvps enable row level security;
alter table gallery_albums enable row level security;
alter table gallery_images enable row level security;
alter table pages enable row level security;
alter table timeline_milestones enable row level security;
alter table membership_applications enable row level security;
alter table contact_messages enable row level security;
alter table site_settings enable row level security;
alter table audit_logs enable row level security;
