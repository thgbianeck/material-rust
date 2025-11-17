# 📋 PROJETO FINAL - OPÇÃO B: URL SHORTENER API

**Duração:** 8 dias (53-60)  
**Nível:** Projeto completo aplicando todos os conceitos aprendidos  
**Stack:** Rust + Axum + SQLx + PostgreSQL + Redis

---

## 🎯 VISÃO GERAL

API REST para encurtamento de URLs com sistema de estatísticas detalhadas, rate limiting, cache distribuído, analytics em tempo real e custom aliases.

**Funcionalidades Principais:**
- ✅ Encurtar URLs longas
- ✅ Custom aliases (slugs personalizados)
- ✅ Redirecionamento rápido com cache
- ✅ Analytics detalhadas (clicks, geolocalização, devices)
- ✅ Rate limiting por IP
- ✅ Expiração de URLs
- ✅ QR Code generation
- ✅ Dashboard de estatísticas

---

## 🏗️ ARQUITETURA

### **Diagrama de Componentes**

```mermaid
graph TB
    Client[Client Browser/App]
    LB[Load Balancer]
    API[API Server - Axum]
    Cache[(Redis Cache)]
    DB[(PostgreSQL)]
    Analytics[Analytics Worker]
    
    LB --> API
    
    
    subgraph "High Performance Path"
        Cache
    end
    
    subgraph "Data Layer"
        DB
        Analytics
    end
    
    style Cache fill:#ff6b6b,stroke:#333,stroke-width:2px
    style API fill:#4ecdc4,stroke:#333,stroke-width:2px
    style DB fill:#95e1d3,stroke:#333,stroke-width:2px
```

### **Fluxo de Redirecionamento (Hot Path)**

```mermaid
sequenceDiagram
    participant User
    participant API
    participant Redis
    participant PostgreSQL
    participant Analytics
    
    User->>API: GET /abc123
    API->>Redis: GET url:abc123
    
    alt Cache Hit
        Redis-->>API: Original URL
        API->>Analytics: Log click (async)
        API-->>User: 301 Redirect
    else Cache Miss
        Redis-->>API: null
        API->>PostgreSQL: SELECT url WHERE short_code='abc123'
        PostgreSQL-->>API: URL data
        API->>Redis: SET url:abc123 (TTL 3600s)
        API->>Analytics: Log click (async)
        API-->>User: 301 Redirect
    end
    
    Analytics->>PostgreSQL: INSERT click event
```

---

## 🗄️ DATABASE SCHEMA

### **Diagrama ER**

```mermaid
erDiagram
    
    USERS {
        uuid id PK
        varchar email UK
        varchar password_hash
        varchar name
        varchar api_key UK
        integer rate_limit
        timestamptz created_at
    }
    
    URLS {
        uuid id PK
        uuid user_id FK
        varchar short_code UK
        text original_url
        varchar title
        boolean is_active
        integer click_count
        timestamptz expires_at
        timestamptz created_at
        timestamptz last_accessed
    }
    
    CLICKS {
        uuid id PK
        uuid url_id FK
        varchar ip_address
        varchar user_agent
        varchar country
        varchar city
        varchar device_type
        varchar browser
        varchar referrer
        timestamptz clicked_at
    }
    
    TAGS {
        uuid id PK
        uuid user_id FK
        varchar name UK
        varchar color
    }
    
    URL_TAGS {
        uuid url_id FK
        uuid tag_id FK
    }
```

---

### **SQL Migrations**

**Migration 001: Create Users Table**

```sql
-- migrations/001_create_users.sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255),
    api_key VARCHAR(64) UNIQUE,
    rate_limit INTEGER DEFAULT 100,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_api_key ON users(api_key);

-- Função para gerar API key
CREATE OR REPLACE FUNCTION generate_api_key()
RETURNS TRIGGER AS $$
BEGIN
    NEW.api_key = encode(gen_random_bytes(32), 'hex');
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_api_key
    BEFORE INSERT ON users
    FOR EACH ROW
    EXECUTE FUNCTION generate_api_key();
```

**Migration 002: Create URLs Table**

```sql
-- migrations/002_create_urls.sql
CREATE TABLE urls (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    short_code VARCHAR(10) UNIQUE NOT NULL,
    original_url TEXT NOT NULL,
    title VARCHAR(500),
    is_active BOOLEAN DEFAULT TRUE,
    click_count INTEGER DEFAULT 0,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_accessed TIMESTAMPTZ
);

CREATE INDEX idx_urls_short_code ON urls(short_code);
CREATE INDEX idx_urls_user_id ON urls(user_id);
CREATE INDEX idx_urls_created_at ON urls(created_at DESC);
CREATE INDEX idx_urls_expires_at ON urls(expires_at) WHERE expires_at IS NOT NULL;

-- Função para atualizar click_count
CREATE OR REPLACE FUNCTION increment_click_count()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE urls 
    SET 
        click_count = click_count + 1,
        last_accessed = NEW.clicked_at
    WHERE id = NEW.url_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
```

**Migration 003: Create Clicks Table**

```sql
-- migrations/003_create_clicks.sql
CREATE TABLE clicks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    url_id UUID NOT NULL REFERENCES urls(id) ON DELETE CASCADE,
    ip_address VARCHAR(45),
    user_agent TEXT,
    country VARCHAR(2),
    city VARCHAR(100),
    device_type VARCHAR(20),
    browser VARCHAR(50),
    referrer TEXT,
    clicked_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_clicks_url_id ON clicks(url_id);
CREATE INDEX idx_clicks_clicked_at ON clicks(clicked_at DESC);
CREATE INDEX idx_clicks_country ON clicks(country);
CREATE INDEX idx_clicks_device_type ON clicks(device_type);

-- Particionamento por data (opcional para escala)
-- CREATE TABLE clicks_2025_11 PARTITION OF clicks
--     FOR VALUES FROM ('2025-11-01') TO ('2025-12-01');

-- Trigger para incrementar contador
CREATE TRIGGER after_click_insert
    AFTER INSERT ON clicks
    FOR EACH ROW
    EXECUTE FUNCTION increment_click_count();
```

**Migration 004: Create Tags and URL_Tags**

```sql
-- migrations/004_create_tags.sql
CREATE TABLE tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(50) NOT NULL,
    color VARCHAR(7) DEFAULT '#808080',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(user_id, name)
);

CREATE INDEX idx_tags_user_id ON tags(user_id);

CREATE TABLE url_tags (
    url_id UUID NOT NULL REFERENCES urls(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    PRIMARY KEY (url_id, tag_id)
);

CREATE INDEX idx_url_tags_url_id ON url_tags(url_id);
CREATE INDEX idx_url_tags_tag_id ON url_tags(tag_id);
```

**Migration 005: Create Rate Limiting Table**

```sql
-- migrations/005_create_rate_limits.sql
CREATE TABLE rate_limit_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    identifier VARCHAR(255) NOT NULL, -- IP ou API key
    endpoint VARCHAR(100) NOT NULL,
    request_count INTEGER DEFAULT 1,
    window_start TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(identifier, endpoint, window_start)
);

CREATE INDEX idx_rate_limit_identifier ON rate_limit_records(identifier);
CREATE INDEX idx_rate_limit_window_start ON rate_limit_records(window_start);

-- Limpar registros antigos (rodar via cron ou worker)
CREATE OR REPLACE FUNCTION cleanup_old_rate_limits()
RETURNS void AS $$
BEGIN
    DELETE FROM rate_limit_records 
    WHERE window_start < NOW() - INTERVAL '1 hour';
END;
$$ LANGUAGE plpgsql;
```

---

## 📁 ESTRUTURA DE CÓDIGO

```
url-shortener-api/
├── Cargo.toml
├── .env.example
├── .gitignore
├── docker-compose.yml
├── Dockerfile
├── README.md
│
├── migrations/
│   ├── 001_create_users.sql
│   ├── 002_create_urls.sql
│   ├── 003_create_clicks.sql
│   ├── 004_create_tags.sql
│   └── 005_create_rate_limits.sql
│
├── src/
│   ├── main.rs
│   │
│   ├── config/
│   │   ├── mod.rs
│   │   └── settings.rs
│   │
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── url.rs
│   │   ├── click.rs
│   │   └── tag.rs
│   │
│   ├── dto/
│   │   ├── mod.rs
│   │   ├── url_dto.rs
│   │   ├── stats_dto.rs
│   │   └── auth_dto.rs
│   │
│   ├── repositories/
│   │   ├── mod.rs
│   │   ├── user_repository.rs
│   │   ├── url_repository.rs
│   │   ├── click_repository.rs
│   │   └── tag_repository.rs
│   │
│   ├── services/
│   │   ├── mod.rs
│   │   ├── url_service.rs
│   │   ├── analytics_service.rs
│   │   ├── cache_service.rs
│   │   ├── short_code_generator.rs
│   │   └── qr_service.rs
│   │
│   ├── api/
│   │   ├── mod.rs
│   │   ├── routes.rs
│   │   ├── handlers/
│   │   │   ├── mod.rs
│   │   │   ├── url_handlers.rs
│   │   │   ├── redirect_handler.rs
│   │   │   ├── stats_handlers.rs
│   │   │   └── auth_handlers.rs
│   │   └── middleware/
│   │       ├── mod.rs
│   │       ├── auth_middleware.rs
│   │       ├── rate_limit_middleware.rs
│   │       └── analytics_middleware.rs
│   │
│   ├── cache/
│   │   ├── mod.rs
│   │   └── redis_client.rs
│   │
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── short_code.rs
│   │   ├── validators.rs
│   │   ├── geo_ip.rs
│   │   └── user_agent_parser.rs
│   │
│   ├── workers/
│   │   ├── mod.rs
│   │   ├── analytics_worker.rs
│   │   └── cleanup_worker.rs
│   │
│   ├── error.rs
│   └── db.rs
│
└── tests/
    ├── common/
    │   └── mod.rs
    ├── integration/
    │   ├── shorten_tests.rs
    │   ├── redirect_tests.rs
    │   └── stats_tests.rs
    └── fixtures/
        └── test_data.rs
```

---

## 📦 DEPENDÊNCIAS (Cargo.toml)

```toml
[package]
name = "url-shortener-api"
version = "0.1.0"
edition = "2021"

[dependencies]
# Web Framework
axum = { version = "0.7", features = ["macros"] }
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace", "limit"] }

# Database
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "migrate"] }

# Redis Cache
redis = { version = "0.24", features = ["tokio-comp", "connection-manager"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# URL & Encoding
url = "2.5"
base64 = "0.21"
nanoid = "0.4"

# QR Code
qrcode = "0.14"
image = "0.24"

# Analytics
user-agent-parser = "0.2"
maxminddb = "0.24" # GeoIP

# Authentication
jsonwebtoken = "9"
bcrypt = "0.15"

# Utilities
uuid = { version = "1.6", features = ["serde", "v4"] }
chrono = { version = "0.4", features = ["serde"] }
validator = { version = "0.16", features = ["derive"] }
thiserror = "1.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Config
dotenvy = "0.15"

# Rate Limiting
governor = "0.6"

[dev-dependencies]
reqwest = { version = "0.11", features = ["json"] }
```

---

## 🔌 API ENDPOINTS

### **Fluxo Principal de Uso**

```mermaid
flowchart TD
    Start([Usuário com URL longa]) --> Register{Tem conta?}
    
    
    CreateAccount --> GetToken[Recebe API Key]
    Login --> GetToken
    
    GetToken --> Shorten[POST /api/v1/shorten]
    Shorten --> ReceiveShort[Recebe short URL]
    
    ReceiveShort --> Share[Compartilha URL]
    Share --> Click[Usuário clica no link]
    
    Click --> Redirect[GET /:short_code]
    Redirect --> Cache{Está no cache?}
    
    
    DBLookup --> UpdateCache[Atualiza cache]
    UpdateCache --> RedirectUser[Redirect para URL original]
    FastRedirect --> LogAnalytics[Log analytics assíncrono]
    RedirectUser --> LogAnalytics
    
    LogAnalytics --> ViewStats[GET /api/v1/urls/:id/stats]
    ViewStats --> End([Visualiza estatísticas])
    
    style Shorten fill:#4ecdc4
    style Redirect fill:#ff6b6b
    style ViewStats fill:#95e1d3
```

---

### **Endpoints Detalhados**

#### **Public Endpoints (sem autenticação)**

```
GET    /:short_code                     # Redirect to original URL
GET    /api/v1/health                   # Health check
```

#### **Authentication**

```
POST   /api/v1/auth/register            # Create account
POST   /api/v1/auth/login               # Login
POST   /api/v1/auth/refresh             # Refresh token
GET    /api/v1/auth/me                  # Get current user info
```

#### **URL Management**

```
POST   /api/v1/shorten                  # Create short URL
GET    /api/v1/urls                     # List user's URLs
GET    /api/v1/urls/:id                 # Get URL details
PUT    /api/v1/urls/:id                 # Update URL
DELETE /api/v1/urls/:id                 # Delete URL
PATCH  /api/v1/urls/:id/toggle          # Activate/deactivate
```

#### **Analytics**

```
GET    /api/v1/urls/:id/stats           # Detailed statistics
GET    /api/v1/urls/:id/clicks          # Click history
GET    /api/v1/dashboard                # User dashboard
GET    /api/v1/stats/export             # Export analytics (CSV)
```

#### **QR Code**

```
GET    /api/v1/urls/:id/qrcode          # Generate QR code
```

---

### **Exemplos de Requisições**

#### **POST /api/v1/shorten**

**Request (authenticated):**
```json
POST /api/v1/shorten
Authorization: Bearer <token>
Content-Type: application/json

{
  "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
  "custom_alias": "rickroll",  // opcional
  "expires_in_days": 30,        // opcional
  "title": "Never Gonna Give You Up",
  "tags": ["music", "classic"]
}
```

**Response (201):**
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "short_code": "rickroll",
  "short_url": "https://short.ly/rickroll",
  "original_url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
  "title": "Never Gonna Give You Up",
  "qr_code_url": "https://short.ly/api/v1/urls/123e4567.../qrcode",
  "expires_at": "2025-12-15T00:00:00Z",
  "created_at": "2025-11-15T10:00:00Z"
}
```

**Request (anonymous - com rate limit):**
```json
POST /api/v1/shorten
Content-Type: application/json

{
  "url": "https://github.com/rust-lang/rust"
}
```

**Response (201):**
```json
{
  "short_code": "aB3xK7",
  "short_url": "https://short.ly/aB3xK7",
  "original_url": "https://github.com/rust-lang/rust",
  "expires_at": "2025-11-22T10:00:00Z",
  "note": "Anonymous links expire in 7 days. Register for permanent links."
}
```

---

#### **GET /:short_code (Redirect)**

**Request:**
```http
GET /rickroll
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64)...
X-Forwarded-For: 203.0.113.42
Referer: https://twitter.com/somepost
```

**Response (301):**
```http
HTTP/1.1 301 Moved Permanently
Location: https://www.youtube.com/watch?v=dQw4w9WgXcQ
Cache-Control: public, max-age=300
```

**Analytics Logged (async):**
```json
{
  "url_id": "123e4567-e89b-12d3-a456-426614174000",
  "ip_address": "203.0.113.42",
  "country": "BR",
  "city": "São Paulo",
  "device_type": "desktop",
  "browser": "Chrome",
  "os": "Windows",
  "referrer": "https://twitter.com/somepost",
  "clicked_at": "2025-11-15T14:30:22Z"
}
```

---

#### **GET /api/v1/urls/:id/stats**

**Request:**
```http
GET /api/v1/urls/123e4567-e89b-12d3-a456-426614174000/stats
Authorization: Bearer <token>
```

**Response (200):**
```json
{
  "url": {
    "short_code": "rickroll",
    "original_url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    "title": "Never Gonna Give You Up",
    "created_at": "2025-11-15T10:00:00Z"
  },
  "summary": {
    "total_clicks": 1247,
    "unique_visitors": 891,
    "clicks_today": 42,
    "clicks_this_week": 287,
    "clicks_this_month": 1247
  },
  "by_country": [
    {"country": "US", "clicks": 412, "percentage": 33.04},
    {"country": "BR", "clicks": 298, "percentage": 23.90},
    {"country": "GB", "clicks": 187, "percentage": 14.99}
  ],
  "by_device": [
    {"device": "mobile", "clicks": 687, "percentage": 55.09},
    {"device": "desktop", "clicks": 512, "percentage": 41.06},
    {"device": "tablet", "clicks": 48, "percentage": 3.85}
  ],
  "by_browser": [
    {"browser": "Chrome", "clicks": 623},
    {"browser": "Safari", "clicks": 312},
    {"browser": "Firefox", "clicks": 187}
  ],
  "by_referrer": [
    {"referrer": "twitter.com", "clicks": 456},
    {"referrer": "facebook.com", "clicks": 289},
    {"referrer": "direct", "clicks": 502}
  ],
  "clicks_timeline": [
    {"date": "2025-11-01", "clicks": 45},
    {"date": "2025-11-02", "clicks": 52},
    {"date": "2025-11-03", "clicks": 38}
  ]
}
```

---

#### **GET /api/v1/dashboard**

**Response (200):**
```json
{
  "user": {
    "name": "Thiago Bianeck",
    "email": "bianeck@example.com",
    "plan": "free"
  },
  "summary": {
    "total_urls": 42,
    "total_clicks": 15678,
    "active_urls": 38,
    "expired_urls": 4
  },
  "top_urls": [
    {
      "short_code": "rickroll",
      "title": "Never Gonna Give You Up",
      "clicks": 1247,
      "created_at": "2025-11-15T10:00:00Z"
    }
  ],
  "recent_activity": [
    {
      "short_code": "github",
      "clicked_at": "2025-11-15T14:30:22Z",
      "country": "BR",
      "device": "mobile"
    }
  ],
  "rate_limit": {
    "limit": 100,
    "used": 23,
    "remaining": 77,
    "reset_at": "2025-11-15T15:00:00Z"
  }
}
```

---

## ⚡ CACHE STRATEGY (Redis)

### **Cache Keys Pattern**

```
url:short_code:<code>        -> Original URL data (TTL: 1 hour)
stats:url:<id>:daily         -> Daily click count (TTL: 24 hours)
ratelimit:<identifier>:shorten -> Rate limit counter (TTL: 1 hour)
ratelimit:<ip>:redirect      -> Redirect rate limit (TTL: 1 minute)
```

### **Implementação de Cache**

```rust
// Exemplo de lógica de cache

pub async fn get_url_by_short_code(
    short_code: &str,
    cache: &RedisClient,
    db: &PgPool,
) -> Result<Url, AppError> {
    // 1. Tentar cache primeiro
    let cache_key = format!("url:short_code:{}", short_code);
    
    if let Some(cached) = cache.get(&cache_key).await? {
        return Ok(serde_json::from_str(&cached)?);
    }
    
    // 2. Cache miss - buscar no DB
    let url = sqlx::query_as!(
        Url,
        "SELECT * FROM urls WHERE short_code = $1 AND is_active = true",
        short_code
    )
    .fetch_one(db)
    .await?;
    
    // 3. Atualizar cache (TTL 1 hora)
    let serialized = serde_json::to_string(&url)?;
    cache.set_ex(&cache_key, &serialized, 3600).await?;
    
    Ok(url)
}
```

---

## 🚦 RATE LIMITING

### **Estratégia de Rate Limit**

```mermaid
graph LR
    A[Request] --> B{Authenticated?}
    
    
    C --> E{Limit Exceeded?}
    D --> F{Limit Exceeded?}
    
    
    
    H --> I[Increment Counter]
    I --> J[Response]
    
    style G fill:#ff6b6b
    style H fill:#4ecdc4
```

### **Limites por Endpoint**

```rust
// Rate limits configuration

pub struct RateLimits {
    // Anonymous users (by IP)
    pub anonymous_shorten: RateLimit,    // 10 por hora
    pub anonymous_redirect: RateLimit,   // 100 por minuto
    
    // Authenticated users (by API key)
    pub authenticated_shorten: RateLimit, // 100 por hora
    pub authenticated_stats: RateLimit,   // 1000 por hora
}

pub struct RateLimit {
    pub requests: u32,
    pub per: Duration,
}
```

---

## 🔢 ALGORITMO DE SHORT CODE

### **Estratégias de Geração**

```mermaid
flowchart TD
    Start[Generate Short Code] --> Custom{Custom Alias?}
    
    
    Validate --> Available{Available?}
    
    Generate --> CheckDB{Exists in DB?}
    
    Save --> Success[Return Short Code]
    
    style Generate fill:#4ecdc4
    style Save fill:#95e1d3
    style Error fill:#ff6b6b
```

### **Implementação**

```rust
use nanoid::nanoid;

pub struct ShortCodeGenerator;

impl ShortCodeGenerator {
    // Alfabeto sem caracteres confusos (sem 0, O, I, l)
    const ALPHABET: &'static str = 
        "abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    
    const DEFAULT_LENGTH: usize = 6;
    
    pub fn generate() -> String {
        nanoid!(Self::DEFAULT_LENGTH, &Self::ALPHABET.chars().collect::<Vec<_>>())
    }
    
    pub fn validate_custom(alias: &str) -> Result<(), ValidationError> {
            return Err(ValidationError::new("length"));
        }
        
            return Err(ValidationError::new("invalid_chars"));
        }
        
        // Palavras reservadas
        const RESERVED: &[&str] = &["api", "admin", "health", "stats"];
        if RESERVED.contains(&alias.to_lowercase().as_str()) {
            return Err(ValidationError::new("reserved"));
        }
        
        Ok(())
    }
}
```

---

## 📊 ANALYTICS WORKER

### **Processamento Assíncrono**

```mermaid
sequenceDiagram
    participant Handler
    participant Channel
    participant Worker
    participant DB
    participant Cache
    
    Handler->>Channel: Send Click Event
    Handler-->>User: Immediate Redirect
    
    loop Background Processing
        Worker->>Channel: Receive Events (batch)
        Worker->>Worker: Parse User-Agent
        Worker->>Worker: GeoIP Lookup
        Worker->>DB: Bulk INSERT clicks
        Worker->>Cache: Update aggregated stats
    end
```

### **Implementação do Worker**

```rust
pub struct AnalyticsWorker {
    receiver: mpsc::Receiver<ClickEvent>,
    db: PgPool,
    redis: RedisClient,
    batch_size: usize,
    flush_interval: Duration,
}

impl AnalyticsWorker {
    pub async fn run(mut self) {
        let mut batch = Vec::with_capacity(self.batch_size);
        let mut interval = tokio::time::interval(self.flush_interval);
        
        loop {
            tokio::select! {
                Some(event) = self.receiver.recv() => {
                    batch.push(event);
                    
                    if batch.len() >= self.batch_size {
                        self.flush_batch(&mut batch).await;
                    }
                }
                
                _ = interval.tick() => {
                    if !batch.is_empty() {
                        self.flush_batch(&mut batch).await;
                    }
                }
            }
        }
    }
    
    async fn flush_batch(&self, batch: &mut Vec<ClickEvent>) {
        // Bulk insert no PostgreSQL
        // Atualizar contadores no Redis
        // Processar GeoIP e User-Agent
        
        batch.clear();
    }
}
```

---

## 🖼️ QR CODE GENERATION

### **Endpoint de QR Code**

**Request:**
```http
GET /api/v1/urls/123e4567-e89b-12d3-a456-426614174000/qrcode?size=300&format=png
Authorization: Bearer <token>
```

**Response:**
```http
HTTP/1.1 200 OK
Content-Type: image/png
Content-Disposition: inline; filename="rickroll-qr.png"
Cache-Control: public, max-age=86400

<binary image data>
```

---

## 🐳 DOCKER

### **docker-compose.yml**

```yaml
version: '3.8'

services:
  postgres:
    image: postgres:16-alpine
    container_name: urlshortener-db
    environment:
      POSTGRES_USER: urlshortener
      POSTGRES_PASSWORD: urlshortener_dev
      POSTGRES_DB: urlshortener
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U urlshortener"]
      interval: 5s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    container_name: urlshortener-cache
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 5s
      timeout: 3s
      retries: 5

  api:
    build: .
    container_name: urlshortener-api
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy
    environment:
      DATABASE_URL: postgres://urlshortener:urlshortener_dev@postgres/urlshortener
      REDIS_URL: redis://redis:6379
      JWT_SECRET: your-secret-key-change-in-production
      BASE_URL: http://localhost:8000
      RUST_LOG: info
    ports:
      - "8000:8000"
    volumes:
      - ./:/app

volumes:
  postgres_data:
  redis_data:
```

### **Dockerfile**

```dockerfile
FROM rust:1.75-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Build dependencies (cached layer)
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source code
COPY . .

# Build application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/url-shortener-api .

EXPOSE 8000

CMD ["./url-shortener-api"]
```

---

## 📊 FEATURES POR DIA

### **Dia 53: Setup e Fundação**
- ✅ Setup projeto e dependencies
- ✅ Docker Compose (PostgreSQL + Redis)
- ✅ Migrations completas
- ✅ Estrutura de código
- ✅ Config e environment

### **Dia 54: Core - Short Code Generation**
- ✅ Models (User, URL, Click)
- ✅ URL Repository
- ✅ Short code generator
- ✅ Custom alias validation
- ✅ Testes de geração

### **Dia 55: Redirect e Cache**
- ✅ Redis client setup
- ✅ Cache service
- ✅ Redirect handler (hot path)
- ✅ Performance optimization
- ✅ Cache invalidation

### **Dia 56: Shorten API**
- ✅ Auth service
- ✅ Shorten handler
- ✅ URL management CRUD
- ✅ Rate limiting
- ✅ Validações

### **Dia 57: Analytics**
- ✅ Click repository
- ✅ Analytics worker (async)
- ✅ User-Agent parsing
- ✅ GeoIP integration
- ✅ Stats endpoints

### **Dia 58: QR Code e Extras**
- ✅ QR code generation
- ✅ Tags system
- ✅ URL expiration worker
- ✅ Dashboard endpoint
- ✅ Export analytics

### **Dia 59: Testes e Docs**
- ✅ Unit tests completos
- ✅ Integration tests
- ✅ Load tests (k6/wrk)
- ✅ API documentation
- ✅ README

### **Dia 60: Deploy e Polish**
- ✅ Dockerfile optimization
- ✅ Performance tuning
- ✅ Monitoring setup (opcional)
- ✅ CI/CD (GitHub Actions)
- ✅ Production checklist

---

## 🚀 QUICK START

```bash
# 1. Clone e setup
git clone <repo>
cd url-shortener-api
cp .env.example .env

# 2. Start services
docker-compose up -d

# 3. Run migrations
sqlx migrate run

# 4. Run API
cargo run

# 5. Test endpoints
curl http://localhost:8000/api/v1/health

# 6. Create short URL
curl -X POST http://localhost:8000/api/v1/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://github.com/rust-lang/rust"}'

# 7. Test redirect
curl -I http://localhost:8000/<short_code>
```

---

## 📈 PERFORMANCE TARGETS

### **Latência**
- Redirect (cache hit): < 10ms
- Redirect (cache miss): < 50ms
- Shorten API: < 100ms
- Stats API: < 200ms

### **Throughput**
- Redirects: 10,000 req/s por instância
- Shorten: 1,000 req/s por instância

### **Cache Hit Rate**
- Target: > 95%

---

## 📚 RECURSOS ADICIONAIS

**Crates Importantes:**
- [redis-rs](https://docs.rs/redis/) - Redis client
- [nanoid](https://docs.rs/nanoid/) - Short ID generation
- [qrcode](https://docs.rs/qrcode/) - QR code generation
- [maxminddb](https://docs.rs/maxminddb/) - GeoIP
- [governor](https://docs.rs/governor/) - Rate limiting

**Inspiração:**
- [bit.ly](https://bitly.com/)
- [TinyURL](https://tinyurl.com/)
- [Rebrandly](https://www.rebrandly.com/)

---

**Sistema focado em performance e analytics! Cache agressivo + processamento assíncrono = velocidade máxima! 🚀⚡**
