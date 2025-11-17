# 📋 PROJETO FINAL - OPÇÃO A: TASK MANAGER API

**Duração:** 8 dias (53-60)  
**Nível:** Projeto completo aplicando todos os conceitos aprendidos  
**Stack:** Rust + Axum + SQLx + PostgreSQL

---

## 🎯 VISÃO GERAL

API REST completa para gerenciamento de tarefas (TODO list) com autenticação, usuários, tags, filtros avançados e persistência em banco de dados.

**Funcionalidades Principais:**
- ✅ Autenticação JWT
- ✅ CRUD completo de tarefas
- ✅ Sistema de usuários
- ✅ Tags e categorias
- ✅ Filtros e busca avançada
- ✅ Compartilhamento de tarefas
- ✅ Estatísticas e dashboard

---

## 🏗️ ARQUITETURA

### **Camadas da Aplicação**

```mermaid
graph TD
    
    style B fill:#f9f,stroke:#333,stroke-width:2px
    style D fill:#bbf,stroke:#333,stroke-width:2px
    style E fill:#bfb,stroke:#333,stroke-width:2px
    style F fill:#fbb,stroke:#333,stroke-width:2px
```

---

## 🗄️ DATABASE SCHEMA

### **Diagrama ER**

```mermaid
erDiagram
    
    USERS {
        uuid id PK
        varchar username UK
        varchar email UK
        varchar password_hash
        varchar full_name
        timestamptz created_at
        timestamptz updated_at
        timestamptz last_login
    }
    
    TASKS {
        uuid id PK
        uuid user_id FK
        varchar title
        text description
        varchar status
        varchar priority
        timestamptz due_date
        timestamptz completed_at
        timestamptz created_at
        timestamptz updated_at
    }
    
    TAGS {
        uuid id PK
        uuid user_id FK
        varchar name
        varchar color
        timestamptz created_at
    }
    
    TASK_TAGS {
        uuid task_id FK
        uuid tag_id FK
        timestamptz created_at
    }
    
    SHARED_TASKS {
        uuid id PK
        uuid task_id FK
        uuid owner_id FK
        uuid shared_with_id FK
        varchar permission
        timestamptz created_at
    }
```

---

### **SQL Migrations**

**Migration 001: Create Users Table**

```sql
-- migrations/001_create_users.sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login TIMESTAMPTZ
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
```

**Migration 002: Create Tasks Table**

```sql
-- migrations/002_create_tasks.sql
CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    priority VARCHAR(20) NOT NULL DEFAULT 'medium',
    due_date TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT status_check CHECK (status IN ('pending', 'in_progress', 'completed', 'archived')),
    CONSTRAINT priority_check CHECK (priority IN ('low', 'medium', 'high', 'urgent'))
);

CREATE INDEX idx_tasks_user_id ON tasks(user_id);
CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_tasks_due_date ON tasks(due_date);
CREATE INDEX idx_tasks_created_at ON tasks(created_at);
```

**Migration 003: Create Tags and Task_Tags**

```sql
-- migrations/003_create_tags.sql
CREATE TABLE tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(50) NOT NULL,
    color VARCHAR(7) DEFAULT '#808080',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(user_id, name)
);

CREATE INDEX idx_tags_user_id ON tags(user_id);

CREATE TABLE task_tags (
    task_id UUID NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    PRIMARY KEY (task_id, tag_id)
);

CREATE INDEX idx_task_tags_task_id ON task_tags(task_id);
CREATE INDEX idx_task_tags_tag_id ON task_tags(tag_id);
```

**Migration 004: Create Shared Tasks**

```sql
-- migrations/004_create_shared_tasks.sql
CREATE TABLE shared_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    shared_with_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    permission VARCHAR(20) NOT NULL DEFAULT 'read',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT permission_check CHECK (permission IN ('read', 'write')),
    UNIQUE(task_id, shared_with_id)
);

CREATE INDEX idx_shared_tasks_task_id ON shared_tasks(task_id);
CREATE INDEX idx_shared_tasks_shared_with_id ON shared_tasks(shared_with_id);
```

---

## 📁 ESTRUTURA DE CÓDIGO

```
task-manager-api/
├── Cargo.toml
├── .env.example
├── .gitignore
├── docker-compose.yml
├── README.md
│
├── migrations/
│   ├── 001_create_users.sql
│   ├── 002_create_tasks.sql
│   ├── 003_create_tags.sql
│   └── 004_create_shared_tasks.sql
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
│   │   ├── task.rs
│   │   ├── tag.rs
│   │   └── shared_task.rs
│   │
│   ├── dto/
│   │   ├── mod.rs
│   │   ├── auth_dto.rs
│   │   ├── task_dto.rs
│   │   ├── user_dto.rs
│   │   └── tag_dto.rs
│   │
│   ├── repositories/
│   │   ├── mod.rs
│   │   ├── user_repository.rs
│   │   ├── task_repository.rs
│   │   ├── tag_repository.rs
│   │   └── shared_task_repository.rs
│   │
│   ├── services/
│   │   ├── mod.rs
│   │   ├── auth_service.rs
│   │   ├── user_service.rs
│   │   ├── task_service.rs
│   │   └── tag_service.rs
│   │
│   ├── api/
│   │   ├── mod.rs
│   │   ├── routes.rs
│   │   ├── handlers/
│   │   │   ├── mod.rs
│   │   │   ├── auth_handlers.rs
│   │   │   ├── user_handlers.rs
│   │   │   ├── task_handlers.rs
│   │   │   └── tag_handlers.rs
│   │   └── middleware/
│   │       ├── mod.rs
│   │       ├── auth_middleware.rs
│   │       └── logging_middleware.rs
│   │
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── jwt.rs
│   │   ├── password.rs
│   │   └── validators.rs
│   │
│   ├── error.rs
│   └── db.rs
│
└── tests/
    ├── common/
    │   └── mod.rs
    ├── integration/
    │   ├── auth_tests.rs
    │   ├── task_tests.rs
    │   └── user_tests.rs
    └── fixtures/
        └── test_data.rs
```

---

## 📦 DEPENDÊNCIAS (Cargo.toml)

```toml
[package]
name = "task-manager-api"
version = "0.1.0"
edition = "2021"

[dependencies]
# Web Framework
axum = { version = "0.7", features = ["macros"] }
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }

# Database
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "migrate"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

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

[dev-dependencies]
reqwest = { version = "0.11", features = ["json"] }
```

---

## 🔌 API ENDPOINTS

### **Fluxo de Autenticação**

```mermaid
sequenceDiagram
    participant Client
    participant API
    participant AuthService
    participant Database
    participant JWT
    
    Client->>API: POST /auth/register
    API->>AuthService: register(email, password)
    AuthService->>Database: INSERT user
    Database-->>AuthService: user created
    AuthService-->>API: success
    API-->>Client: 201 Created
    
    Client->>API: POST /auth/login
    API->>AuthService: login(email, password)
    AuthService->>Database: SELECT user
    Database-->>AuthService: user data
    AuthService->>AuthService: verify password
    AuthService->>JWT: generate token
    JWT-->>AuthService: access_token
    AuthService-->>API: token + user
    API-->>Client: 200 OK + JWT
    
    Client->>API: GET /tasks (Authorization: Bearer token)
    API->>AuthService: verify_token(token)
    AuthService->>JWT: decode & validate
    JWT-->>AuthService: user_id
    AuthService-->>API: authenticated
    API->>Database: SELECT tasks WHERE user_id
    Database-->>API: tasks
    API-->>Client: 200 OK + tasks
```

### **Endpoints Detalhados**

#### **Authentication**

```
POST   /api/v1/auth/register
POST   /api/v1/auth/login
POST   /api/v1/auth/refresh
POST   /api/v1/auth/logout
GET    /api/v1/auth/me
```

**POST /api/v1/auth/register**
```json
Request:
{
  "username": "bianeck",
  "email": "bianeck@example.com",
  "password": "SecureP@ss123",
  "full_name": "Thiago Bianeck"
}

Response (201):
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "username": "bianeck",
  "email": "bianeck@example.com",
  "full_name": "Thiago Bianeck",
  "created_at": "2025-11-15T10:00:00Z"
}
```

**POST /api/v1/auth/login**
```json
Request:
{
  "email": "bianeck@example.com",
  "password": "SecureP@ss123"
}

Response (200):
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 3600,
  "user": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "username": "bianeck",
    "email": "bianeck@example.com"
  }
}
```

#### **Tasks**

```
GET    /api/v1/tasks                    # List all tasks (with filters)
POST   /api/v1/tasks                    # Create new task
GET    /api/v1/tasks/:id                # Get task by ID
PUT    /api/v1/tasks/:id                # Update task
DELETE /api/v1/tasks/:id                # Delete task
PATCH  /api/v1/tasks/:id/status         # Update task status
PATCH  /api/v1/tasks/:id/complete       # Mark as completed
GET    /api/v1/tasks/stats              # Get statistics
```

**GET /api/v1/tasks (com filtros)**
```
Query Parameters:
- tags: tag1,tag2,tag3
- due_before: 2025-12-31
- due_after: 2025-11-01
- search: search term
- order: asc|desc
- page: 1
- per_page: 20

Example: GET /api/v1/tasks?status=pending&priority=high&sort=due_date&order=asc

Response (200):
{
  "data": [
    {
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "title": "Estudar Rust - Dia 15",
      "description": "Ownership e Borrowing",
      "status": "pending",
      "priority": "high",
      "due_date": "2025-11-29T23:59:59Z",
      "tags": [
        {"id": "tag-id-1", "name": "rust", "color": "#f74c00"},
        {"id": "tag-id-2", "name": "estudos", "color": "#0066cc"}
      ],
      "created_at": "2025-11-15T10:00:00Z",
      "updated_at": "2025-11-15T10:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 45,
    "total_pages": 3
  }
}
```

**POST /api/v1/tasks**
```json
Request:
{
  "title": "Implementar autenticação JWT",
  "description": "Criar sistema de login com tokens JWT",
  "priority": "high",
  "due_date": "2025-12-01T18:00:00Z",
  "tag_ids": ["tag-id-1", "tag-id-2"]
}

Response (201):
{
  "id": "new-task-id",
  "title": "Implementar autenticação JWT",
  "description": "Criar sistema de login com tokens JWT",
  "status": "pending",
  "priority": "high",
  "due_date": "2025-12-01T18:00:00Z",
  "tags": [...],
  "created_at": "2025-11-15T14:30:00Z",
  "updated_at": "2025-11-15T14:30:00Z"
}
```

**PATCH /api/v1/tasks/:id/complete**
```json
Response (200):
{
  "id": "task-id",
  "status": "completed",
  "completed_at": "2025-11-15T15:45:00Z",
  "updated_at": "2025-11-15T15:45:00Z"
}
```

**GET /api/v1/tasks/stats**
```json
Response (200):
{
  "total": 45,
  "by_status": {
    "pending": 20,
    "in_progress": 10,
    "completed": 15,
    "archived": 0
  },
  "by_priority": {
    "low": 5,
    "medium": 25,
    "high": 12,
    "urgent": 3
  },
  "overdue": 3,
  "due_today": 2,
  "due_this_week": 8,
  "completion_rate": 33.33
}
```

#### **Tags**

```
GET    /api/v1/tags                     # List all user tags
POST   /api/v1/tags                     # Create new tag
PUT    /api/v1/tags/:id                 # Update tag
DELETE /api/v1/tags/:id                 # Delete tag
```

#### **Shared Tasks**

```
POST   /api/v1/tasks/:id/share          # Share task with another user
GET    /api/v1/tasks/shared             # List tasks shared with me
DELETE /api/v1/tasks/:id/share/:user_id # Revoke sharing
```

---

## 🔐 AUTENTICAÇÃO JWT

### **Token Structure**

```json
{
  "header": {
    "alg": "HS256",
    "typ": "JWT"
  },
  "payload": {
    "sub": "user-id-uuid",
    "email": "bianeck@example.com",
    "exp": 1700000000,
    "iat": 1699996400
  },
  "signature": "..."
}
```

### **Middleware Flow**

```mermaid
flowchart TD
    A[Request] --> B{Has Authorization Header?}
    J --> K[Continue to Handler]
```

---

## ✅ VALIDAÇÕES

### **User Validation**

```rust
// Exemplo de validação com validator crate
#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    #[validate(custom = "validate_password_strength")]
    pub password: String,
    
    #[validate(length(max = 255))]
    pub full_name: Option<String>,
}

fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    
    if has_uppercase && has_lowercase && has_digit {
        Ok(())
    } else {
        Err(ValidationError::new("password_weak"))
    }
}
```

---

## 🚨 ERROR HANDLING

### **Error Types**

```rust
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Authentication failed: {0}")]
    Authentication(String),
    
    #[error("Authorization failed: {0}")]
    Authorization(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Internal server error")]
    Internal,
}
```

### **Error Response Format**

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid input data",
    "details": {
      "field": "email",
      "issue": "Invalid email format"
    }
  },
  "timestamp": "2025-11-15T10:00:00Z",
  "path": "/api/v1/auth/register"
}
```

---

## 🧪 TESTES

### **Estrutura de Testes**

```rust
// tests/integration/task_tests.rs

#[tokio::test]
async fn test_create_task_success() {
    let app = setup_test_app().await;
    let token = create_test_user_and_login(&app).await;
    
    let response = app
        .post("/api/v1/tasks")
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "title": "Test Task",
            "priority": "high"
        }))
        .await;
    
    assert_eq!(response.status(), 201);
    
    let task: TaskResponse = response.json().await;
    assert_eq!(task.title, "Test Task");
    assert_eq!(task.priority, "high");
}

#[tokio::test]
async fn test_create_task_unauthorized() {
    let app = setup_test_app().await;
    
    let response = app
        .post("/api/v1/tasks")
        .json(&json!({"title": "Test"}))
        .await;
    
    assert_eq!(response.status(), 401);
}
```

---

## 📊 FEATURES POR DIA

### **Dia 53: Setup e Fundação**
- ✅ Configurar projeto e dependencies
- ✅ Docker Compose para PostgreSQL
- ✅ Migrations básicas
- ✅ Estrutura de pastas
- ✅ Config e environment variables

### **Dia 54: Models e Database**
- ✅ Implementar models (User, Task, Tag)
- ✅ Repository layer completo
- ✅ Conexão com database
- ✅ Testes de repository

### **Dia 55: Service Layer**
- ✅ Auth service (register, login, JWT)
- ✅ Task service (CRUD)
- ✅ Tag service
- ✅ Validações

### **Dia 56: API Handlers**
- ✅ Routes setup
- ✅ Auth handlers
- ✅ Task handlers
- ✅ Error handling

### **Dia 57: Features Avançadas**
- ✅ Filtros e busca
- ✅ Paginação
- ✅ Shared tasks
- ✅ Estatísticas

### **Dia 58: Testes**
- ✅ Unit tests
- ✅ Integration tests
- ✅ API tests
- ✅ Coverage

### **Dia 59: Documentação**
- ✅ README completo
- ✅ API documentation
- ✅ Deployment guide
- ✅ Architecture docs

### **Dia 60: Polish e Deploy**
- ✅ Refatoração
- ✅ Performance tuning
- ✅ Dockerfile
- ✅ CI/CD (opcional)

---

## 🐳 DOCKER

### **docker-compose.yml**

```yaml
version: '3.8'

services:
  postgres:
    image: postgres:16-alpine
    container_name: taskmanager-db
    environment:
      POSTGRES_USER: taskmanager
      POSTGRES_PASSWORD: taskmanager_dev
      POSTGRES_DB: taskmanager
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U taskmanager"]
      interval: 5s
      timeout: 5s
      retries: 5

  api:
    build: .
    container_name: taskmanager-api
    depends_on:
      postgres:
        condition: service_healthy
    environment:
      DATABASE_URL: postgres://taskmanager:taskmanager_dev@postgres/taskmanager
      JWT_SECRET: your-secret-key-change-in-production
      RUST_LOG: info
    ports:
      - "8000:8000"
    volumes:
      - ./:/app

volumes:
  postgres_data:
```

---

## 🚀 QUICK START

```bash
# 1. Clone e setup
git clone <repo>
cd task-manager-api
cp .env.example .env

# 2. Start database
docker-compose up -d postgres

# 3. Run migrations
sqlx migrate run

# 4. Run API
cargo run

# 5. Test
curl http://localhost:8000/health
```

---

## 📚 RECURSOS DE APRENDIZADO

**Crates Importantes:**
- [Axum](https://docs.rs/axum/) - Web framework
- [SQLx](https://docs.rs/sqlx/) - SQL toolkit
- [jsonwebtoken](https://docs.rs/jsonwebtoken/) - JWT
- [validator](https://docs.rs/validator/) - Validations

**Tutoriais Relacionados:**
- [Building a REST API with Axum](https://blog.logrocket.com/building-rest-api-rust-axum/)
- [SQLx Tutorial](https://github.com/launchbadge/sqlx/tree/main/examples)

---

**Este é o projeto mais completo! Aplica todos os conceitos aprendidos nos 52 dias anteriores. 🚀**
