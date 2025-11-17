# 📋 PROJETO FINAL - OPÇÃO C: CHAT SERVER

**Duração:** 8 dias (53-60)  
**Nível:** Projeto completo aplicando todos os conceitos aprendidos  
**Stack:** Rust + Axum + WebSockets + SQLx + PostgreSQL + Redis

---

## 🎯 VISÃO GERAL

Sistema de chat em tempo real com WebSockets, rooms/channels, mensagens persistidas, presença online, typing indicators, file sharing e notificações.

**Funcionalidades Principais:**
- ✅ WebSocket real-time messaging
- ✅ Rooms/Channels públicos e privados
- ✅ Direct Messages (DMs) 1-on-1
- ✅ Presença online/offline
- ✅ Typing indicators
- ✅ Message history com pagination
- ✅ File/Image sharing
- ✅ Reactions e threads
- ✅ Push notifications

---

## 🏗️ ARQUITETURA

### **Diagrama de Sistema**

```mermaid
graph TB
    Client1[Client 1 - WebSocket]
    Client2[Client 2 - WebSocket]
    Client3[Client 3 - WebSocket]
    
    LB[Load Balancer]
    
    Server1[Chat Server 1]
    Server2[Chat Server 2]
    
    PubSub[(Redis Pub/Sub)]
    Cache[(Redis Cache)]
    DB[(PostgreSQL)]
    Storage[File Storage S3]
    
    
    LB --> Server1
    LB --> Server2
    
    
    Server1 --> Cache
    Server2 --> Cache
    
    Server1 --> DB
    Server2 --> DB
    
    Server1 --> Storage
    Server2 --> Storage
    
    style PubSub fill:#ff6b6b,stroke:#333,stroke-width:2px
    style Server1 fill:#4ecdc4,stroke:#333,stroke-width:2px
    style Server2 fill:#4ecdc4,stroke:#333,stroke-width:2px
```

### **Fluxo de Mensagem Real-Time**

```mermaid
sequenceDiagram
    participant C1 as Client 1
    participant S1 as Server 1
    participant Redis as Redis Pub/Sub
    participant S2 as Server 2
    participant C2 as Client 2
    participant DB as PostgreSQL
    
    C1->>S1: Send Message (WS)
    
    S1->>DB: INSERT message
    DB-->>S1: message_id
    
    S1->>Redis: PUBLISH channel:room_123
    
    S1->>C1: ACK (message delivered)
    
    Redis-->>S1: message event
    Redis-->>S2: message event
    
    S1->>C1: Broadcast message
    S2->>C2: Broadcast message
    
    Note over C1,C2: Real-time delivery < 50ms
```

### **WebSocket Connection Lifecycle**

```mermaid
stateDiagram-v2
    [*] --> Connecting: Client initiates WS
    Connecting --> Authenticating: Connection established
    Authenticating --> Connected: Auth token valid
    Authenticating --> Disconnected: Auth failed
    
    Connected --> Subscribed: Join room
    Subscribed --> Typing: User typing
    Typing --> Subscribed: Stop typing
    
    Subscribed --> MessageSent: Send message
    MessageSent --> Subscribed: Message delivered
    
    Subscribed --> Unsubscribed: Leave room
    Unsubscribed --> Connected: Left room
    
    Connected --> Disconnected: Client closes
    Subscribed --> Disconnected: Connection lost
    
    Disconnected --> [*]
    
    Connected --> Reconnecting: Network error
    Reconnecting --> Connected: Reconnected
    Reconnecting --> Disconnected: Max retries
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
        varchar display_name
        varchar avatar_url
        varchar status
        timestamptz last_seen
        boolean is_online
        timestamptz created_at
    }
    
    ROOMS {
        uuid id PK
        varchar name
        varchar type
        varchar description
        uuid created_by FK
        boolean is_private
        integer max_members
        timestamptz created_at
    }
    
    USER_ROOMS {
        uuid id PK
        uuid user_id FK
        uuid room_id FK
        varchar role
        timestamptz joined_at
        timestamptz last_read_at
        boolean muted
    }
    
    MESSAGES {
        uuid id PK
        uuid room_id FK
        uuid user_id FK
        uuid reply_to_id FK
        text content
        varchar message_type
        boolean is_edited
        boolean is_deleted
        timestamptz created_at
        timestamptz updated_at
    }
    
    MESSAGE_ATTACHMENTS {
        uuid id PK
        uuid message_id FK
        varchar file_name
        varchar file_type
        varchar file_url
        bigint file_size
        timestamptz uploaded_at
    }
    
    REACTIONS {
        uuid id PK
        uuid message_id FK
        uuid user_id FK
        varchar emoji
        timestamptz created_at
    }
    
    THREAD_MESSAGES {
        uuid parent_id FK
        uuid reply_id FK
        timestamptz created_at
    }
    
    TYPING_INDICATORS {
        uuid user_id FK
        uuid room_id FK
        timestamptz started_at
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
    display_name VARCHAR(100) NOT NULL,
    avatar_url VARCHAR(500),
    status VARCHAR(20) DEFAULT 'offline',
    status_message VARCHAR(200),
    last_seen TIMESTAMPTZ,
    is_online BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_is_online ON users(is_online) WHERE is_online = TRUE;

-- Função para atualizar updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

**Migration 002: Create Rooms Table**

```sql
-- migrations/002_create_rooms.sql
CREATE TABLE rooms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    type VARCHAR(20) NOT NULL DEFAULT 'channel',
    description TEXT,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    is_private BOOLEAN DEFAULT FALSE,
    max_members INTEGER DEFAULT 100,
    avatar_url VARCHAR(500),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT room_type_check CHECK (type IN ('channel', 'direct', 'group'))
);

CREATE INDEX idx_rooms_type ON rooms(type);
CREATE INDEX idx_rooms_is_private ON rooms(is_private);
CREATE INDEX idx_rooms_created_by ON rooms(created_by);

CREATE TRIGGER update_rooms_updated_at
    BEFORE UPDATE ON rooms
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

**Migration 003: Create User_Rooms (Membership)**

```sql
-- migrations/003_create_user_rooms.sql
CREATE TABLE user_rooms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    room_id UUID NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    role VARCHAR(20) DEFAULT 'member',
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_read_at TIMESTAMPTZ,
    muted BOOLEAN DEFAULT FALSE,
    
    CONSTRAINT role_check CHECK (role IN ('admin', 'moderator', 'member')),
    UNIQUE(user_id, room_id)
);

CREATE INDEX idx_user_rooms_user_id ON user_rooms(user_id);
CREATE INDEX idx_user_rooms_room_id ON user_rooms(room_id);
CREATE INDEX idx_user_rooms_role ON user_rooms(role);
```

**Migration 004: Create Messages Table**

```sql
-- migrations/004_create_messages.sql
CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    room_id UUID NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE SET NULL,
    reply_to_id UUID REFERENCES messages(id) ON DELETE SET NULL,
    content TEXT NOT NULL,
    message_type VARCHAR(20) DEFAULT 'text',
    is_edited BOOLEAN DEFAULT FALSE,
    is_deleted BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT message_type_check CHECK (message_type IN ('text', 'image', 'file', 'system'))
);

CREATE INDEX idx_messages_room_id ON messages(room_id);
CREATE INDEX idx_messages_user_id ON messages(user_id);
CREATE INDEX idx_messages_created_at ON messages(created_at DESC);
CREATE INDEX idx_messages_reply_to_id ON messages(reply_to_id) WHERE reply_to_id IS NOT NULL;

-- Full-text search
CREATE INDEX idx_messages_content_fts ON messages USING gin(to_tsvector('english', content));

CREATE TRIGGER update_messages_updated_at
    BEFORE UPDATE ON messages
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

**Migration 005: Create Message Attachments**

```sql
-- migrations/005_create_message_attachments.sql
CREATE TABLE message_attachments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    message_id UUID NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    file_name VARCHAR(255) NOT NULL,
    file_type VARCHAR(100) NOT NULL,
    file_url VARCHAR(500) NOT NULL,
    file_size BIGINT NOT NULL,
    thumbnail_url VARCHAR(500),
    uploaded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_attachments_message_id ON message_attachments(message_id);
```

**Migration 006: Create Reactions**

```sql
-- migrations/006_create_reactions.sql
CREATE TABLE reactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    message_id UUID NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    emoji VARCHAR(10) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(message_id, user_id, emoji)
);

CREATE INDEX idx_reactions_message_id ON reactions(message_id);
CREATE INDEX idx_reactions_user_id ON reactions(user_id);
```

**Migration 007: Create Thread Messages**

```sql
-- migrations/007_create_thread_messages.sql
CREATE TABLE thread_messages (
    parent_id UUID NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    reply_id UUID NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    PRIMARY KEY (parent_id, reply_id)
);

CREATE INDEX idx_thread_messages_parent_id ON thread_messages(parent_id);
CREATE INDEX idx_thread_messages_reply_id ON thread_messages(reply_id);
```

**Migration 008: Create Typing Indicators (ephemeral)**

```sql
-- migrations/008_create_typing_indicators.sql
-- Nota: Typing indicators podem ser apenas no Redis
-- Tabela opcional para histórico/debugging

CREATE TABLE typing_indicators (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    room_id UUID NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    PRIMARY KEY (user_id, room_id)
);

CREATE INDEX idx_typing_room_id ON typing_indicators(room_id);

-- Auto-cleanup de typing indicators antigos
CREATE OR REPLACE FUNCTION cleanup_old_typing_indicators()
RETURNS void AS $$
BEGIN
    DELETE FROM typing_indicators 
    WHERE started_at < NOW() - INTERVAL '10 seconds';
END;
$$ LANGUAGE plpgsql;
```

**Migration 009: Create Direct Message Helpers**

```sql
-- migrations/009_create_dm_helpers.sql
-- View para facilitar queries de DMs
CREATE VIEW direct_messages AS
SELECT 
    r.id as room_id,
    r.created_at,
    ur1.user_id as user1_id,
    ur2.user_id as user2_id,
    u1.username as user1_username,
    u2.username as user2_username
FROM rooms r
JOIN user_rooms ur1 ON r.id = ur1.room_id
JOIN user_rooms ur2 ON r.id = ur2.room_id AND ur1.user_id < ur2.user_id
JOIN users u1 ON ur1.user_id = u1.id
JOIN users u2 ON ur2.user_id = u2.id
WHERE r.type = 'direct';

-- Função para criar DM entre dois usuários
CREATE OR REPLACE FUNCTION create_or_get_dm_room(
    user1_id UUID,
    user2_id UUID
) RETURNS UUID AS $$
DECLARE
    room_id UUID;
BEGIN
    -- Verificar se já existe DM entre os usuários
    SELECT r.id INTO room_id
    FROM rooms r
    JOIN user_rooms ur1 ON r.id = ur1.room_id
    JOIN user_rooms ur2 ON r.id = ur2.room_id
    WHERE r.type = 'direct'
        AND ur1.user_id = user1_id
        AND ur2.user_id = user2_id;
    
    -- Se não existe, criar
    IF room_id IS NULL THEN
        INSERT INTO rooms (name, type, is_private, max_members)
        VALUES ('DM', 'direct', TRUE, 2)
        RETURNING id INTO room_id;
        
        INSERT INTO user_rooms (user_id, room_id, role)
        VALUES 
            (user1_id, room_id, 'member'),
            (user2_id, room_id, 'member');
    END IF;
    
    RETURN room_id;
END;
$$ LANGUAGE plpgsql;
```

**Migration 010: Create Notifications**

```sql
-- migrations/010_create_notifications.sql
CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL,
    title VARCHAR(200) NOT NULL,
    message TEXT NOT NULL,
    data JSONB,
    is_read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    read_at TIMESTAMPTZ
);

CREATE INDEX idx_notifications_user_id ON notifications(user_id);
CREATE INDEX idx_notifications_is_read ON notifications(is_read) WHERE is_read = FALSE;
CREATE INDEX idx_notifications_created_at ON notifications(created_at DESC);
```

---

## 📁 ESTRUTURA DE CÓDIGO

```
chat-server/
├── Cargo.toml
├── .env.example
├── .gitignore
├── docker-compose.yml
├── Dockerfile
├── README.md
│
├── migrations/
│   ├── 001_create_users.sql
│   ├── 002_create_rooms.sql
│   ├── 003_create_user_rooms.sql
│   ├── 004_create_messages.sql
│   ├── 005_create_message_attachments.sql
│   ├── 006_create_reactions.sql
│   ├── 007_create_thread_messages.sql
│   ├── 008_create_typing_indicators.sql
│   ├── 009_create_dm_helpers.sql
│   └── 010_create_notifications.sql
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
│   │   ├── room.rs
│   │   ├── message.rs
│   │   ├── reaction.rs
│   │   └── notification.rs
│   │
│   ├── dto/
│   │   ├── mod.rs
│   │   ├── auth_dto.rs
│   │   ├── message_dto.rs
│   │   ├── room_dto.rs
│   │   └── websocket_dto.rs
│   │
│   ├── repositories/
│   │   ├── mod.rs
│   │   ├── user_repository.rs
│   │   ├── room_repository.rs
│   │   ├── message_repository.rs
│   │   └── notification_repository.rs
│   │
│   ├── services/
│   │   ├── mod.rs
│   │   ├── auth_service.rs
│   │   ├── chat_service.rs
│   │   ├── room_service.rs
│   │   ├── presence_service.rs
│   │   └── notification_service.rs
│   │
│   ├── websocket/
│   │   ├── mod.rs
│   │   ├── connection.rs
│   │   ├── handler.rs
│   │   ├── session.rs
│   │   └── events.rs
│   │
│   ├── pubsub/
│   │   ├── mod.rs
│   │   ├── redis_pubsub.rs
│   │   └── message_broker.rs
│   │
│   ├── api/
│   │   ├── mod.rs
│   │   ├── routes.rs
│   │   ├── handlers/
│   │   │   ├── mod.rs
│   │   │   ├── auth_handlers.rs
│   │   │   ├── room_handlers.rs
│   │   │   ├── message_handlers.rs
│   │   │   └── user_handlers.rs
│   │   └── middleware/
│   │       ├── mod.rs
│   │       ├── auth_middleware.rs
│   │       └── rate_limit_middleware.rs
│   │
│   ├── storage/
│   │   ├── mod.rs
│   │   └── file_storage.rs
│   │
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── jwt.rs
│   │   └── validators.rs
│   │
│   ├── error.rs
│   ├── db.rs
│   └── state.rs
│
└── tests/
    ├── common/
    │   └── mod.rs
    ├── integration/
    │   ├── auth_tests.rs
    │   ├── chat_tests.rs
    │   └── websocket_tests.rs
    └── fixtures/
        └── test_data.rs
```

---

## 📦 DEPENDÊNCIAS (Cargo.toml)

```toml
[package]
name = "chat-server"
version = "0.1.0"
edition = "2021"

[dependencies]
# Web Framework
axum = { version = "0.7", features = ["ws", "macros"] }
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace", "fs"] }

# WebSocket
tokio-tungstenite = "0.21"
futures = "0.3"

# Database
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "migrate"] }

# Redis (Cache + Pub/Sub)
redis = { version = "0.24", features = ["tokio-comp", "connection-manager", "streams"] }

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

# File Upload
multer = "3.0"
mime = "0.3"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# Config
dotenvy = "0.15"

# Concurrency
dashmap = "5.5"
tokio-util = "0.7"

[dev-dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio-tungstenite = "0.21"
```

---

## 🔌 API ENDPOINTS

### **REST API (HTTP)**

#### **Authentication**

```
POST   /api/v1/auth/register            # Register new user
POST   /api/v1/auth/login               # Login
POST   /api/v1/auth/refresh             # Refresh token
GET    /api/v1/auth/me                  # Get current user
```

#### **Rooms/Channels**

```
GET    /api/v1/rooms                    # List all rooms
POST   /api/v1/rooms                    # Create room
GET    /api/v1/rooms/:id                # Get room details
PUT    /api/v1/rooms/:id                # Update room
DELETE /api/v1/rooms/:id                # Delete room
POST   /api/v1/rooms/:id/join           # Join room
POST   /api/v1/rooms/:id/leave          # Leave room
GET    /api/v1/rooms/:id/members        # List room members
```

#### **Direct Messages**

```
GET    /api/v1/dms                      # List DM conversations
POST   /api/v1/dms/:user_id             # Start DM with user
```

#### **Messages (HTTP for history)**

```
GET    /api/v1/rooms/:id/messages       # Get message history
GET    /api/v1/messages/:id             # Get single message
PUT    /api/v1/messages/:id             # Edit message
DELETE /api/v1/messages/:id             # Delete message
POST   /api/v1/messages/:id/reactions   # Add reaction
```

#### **File Upload**

```
POST   /api/v1/upload                   # Upload file/image
```

#### **Users**

```
GET    /api/v1/users                    # Search users
GET    /api/v1/users/:id                # Get user profile
PUT    /api/v1/users/me                 # Update own profile
GET    /api/v1/users/online             # List online users
```

---

### **WebSocket API (Real-time)**

#### **Connection**

```
WS     /ws?token=<jwt_token>            # WebSocket endpoint
```

#### **Client → Server Events**

```json
// Join room
{
  "type": "join_room",
  "room_id": "uuid",
  "timestamp": "2025-11-15T10:00:00Z"
}

// Leave room
{
  "type": "leave_room",
  "room_id": "uuid"
}

// Send message
{
  "type": "send_message",
  "room_id": "uuid",
  "content": "Hello, world!",
  "reply_to_id": "uuid",  // optional
  "attachments": []        // optional
}

// Typing indicator
{
  "type": "typing",
  "room_id": "uuid",
  "is_typing": true
}

// Edit message
{
  "type": "edit_message",
  "message_id": "uuid",
  "new_content": "Updated message"
}

// Delete message
{
  "type": "delete_message",
  "message_id": "uuid"
}

// Add reaction
{
  "type": "add_reaction",
  "message_id": "uuid",
  "emoji": "👍"
}

// Remove reaction
{
  "type": "remove_reaction",
  "message_id": "uuid",
  "emoji": "👍"
}

// Ping (keep-alive)
{
  "type": "ping"
}
```

#### **Server → Client Events**

```json
// New message
{
  "type": "new_message",
  "message": {
    "id": "uuid",
    "room_id": "uuid",
    "user": {
      "id": "uuid",
      "username": "bianeck",
      "display_name": "Thiago Bianeck",
      "avatar_url": "https://..."
    },
    "content": "Hello!",
    "created_at": "2025-11-15T10:00:00Z"
  }
}

// Message deleted
{
  "type": "message_deleted",
  "message_id": "uuid",
  "room_id": "uuid"
}

// Message edited
{
  "type": "message_edited",
  "message_id": "uuid",
  "new_content": "Updated",
  "edited_at": "2025-11-15T10:01:00Z"
}

// User typing
{
  "type": "user_typing",
  "room_id": "uuid",
  "user": {
    "id": "uuid",
    "username": "bianeck"
  },
  "is_typing": true
}

// User joined room
{
  "type": "user_joined",
  "room_id": "uuid",
  "user": {...}
}

// User left room
{
  "type": "user_left",
  "room_id": "uuid",
  "user_id": "uuid"
}

// User online status
{
  "type": "user_status",
  "user_id": "uuid",
  "status": "online",
  "last_seen": "2025-11-15T10:00:00Z"
}

// Reaction added
{
  "type": "reaction_added",
  "message_id": "uuid",
  "user_id": "uuid",
  "emoji": "👍"
}

// Pong (response to ping)
{
  "type": "pong"
}

// Error
{
  "type": "error",
  "code": "UNAUTHORIZED",
  "message": "Invalid token"
}
```

---

### **Exemplos de Uso**

#### **1. Autenticação e Conexão WebSocket**

```javascript
// 1. Login via REST
const loginResponse = await fetch('http://localhost:8000/api/v1/auth/login', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    email: 'bianeck@example.com',
    password: 'SecurePass123'
  })
});

const { access_token } = await loginResponse.json();

// 2. Conectar WebSocket
const ws = new WebSocket(`ws://localhost:8000/ws?token=${access_token}`);

ws.onopen = () => {
  console.log('Connected!');
  
  // 3. Join room
  ws.send(JSON.stringify({
    type: 'join_room',
    room_id: 'general-room-uuid'
  }));
};

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  console.log('Received:', message);
};
```

#### **2. Enviar Mensagem**

```javascript
ws.send(JSON.stringify({
  type: 'send_message',
  room_id: 'general-room-uuid',
  content: 'Hello from Rust Chat!'
}));
```

#### **3. Typing Indicator**

```javascript
// User starts typing
input.addEventListener('input', () => {
  ws.send(JSON.stringify({
    type: 'typing',
    room_id: currentRoomId,
    is_typing: true
  }));
  
  clearTimeout(typingTimeout);
  typingTimeout = setTimeout(() => {
    ws.send(JSON.stringify({
      type: 'typing',
      room_id: currentRoomId,
      is_typing: false
    }));
  }, 2000);
});
```

---

## 🔄 REDIS PUB/SUB

### **Channels Pattern**

```
room:<room_id>              -> Mensagens de uma sala
user:<user_id>              -> Notificações para usuário
presence:online             -> Lista de usuários online
typing:<room_id>            -> Typing indicators
```

### **Implementação Pub/Sub**

```rust
pub struct RedisPubSub {
    client: redis::Client,
}

impl RedisPubSub {
    pub async fn publish_message(&self, room_id: &Uuid, message: &Message) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        let channel = format!("room:{}", room_id);
        let payload = serde_json::to_string(message)?;
        
        conn.publish(channel, payload).await?;
        Ok(())
    }
    
    pub async fn subscribe_to_room(&self, room_id: &Uuid) -> Result<PubSubStream> {
        let mut conn = self.client.get_async_connection().await?;
        let channel = format!("room:{}", room_id);
        
        let mut pubsub = conn.into_pubsub();
        pubsub.subscribe(channel).await?;
        
        Ok(pubsub.into_on_message())
    }
}
```

---

## 🌐 WEBSOCKET HANDLER

### **Connection Manager**

```rust
use dashmap::DashMap;
use tokio::sync::mpsc;

pub struct ConnectionManager {
    // user_id -> Vec<WebSocket connections>
    connections: Arc<DashMap<Uuid, Vec<mpsc::UnboundedSender<WsMessage>>>>,
    // room_id -> Set<user_id>
    room_members: Arc<DashMap<Uuid, HashSet<Uuid>>>,
}

impl ConnectionManager {
    pub fn add_connection(&self, user_id: Uuid, sender: mpsc::UnboundedSender<WsMessage>) {
        self.connections
            .entry(user_id)
            .or_insert_with(Vec::new)
            .push(sender);
    }
    
    pub fn remove_connection(&self, user_id: &Uuid) {
        self.connections.remove(user_id);
    }
    
    pub async fn broadcast_to_room(&self, room_id: &Uuid, message: WsMessage) {
        if let Some(members) = self.room_members.get(room_id) {
            for user_id in members.iter() {
                self.send_to_user(user_id, message.clone()).await;
            }
        }
    }
    
    pub async fn send_to_user(&self, user_id: &Uuid, message: WsMessage) {
        if let Some(connections) = self.connections.get(user_id) {
            for sender in connections.iter() {
                let _ = sender.send(message.clone());
            }
        }
    }
}
```

### **WebSocket Handler Flow**

```mermaid
flowchart TD
    A[New WS Connection] --> B{Valid JWT Token?}
    
    D --> E[Add to ConnectionManager]
    E --> F[Set User Online]
    
    F --> G{Receive Message}
    
    
    H --> L[Add to room_members]
    L --> M[Broadcast user_joined]
    M --> G
    
    I --> N[Save to DB]
    N --> O[Publish to Redis]
    O --> P[Broadcast to Room]
    P --> G
    
    J --> Q[Publish to Redis]
    Q --> R[Broadcast typing indicator]
    R --> G
    
    K --> S[Send pong]
    S --> G
    
    T --> U[Remove from ConnectionManager]
    U --> V[Set User Offline]
    V --> W[Broadcast user_left]
    W --> X[Close Connection]
    
    style B fill:#f9f
    style I fill:#4ecdc4
    style T fill:#ff6b6b
```

---

## 📊 PRESENCE SYSTEM

### **Online Status Management**

```rust
pub struct PresenceService {
    redis: RedisClient,
    db: PgPool,
}

impl PresenceService {
    pub async fn set_online(&self, user_id: Uuid) -> Result<()> {
        // Redis: adicionar a set de usuários online
        let key = "presence:online";
        self.redis.sadd(key, user_id.to_string()).await?;
        
        // DB: atualizar status
        sqlx::query!(
            "UPDATE users SET is_online = true, last_seen = NOW() WHERE id = $1",
            user_id
        )
        .execute(&self.db)
        .await?;
        
        // Broadcast status change
        self.broadcast_status_change(user_id, "online").await?;
        
        Ok(())
    }
    
    pub async fn set_offline(&self, user_id: Uuid) -> Result<()> {
        let key = "presence:online";
        self.redis.srem(key, user_id.to_string()).await?;
        
        sqlx::query!(
            "UPDATE users SET is_online = false, last_seen = NOW() WHERE id = $1",
            user_id
        )
        .execute(&self.db)
        .await?;
        
        self.broadcast_status_change(user_id, "offline").await?;
        
        Ok(())
    }
    
    pub async fn get_online_users(&self) -> Result<Vec<Uuid>> {
        let key = "presence:online";
        let members: Vec<String> = self.redis.smembers(key).await?;
        
        Ok(members.into_iter()
            .collect())
    }
}
```

---

## 💬 MESSAGE FLOW

### **Envio de Mensagem (Detalhado)**

```mermaid
sequenceDiagram
    participant Client
    participant WSHandler
    participant MessageService
    participant DB
    participant Redis
    participant PubSub
    participant OtherClients
    
    Client->>WSHandler: send_message event
    
    WSHandler->>MessageService: save_message()
    MessageService->>DB: INSERT INTO messages
    DB-->>MessageService: message_id
    
    MessageService->>Redis: Cache message
    MessageService-->>WSHandler: Message saved
    
    WSHandler->>PubSub: PUBLISH room:uuid
    
    PubSub-->>WSHandler: (self receive)
    PubSub-->>OtherClients: (broadcast)
    
    WSHandler->>Client: ACK + message
    OtherClients->>OtherClients: Display message
    
    Note over DB,Redis: < 50ms total latency
```

---

## 📤 FILE UPLOAD

### **Upload Flow**

```rust
pub async fn upload_file(
    mut multipart: Multipart,
    user_id: Uuid,
) -> Result<UploadResponse> {
    while let Some(field) = multipart.next_field().await? {
        let name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await?;
        
        // Validar tipo de arquivo
        validate_file_type(&content_type)?;
        
        // Validar tamanho (max 10MB)
        if data.len() > 10 * 1024 * 1024 {
            return Err(AppError::FileTooLarge);
        }
        
        // Gerar nome único
        let file_id = Uuid::new_v4();
        let extension = Path::new(&name).extension().unwrap().to_str().unwrap();
        let file_name = format!("{}.{}", file_id, extension);
        
        // Upload para storage (S3 ou local)
        let url = storage::upload(&file_name, &data, &content_type).await?;
        
        // Gerar thumbnail se for imagem
        let thumbnail_url = if content_type.starts_with("image/") {
            Some(generate_thumbnail(&data, &file_id).await?)
        } else {
            None
        };
        
        return Ok(UploadResponse {
            file_id,
            file_name: name,
            file_url: url,
            thumbnail_url,
            file_size: data.len() as i64,
        });
    }
    
    Err(AppError::NoFileProvided)
}
```

---

## 🐳 DOCKER

### **docker-compose.yml**

```yaml
version: '3.8'

services:
  postgres:
    image: postgres:16-alpine
    container_name: chat-db
    environment:
      POSTGRES_USER: chatserver
      POSTGRES_PASSWORD: chatserver_dev
      POSTGRES_DB: chatserver
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U chatserver"]
      interval: 5s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    container_name: chat-redis
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
    command: redis-server --appendonly yes
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 5s
      timeout: 3s
      retries: 5

  api:
    build: .
    container_name: chat-api
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy
    environment:
      DATABASE_URL: postgres://chatserver:chatserver_dev@postgres/chatserver
      REDIS_URL: redis://redis:6379
      JWT_SECRET: your-secret-key-change-in-production
      RUST_LOG: info,chat_server=debug
      FILE_STORAGE_PATH: /app/uploads
    ports:
      - "8000:8000"
    volumes:
      - ./:/app
      - uploads:/app/uploads

volumes:
  postgres_data:
  redis_data:
  uploads:
```

---

## 📊 FEATURES POR DIA

### **Dia 53: Setup e Auth**
- ✅ Projeto setup + dependencies
- ✅ Docker Compose (PostgreSQL + Redis)
- ✅ Migrations (users, rooms básicas)
- ✅ Auth service (JWT)
- ✅ REST API básica

### **Dia 54: WebSocket Foundation**
- ✅ WebSocket endpoint
- ✅ Connection manager
- ✅ Session handling
- ✅ Ping/Pong keep-alive
- ✅ Auth via JWT token

### **Dia 55: Rooms e Membership**
- ✅ Room CRUD
- ✅ Join/Leave rooms
- ✅ User-rooms relationship
- ✅ Direct messages
- ✅ Room types (channel/DM/group)

### **Dia 56: Mensagens Real-Time**
- ✅ Send message via WebSocket
- ✅ Message persistence
- ✅ Redis Pub/Sub
- ✅ Broadcast to room
- ✅ Message history (REST)

### **Dia 57: Features Avançadas**
- ✅ Typing indicators
- ✅ Online/offline presence
- ✅ Message reactions
- ✅ Edit/Delete messages
- ✅ Threads (replies)

### **Dia 58: File Upload**
- ✅ Multipart file upload
- ✅ Image thumbnails
- ✅ File storage (local/S3)
- ✅ Attachments em mensagens
- ✅ Validação de tipos

### **Dia 59: Testes e Docs**
- ✅ Unit tests
- ✅ WebSocket integration tests
- ✅ Load testing
- ✅ API documentation
- ✅ WebSocket protocol docs

### **Dia 60: Polish e Deploy**
- ✅ Notifications system
- ✅ Performance tuning
- ✅ Reconnection logic
- ✅ Rate limiting
- ✅ Production deployment

---

## 🚀 QUICK START

```bash
# 1. Clone e setup
git clone <repo>
cd chat-server
cp .env.example .env

# 2. Start services
docker-compose up -d

# 3. Run migrations
sqlx migrate run

# 4. Run server
cargo run

# 5. Test REST API
curl http://localhost:8000/api/v1/health

# 6. Test WebSocket (usando wscat)
npm install -g wscat

# Login first
curl -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"password"}'

# Connect WebSocket
wscat -c "ws://localhost:8000/ws?token=<YOUR_JWT_TOKEN>"

# Send message
{"type":"join_room","room_id":"<ROOM_UUID>"}
{"type":"send_message","room_id":"<ROOM_UUID>","content":"Hello!"}
```

---

## 📈 PERFORMANCE TARGETS

### **Latência**
- WebSocket message delivery: < 50ms (P99)
- Message persistence: < 20ms
- Redis Pub/Sub propagation: < 10ms
- File upload: < 500ms (para 1MB)

### **Throughput**
- Concurrent WebSocket connections: 10,000 per instance
- Messages per second: 5,000 per instance
- Online user updates: 1,000/s

### **Escalabilidade**
- Horizontal scaling via Redis Pub/Sub
- Stateless server instances
- Database connection pooling

---

## 🛡️ SEGURANÇA

```rust
// Rate limiting por usuário
pub struct ChatRateLimiter {
    // user_id -> (count, window_start)
    limits: DashMap<Uuid, (u32, Instant)>,
}

impl ChatRateLimiter {
    pub fn check_limit(&self, user_id: &Uuid) -> Result<(), RateLimitError> {
        const MAX_MESSAGES: u32 = 10;
        const WINDOW: Duration = Duration::from_secs(10);
        
        let now = Instant::now();
        
        let mut entry = self.limits.entry(*user_id).or_insert((0, now));
        
        if now.duration_since(entry.1) > WINDOW {
            *entry = (1, now);
            Ok(())
        } else if entry.0 < MAX_MESSAGES {
            entry.0 += 1;
            Ok(())
        } else {
            Err(RateLimitError::TooManyMessages)
        }
    }
}
```

---

## 📚 RECURSOS ADICIONAIS

**Crates Importantes:**
- [axum](https://docs.rs/axum/) - Web framework com WS
- [tokio-tungstenite](https://docs.rs/tokio-tungstenite/) - WebSocket
- [redis](https://docs.rs/redis/) - Redis client
- [dashmap](https://docs.rs/dashmap/) - Concurrent HashMap
- [multer](https://docs.rs/multer/) - Multipart form-data

**Frontend Examples:**
- JavaScript/TypeScript WebSocket client
- React chat UI components
- Vue.js chat application

**Inspiração:**
- [Slack](https://slack.com/)
- [Discord](https://discord.com/)
- [Rocket.Chat](https://rocket.chat/)

---

**Sistema completo de chat em tempo real! WebSockets + Redis Pub/Sub + Concorrência = Chat escalável e performático! 💬🚀**
