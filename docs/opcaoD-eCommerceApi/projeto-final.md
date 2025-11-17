# 📋 PROJETO FINAL - OPÇÃO D: E-COMMERCE API

**Duração:** 8 dias (53-60)  
**Nível:** Projeto completo aplicando todos os conceitos aprendidos  
**Stack:** Rust + Axum + SQLx + PostgreSQL + Redis + Stripe

---

## 🎯 VISÃO GERAL

API REST completa para plataforma de e-commerce com multi-vendor, gestão de produtos, carrinho de compras, checkout, pagamentos, sistema de avaliações, cupons de desconto e analytics.

**Funcionalidades Principais:**
- ✅ Catálogo de produtos com busca e filtros
- ✅ Carrinho de compras persistente
- ✅ Checkout com cálculo de frete
- ✅ Integração com gateway de pagamento (Stripe)
- ✅ Gestão de pedidos e status
- ✅ Controle de inventário (estoque)
- ✅ Sistema de avaliações e comentários
- ✅ Cupons e descontos
- ✅ Multi-vendor (vendedores)
- ✅ Analytics e relatórios

---

## 🏗️ ARQUITETURA

### **Diagrama de Componentes**

```mermaid
graph TB
    Client[Client App/Web]
    
    subgraph "API Layer"
        API[E-commerce API]
        Auth[Auth Service]
    end
    
    subgraph "Core Services"
        Product[Product Service]
        Cart[Cart Service]
        Order[Order Service]
        Payment[Payment Service]
        Inventory[Inventory Service]
    end
    
    subgraph "Data Layer"
        DB[(PostgreSQL)]
        Cache[(Redis Cache)]
        Queue[Job Queue]
    end
    
    subgraph "External Services"
        Stripe[Stripe API]
        Shipping[Shipping API]
        Email[Email Service]
    end
    
    Client --> API
    API --> Auth
    API --> Product
    API --> Cart
    API --> Order
    
    Cart --> Cache
    Product --> Cache
    
    Order --> Payment
    Order --> Inventory
    Payment --> Stripe
    Order --> Shipping
    Order --> Email
    
    Product --> DB
    Cart --> DB
    Order --> DB
    Inventory --> DB
    
    Order --> Queue
    Queue --> Email
    
    style API fill:#4ecdc4,stroke:#333,stroke-width:2px
    style DB fill:#95e1d3,stroke:#333,stroke-width:2px
    style Cache fill:#ff6b6b,stroke:#333,stroke-width:2px
```

### **Fluxo de Compra (Checkout)**

```mermaid
sequenceDiagram
    participant Customer
    participant API
    participant Cart
    participant Inventory
    participant Payment
    participant Stripe
    participant Order
    participant Email
    
    Customer->>API: Add items to cart
    API->>Cart: Save cart items
    Cart->>API: Cart saved
    
    Customer->>API: Proceed to checkout
    API->>Cart: Get cart items
    Cart-->>API: Cart data
    
    API->>Inventory: Check stock availability
    Inventory-->>API: Stock confirmed
    
    API->>API: Calculate totals + shipping
    API-->>Customer: Checkout summary
    
    Customer->>API: Confirm payment
    API->>Inventory: Reserve stock (lock)
    
    API->>Payment: Process payment
    Payment->>Stripe: Charge customer
    Stripe-->>Payment: Payment successful
    
    Payment-->>API: Payment confirmed
    API->>Order: Create order
    Order->>Inventory: Deduct stock
    Order->>Email: Send confirmation
    
    API-->>Customer: Order created
    
    Note over Inventory: Transactional<br/>stock management
```

### **Estados de Pedido**

```mermaid
stateDiagram-v2
    [*] --> Pending: Create Order
    
    Pending --> PaymentProcessing: Start Payment
    PaymentProcessing --> Paid: Payment Success
    PaymentProcessing --> Failed: Payment Failed
    
    Paid --> Processing: Prepare Order
    Processing --> Shipped: Ship Order
    Shipped --> Delivered: Delivery Confirmed
    
    Pending --> Cancelled: Customer Cancels
    PaymentProcessing --> Cancelled: Timeout
    Paid --> Cancelled: Cancel Request
    Processing --> Cancelled: Cancel Request
    
    Failed --> [*]
    Cancelled --> [*]
    Delivered --> [*]
    
    Delivered --> Returned: Return Request
    Returned --> Refunded: Refund Processed
    Refunded --> [*]
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
        varchar full_name
        varchar phone
        jsonb addresses
        varchar role
        timestamptz created_at
    }
    
    VENDORS {
        uuid id PK
        uuid user_id FK
        varchar store_name UK
        text description
        varchar logo_url
        decimal commission_rate
        boolean is_active
        timestamptz created_at
    }
    
    CATEGORIES {
        uuid id PK
        varchar name
        varchar slug UK
        uuid parent_id FK
        integer order
    }
    
    PRODUCTS {
        uuid id PK
        uuid vendor_id FK
        uuid category_id FK
        varchar name
        varchar slug UK
        text description
        decimal price
        decimal compare_at_price
        varchar sku UK
        integer stock_quantity
        boolean is_active
        jsonb metadata
        timestamptz created_at
    }
    
    PRODUCT_VARIANTS {
        uuid id PK
        uuid product_id FK
        varchar name
        varchar sku UK
        decimal price_modifier
        integer stock_quantity
        jsonb attributes
    }
    
    PRODUCT_IMAGES {
        uuid id PK
        uuid product_id FK
        varchar url
        integer order
        boolean is_primary
    }
    
    CART_ITEMS {
        uuid id PK
        uuid user_id FK
        uuid product_id FK
        uuid variant_id FK
        integer quantity
        timestamptz created_at
        timestamptz updated_at
    }
    
    ORDERS {
        uuid id PK
        varchar order_number UK
        uuid user_id FK
        uuid coupon_id FK
        decimal subtotal
        decimal discount
        decimal shipping_cost
        decimal tax
        decimal total
        varchar status
        jsonb shipping_address
        jsonb billing_address
        timestamptz created_at
    }
    
    ORDER_ITEMS {
        uuid id PK
        uuid order_id FK
        uuid product_id FK
        uuid variant_id FK
        varchar product_name
        decimal unit_price
        integer quantity
        decimal subtotal
    }
    
    PAYMENTS {
        uuid id PK
        uuid order_id FK
        decimal amount
        varchar currency
        varchar payment_method
        varchar stripe_payment_id
        varchar status
        timestamptz paid_at
    }
    
    SHIPPING_INFO {
        uuid id PK
        uuid order_id FK
        varchar carrier
        varchar tracking_number
        timestamptz shipped_at
        timestamptz delivered_at
    }
    
    REVIEWS {
        uuid id PK
        uuid product_id FK
        uuid user_id FK
        uuid order_id FK
        integer rating
        text comment
        jsonb images
        boolean is_verified_purchase
        timestamptz created_at
    }
    
    COUPONS {
        uuid id PK
        varchar code UK
        varchar type
        decimal value
        decimal min_purchase
        integer usage_limit
        integer used_count
        timestamptz valid_from
        timestamptz valid_until
        boolean is_active
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
    full_name VARCHAR(255) NOT NULL,
    phone VARCHAR(20),
    addresses JSONB DEFAULT '[]',
    role VARCHAR(20) DEFAULT 'customer',
    is_active BOOLEAN DEFAULT TRUE,
    email_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT role_check CHECK (role IN ('customer', 'vendor', 'admin'))
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role);

-- Trigger para updated_at
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

**Migration 002: Create Vendors Table**

```sql
-- migrations/002_create_vendors.sql
CREATE TABLE vendors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID UNIQUE NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    store_name VARCHAR(255) UNIQUE NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    logo_url VARCHAR(500),
    banner_url VARCHAR(500),
    commission_rate DECIMAL(5,2) DEFAULT 15.00,
    is_active BOOLEAN DEFAULT TRUE,
    total_sales DECIMAL(12,2) DEFAULT 0,
    rating_average DECIMAL(3,2) DEFAULT 0,
    rating_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_vendors_user_id ON vendors(user_id);
CREATE INDEX idx_vendors_slug ON vendors(slug);
CREATE INDEX idx_vendors_is_active ON vendors(is_active) WHERE is_active = TRUE;

CREATE TRIGGER update_vendors_updated_at
    BEFORE UPDATE ON vendors
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

**Migration 003: Create Categories Table**

```sql
-- migrations/003_create_categories.sql
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    parent_id UUID REFERENCES categories(id) ON DELETE CASCADE,
    image_url VARCHAR(500),
    display_order INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_categories_slug ON categories(slug);
CREATE INDEX idx_categories_parent_id ON categories(parent_id);
CREATE INDEX idx_categories_is_active ON categories(is_active) WHERE is_active = TRUE;

-- Função para obter path completo da categoria
CREATE OR REPLACE FUNCTION get_category_path(category_id UUID)
RETURNS TEXT AS $$
DECLARE
    path TEXT := '';
    current_id UUID := category_id;
    current_name VARCHAR(100);
    parent UUID;
BEGIN
    LOOP
        SELECT name, parent_id INTO current_name, parent
        FROM categories WHERE id = current_id;
        
        EXIT WHEN current_id IS NULL;
        
        IF path = '' THEN
            path := current_name;
        ELSE
        END IF;
        
        current_id := parent;
    END LOOP;
    
    RETURN path;
END;
$$ LANGUAGE plpgsql;
```

**Migration 004: Create Products Table**

```sql
-- migrations/004_create_products.sql
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vendor_id UUID NOT NULL REFERENCES vendors(id) ON DELETE CASCADE,
    category_id UUID REFERENCES categories(id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    description TEXT,
    short_description VARCHAR(500),
    price DECIMAL(10,2) NOT NULL,
    compare_at_price DECIMAL(10,2),
    cost_price DECIMAL(10,2),
    sku VARCHAR(100) UNIQUE NOT NULL,
    barcode VARCHAR(100),
    stock_quantity INTEGER DEFAULT 0,
    low_stock_threshold INTEGER DEFAULT 10,
    weight DECIMAL(8,2),
    dimensions JSONB,
    is_active BOOLEAN DEFAULT TRUE,
    is_featured BOOLEAN DEFAULT FALSE,
    metadata JSONB DEFAULT '{}',
    seo_title VARCHAR(255),
    seo_description TEXT,
    views_count INTEGER DEFAULT 0,
    sales_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT price_positive CHECK (price >= 0),
    CONSTRAINT stock_non_negative CHECK (stock_quantity >= 0),
    UNIQUE(vendor_id, slug)
);

CREATE INDEX idx_products_vendor_id ON products(vendor_id);
CREATE INDEX idx_products_category_id ON products(category_id);
CREATE INDEX idx_products_slug ON products(slug);
CREATE INDEX idx_products_sku ON products(sku);
CREATE INDEX idx_products_is_active ON products(is_active) WHERE is_active = TRUE;
CREATE INDEX idx_products_price ON products(price);
CREATE INDEX idx_products_created_at ON products(created_at DESC);

-- Full-text search

CREATE TRIGGER update_products_updated_at
    BEFORE UPDATE ON products
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

**Migration 005: Create Product Variants**

```sql
-- migrations/005_create_product_variants.sql
CREATE TABLE product_variants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    sku VARCHAR(100) UNIQUE NOT NULL,
    barcode VARCHAR(100),
    price_modifier DECIMAL(10,2) DEFAULT 0,
    stock_quantity INTEGER DEFAULT 0,
    attributes JSONB NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT stock_non_negative CHECK (stock_quantity >= 0)
);

CREATE INDEX idx_variants_product_id ON product_variants(product_id);
CREATE INDEX idx_variants_sku ON product_variants(sku);
CREATE INDEX idx_variants_attributes ON product_variants USING gin(attributes);
```

**Migration 006: Create Product Images**

```sql
-- migrations/006_create_product_images.sql
CREATE TABLE product_images (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    url VARCHAR(500) NOT NULL,
    alt_text VARCHAR(255),
    display_order INTEGER DEFAULT 0,
    is_primary BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_images_product_id ON product_images(product_id);
CREATE INDEX idx_images_display_order ON product_images(display_order);

-- Garantir apenas uma imagem primária por produto
CREATE UNIQUE INDEX idx_one_primary_per_product 
    ON product_images(product_id) 
    WHERE is_primary = TRUE;
```

**Migration 007: Create Cart Items**

```sql
-- migrations/007_create_cart_items.sql
CREATE TABLE cart_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    variant_id UUID REFERENCES product_variants(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT quantity_positive CHECK (quantity > 0),
    UNIQUE(user_id, product_id, variant_id)
);

CREATE INDEX idx_cart_user_id ON cart_items(user_id);
CREATE INDEX idx_cart_product_id ON cart_items(product_id);

CREATE TRIGGER update_cart_items_updated_at
    BEFORE UPDATE ON cart_items
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

**Migration 008: Create Orders**

```sql
-- migrations/008_create_orders.sql
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_number VARCHAR(50) UNIQUE NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    coupon_id UUID REFERENCES coupons(id) ON DELETE SET NULL,
    
    subtotal DECIMAL(10,2) NOT NULL,
    discount DECIMAL(10,2) DEFAULT 0,
    shipping_cost DECIMAL(10,2) DEFAULT 0,
    tax DECIMAL(10,2) DEFAULT 0,
    total DECIMAL(10,2) NOT NULL,
    
    currency VARCHAR(3) DEFAULT 'USD',
    
    status VARCHAR(20) DEFAULT 'pending',
    
    shipping_address JSONB NOT NULL,
    billing_address JSONB NOT NULL,
    
    customer_notes TEXT,
    admin_notes TEXT,
    
    ip_address VARCHAR(45),
    user_agent TEXT,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT status_check CHECK (status IN (
        'pending', 'payment_processing', 'paid', 
        'processing', 'shipped', 'delivered', 
        'cancelled', 'failed', 'refunded', 'returned'
    ))
);

CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_orders_order_number ON orders(order_number);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_created_at ON orders(created_at DESC);

-- Função para gerar order_number único
CREATE OR REPLACE FUNCTION generate_order_number()
RETURNS TRIGGER AS $$
BEGIN
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE SEQUENCE order_number_seq START 1;

CREATE TRIGGER set_order_number
    BEFORE INSERT ON orders
    FOR EACH ROW
    EXECUTE FUNCTION generate_order_number();

CREATE TRIGGER update_orders_updated_at
    BEFORE UPDATE ON orders
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

**Migration 009: Create Order Items**

```sql
-- migrations/009_create_order_items.sql
CREATE TABLE order_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE RESTRICT,
    variant_id UUID REFERENCES product_variants(id) ON DELETE RESTRICT,
    vendor_id UUID NOT NULL REFERENCES vendors(id) ON DELETE RESTRICT,
    
    product_name VARCHAR(255) NOT NULL,
    variant_name VARCHAR(100),
    sku VARCHAR(100) NOT NULL,
    
    unit_price DECIMAL(10,2) NOT NULL,
    quantity INTEGER NOT NULL,
    subtotal DECIMAL(10,2) NOT NULL,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT quantity_positive CHECK (quantity > 0)
);

CREATE INDEX idx_order_items_order_id ON order_items(order_id);
CREATE INDEX idx_order_items_product_id ON order_items(product_id);
CREATE INDEX idx_order_items_vendor_id ON order_items(vendor_id);
```

**Migration 010: Create Payments**

```sql
-- migrations/010_create_payments.sql
CREATE TABLE payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID UNIQUE NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    
    amount DECIMAL(10,2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'USD',
    
    payment_method VARCHAR(50) NOT NULL,
    payment_provider VARCHAR(50) DEFAULT 'stripe',
    
    stripe_payment_intent_id VARCHAR(255),
    stripe_charge_id VARCHAR(255),
    
    status VARCHAR(20) DEFAULT 'pending',
    
    metadata JSONB DEFAULT '{}',
    
    paid_at TIMESTAMPTZ,
    refunded_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT status_check CHECK (status IN (
        'pending', 'processing', 'succeeded', 
        'failed', 'cancelled', 'refunded'
    ))
);

CREATE INDEX idx_payments_order_id ON payments(order_id);
CREATE INDEX idx_payments_stripe_payment_intent_id ON payments(stripe_payment_intent_id);
CREATE INDEX idx_payments_status ON payments(status);

CREATE TRIGGER update_payments_updated_at
    BEFORE UPDATE ON payments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

**Migration 011: Create Shipping Info**

```sql
-- migrations/011_create_shipping_info.sql
CREATE TABLE shipping_info (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID UNIQUE NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    
    carrier VARCHAR(100),
    service_name VARCHAR(100),
    tracking_number VARCHAR(255),
    tracking_url VARCHAR(500),
    
    estimated_delivery TIMESTAMPTZ,
    
    shipped_at TIMESTAMPTZ,
    in_transit_at TIMESTAMPTZ,
    out_for_delivery_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    
    delivery_signature TEXT,
    delivery_notes TEXT,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_shipping_order_id ON shipping_info(order_id);
CREATE INDEX idx_shipping_tracking_number ON shipping_info(tracking_number);

CREATE TRIGGER update_shipping_info_updated_at
    BEFORE UPDATE ON shipping_info
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

**Migration 012: Create Reviews**

```sql
-- migrations/012_create_reviews.sql
CREATE TABLE reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    order_id UUID REFERENCES orders(id) ON DELETE SET NULL,
    
    rating INTEGER NOT NULL,
    title VARCHAR(255),
    comment TEXT,
    
    images JSONB DEFAULT '[]',
    
    is_verified_purchase BOOLEAN DEFAULT FALSE,
    is_approved BOOLEAN DEFAULT TRUE,
    
    helpful_count INTEGER DEFAULT 0,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT rating_check CHECK (rating >= 1 AND rating <= 5),
    UNIQUE(product_id, user_id, order_id)
);

CREATE INDEX idx_reviews_product_id ON reviews(product_id);
CREATE INDEX idx_reviews_user_id ON reviews(user_id);
CREATE INDEX idx_reviews_rating ON reviews(rating);
CREATE INDEX idx_reviews_is_approved ON reviews(is_approved) WHERE is_approved = TRUE;

-- Trigger para atualizar rating médio do produto
CREATE OR REPLACE FUNCTION update_product_rating()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE products p
    SET 
        rating_average = (
            SELECT COALESCE(AVG(rating), 0)
            FROM reviews
            WHERE product_id = p.id AND is_approved = TRUE
        ),
        rating_count = (
            SELECT COUNT(*)
            FROM reviews
            WHERE product_id = p.id AND is_approved = TRUE
        )
    WHERE p.id = COALESCE(NEW.product_id, OLD.product_id);
    
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER after_review_change
    AFTER INSERT OR UPDATE OR DELETE ON reviews
    FOR EACH ROW
    EXECUTE FUNCTION update_product_rating();
```

**Migration 013: Create Coupons**

```sql
-- migrations/013_create_coupons.sql
CREATE TABLE coupons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    
    type VARCHAR(20) NOT NULL,
    value DECIMAL(10,2) NOT NULL,
    
    min_purchase DECIMAL(10,2) DEFAULT 0,
    max_discount DECIMAL(10,2),
    
    usage_limit INTEGER,
    usage_limit_per_user INTEGER DEFAULT 1,
    used_count INTEGER DEFAULT 0,
    
    valid_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    valid_until TIMESTAMPTZ,
    
    is_active BOOLEAN DEFAULT TRUE,
    
    applicable_categories UUID[],
    applicable_products UUID[],
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT type_check CHECK (type IN ('percentage', 'fixed_amount', 'free_shipping'))
);

CREATE INDEX idx_coupons_code ON coupons(code);
CREATE INDEX idx_coupons_is_active ON coupons(is_active) WHERE is_active = TRUE;
CREATE INDEX idx_coupons_valid_until ON coupons(valid_until);
```

**Migration 014: Create Inventory Logs**

```sql
-- migrations/014_create_inventory_logs.sql
CREATE TABLE inventory_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    variant_id UUID REFERENCES product_variants(id) ON DELETE CASCADE,
    order_id UUID REFERENCES orders(id) ON DELETE SET NULL,
    
    type VARCHAR(20) NOT NULL,
    quantity_change INTEGER NOT NULL,
    quantity_after INTEGER NOT NULL,
    
    reason TEXT,
    
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT type_check CHECK (type IN (
        'restock', 'sale', 'return', 'damaged', 'lost', 'adjustment'
    ))
);

CREATE INDEX idx_inventory_logs_product_id ON inventory_logs(product_id);
CREATE INDEX idx_inventory_logs_order_id ON inventory_logs(order_id);
CREATE INDEX idx_inventory_logs_created_at ON inventory_logs(created_at DESC);
```

**Migration 015: Create Wishlist**

```sql
-- migrations/015_create_wishlist.sql
CREATE TABLE wishlist_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    variant_id UUID REFERENCES product_variants(id) ON DELETE CASCADE,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(user_id, product_id, variant_id)
);

CREATE INDEX idx_wishlist_user_id ON wishlist_items(user_id);
CREATE INDEX idx_wishlist_product_id ON wishlist_items(product_id);
```

---

## 📁 ESTRUTURA DE CÓDIGO

```
ecommerce-api/
├── Cargo.toml
├── .env.example
├── .gitignore
├── docker-compose.yml
├── Dockerfile
├── README.md
│
├── migrations/
│   ├── 001_create_users.sql
│   ├── 002_create_vendors.sql
│   ├── 003_create_categories.sql
│   ├── 004_create_products.sql
│   ├── 005_create_product_variants.sql
│   ├── 006_create_product_images.sql
│   ├── 007_create_cart_items.sql
│   ├── 008_create_orders.sql
│   ├── 009_create_order_items.sql
│   ├── 010_create_payments.sql
│   ├── 011_create_shipping_info.sql
│   ├── 012_create_reviews.sql
│   ├── 013_create_coupons.sql
│   ├── 014_create_inventory_logs.sql
│   └── 015_create_wishlist.sql
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
│   │   ├── vendor.rs
│   │   ├── product.rs
│   │   ├── category.rs
│   │   ├── cart.rs
│   │   ├── order.rs
│   │   ├── payment.rs
│   │   ├── review.rs
│   │   └── coupon.rs
│   │
│   ├── dto/
│   │   ├── mod.rs
│   │   ├── auth_dto.rs
│   │   ├── product_dto.rs
│   │   ├── cart_dto.rs
│   │   ├── order_dto.rs
│   │   └── checkout_dto.rs
│   │
│   ├── repositories/
│   │   ├── mod.rs
│   │   ├── user_repository.rs
│   │   ├── product_repository.rs
│   │   ├── cart_repository.rs
│   │   ├── order_repository.rs
│   │   ├── payment_repository.rs
│   │   └── inventory_repository.rs
│   │
│   ├── services/
│   │   ├── mod.rs
│   │   ├── auth_service.rs
│   │   ├── product_service.rs
│   │   ├── cart_service.rs
│   │   ├── order_service.rs
│   │   ├── payment_service.rs
│   │   ├── inventory_service.rs
│   │   ├── coupon_service.rs
│   │   └── shipping_service.rs
│   │
│   ├── api/
│   │   ├── mod.rs
│   │   ├── routes.rs
│   │   ├── handlers/
│   │   │   ├── mod.rs
│   │   │   ├── auth_handlers.rs
│   │   │   ├── product_handlers.rs
│   │   │   ├── cart_handlers.rs
│   │   │   ├── order_handlers.rs
│   │   │   ├── checkout_handlers.rs
│   │   │   ├── review_handlers.rs
│   │   │   └── vendor_handlers.rs
│   │   └── middleware/
│   │       ├── mod.rs
│   │       ├── auth_middleware.rs
│   │       └── role_middleware.rs
│   │
│   ├── payment/
│   │   ├── mod.rs
│   │   ├── stripe_client.rs
│   │   └── payment_processor.rs
│   │
│   ├── cache/
│   │   ├── mod.rs
│   │   └── redis_client.rs
│   │
│   ├── workers/
│   │   ├── mod.rs
│   │   ├── email_worker.rs
│   │   └── analytics_worker.rs
│   │
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── jwt.rs
│   │   ├── validators.rs
│   │   ├── slug.rs
│   │   └── price_calculator.rs
│   │
│   ├── error.rs
│   ├── db.rs
│   └── state.rs
│
└── tests/
    ├── common/
    │   └── mod.rs
    ├── integration/
    │   ├── product_tests.rs
    │   ├── cart_tests.rs
    │   ├── checkout_tests.rs
    │   └── order_tests.rs
    └── fixtures/
        └── test_data.rs
```

---

## 📦 DEPENDÊNCIAS (Cargo.toml)

```toml
[package]
name = "ecommerce-api"
version = "0.1.0"
edition = "2021"

[dependencies]
# Web Framework
axum = { version = "0.7", features = ["macros", "multipart"] }
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace", "fs"] }

# Database
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "migrate", "json"] }

# Redis Cache
redis = { version = "0.24", features = ["tokio-comp", "connection-manager"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Payment Processing
stripe-rust = { version = "0.26", features = ["async"] }

# Authentication
jsonwebtoken = "9"
bcrypt = "0.15"

# Utilities
uuid = { version = "1.6", features = ["serde", "v4"] }
chrono = { version = "0.4", features = ["serde"] }
validator = { version = "0.16", features = ["derive"] }
thiserror = "1.0"
anyhow = "1.0"
slug = "0.1"

# Decimal for money
rust_decimal = { version = "1.33", features = ["serde-float"] }

# Email
lettre = { version = "0.11", features = ["tokio1-native-tls"] }

# Image processing
image = "0.24"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# Config
dotenvy = "0.15"

# Job Queue
tokio-cron-scheduler = "0.9"

[dev-dependencies]
reqwest = { version = "0.11", features = ["json"] }
```

---

## 🔌 API ENDPOINTS

### **Catálogo de Produtos**

```
GET    /api/v1/products                 # List products (with filters)
GET    /api/v1/products/:id             # Get product details
GET    /api/v1/products/slug/:slug      # Get product by slug
GET    /api/v1/products/:id/variants    # Get product variants
GET    /api/v1/products/:id/reviews     # Get product reviews
```

**GET /api/v1/products (com filtros)**
```
Query Parameters:
- category: uuid
- vendor: uuid
- min_price: 10.00
- max_price: 100.00
- in_stock: true
- featured: true
- order: asc|desc
- search: search term
- page: 1
- per_page: 20

Example: GET /api/v1/products?category=electronics&min_price=50&max_price=500&sort=price&order=asc

Response (200):
{
  "data": [
    {
      "id": "uuid",
      "vendor": {
        "id": "uuid",
        "store_name": "TechStore",
        "slug": "techstore"
      },
      "category": {
        "id": "uuid",
        "name": "Electronics",
        "slug": "electronics"
      },
      "name": "Wireless Mouse",
      "slug": "wireless-mouse",
      "description": "High precision wireless mouse",
      "price": 49.99,
      "compare_at_price": 79.99,
      "sku": "WM-001",
      "stock_quantity": 150,
      "in_stock": true,
      "images": [
        {
          "url": "https://cdn.example.com/mouse-1.jpg",
          "is_primary": true
        }
      ],
      "rating_average": 4.5,
      "rating_count": 89,
      "variants": [
        {
          "id": "uuid",
          "name": "Black",
          "price": 49.99,
          "stock": 75
        }
      ]
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 245,
    "total_pages": 13
  }
}
```

---

### **Carrinho de Compras**

```
GET    /api/v1/cart                     # Get current user's cart
POST   /api/v1/cart/items               # Add item to cart
PUT    /api/v1/cart/items/:id           # Update cart item quantity
DELETE /api/v1/cart/items/:id           # Remove item from cart
DELETE /api/v1/cart                     # Clear cart
```

**POST /api/v1/cart/items**
```json
Request:
{
  "product_id": "uuid",
  "variant_id": "uuid",  // optional
  "quantity": 2
}

Response (201):
{
  "id": "uuid",
  "product": {
    "id": "uuid",
    "name": "Wireless Mouse",
    "price": 49.99,
    "image_url": "https://..."
  },
  "variant": {
    "id": "uuid",
    "name": "Black"
  },
  "quantity": 2,
  "subtotal": 99.98
}
```

**GET /api/v1/cart**
```json
Response (200):
{
  "items": [
    {
      "id": "uuid",
      "product": {
        "id": "uuid",
        "name": "Wireless Mouse",
        "price": 49.99,
        "stock_quantity": 150,
        "image_url": "https://..."
      },
      "variant": {
        "id": "uuid",
        "name": "Black"
      },
      "quantity": 2,
      "subtotal": 99.98
    }
  ],
  "summary": {
    "items_count": 1,
    "subtotal": 99.98,
    "estimated_shipping": 10.00,
    "estimated_tax": 8.80,
    "estimated_total": 118.78
  }
}
```

---

### **Checkout**

```
POST   /api/v1/checkout/validate        # Validate cart before checkout
POST   /api/v1/checkout/calculate       # Calculate totals with shipping
POST   /api/v1/checkout/process         # Process order
POST   /api/v1/checkout/apply-coupon    # Apply coupon code
```

**POST /api/v1/checkout/calculate**
```json
Request:
{
  "shipping_address": {
    "full_name": "Thiago Bianeck",
    "street": "Rua Example, 123",
    "city": "Francisco Beltrão",
    "state": "PR",
    "postal_code": "85601-000",
    "country": "BR",
    "phone": "+55 46 99999-9999"
  },
  "coupon_code": "DISCOUNT10"  // optional
}

Response (200):
{
  "items": [...],
  "subtotal": 99.98,
  "discount": 10.00,
  "shipping_cost": 15.50,
  "tax": 9.27,
  "total": 114.75,
  "coupon_applied": {
    "code": "DISCOUNT10",
    "type": "percentage",
    "value": 10
  },
  "shipping_options": [
    {
      "carrier": "Correios",
      "service": "SEDEX",
      "cost": 15.50,
      "estimated_days": 3
    },
    {
      "carrier": "Correios",
      "service": "PAC",
      "cost": 10.00,
      "estimated_days": 7
    }
  ]
}
```

**POST /api/v1/checkout/process**
```json
Request:
{
  "shipping_address": {...},
  "billing_address": {...},  // optional, defaults to shipping
  "shipping_method": "SEDEX",
  "payment_method": {
    "type": "credit_card",
    "stripe_token": "tok_..."
  },
  "coupon_code": "DISCOUNT10",
  "customer_notes": "Deliver after 5pm"
}

Response (201):
{
  "order": {
    "id": "uuid",
    "order_number": "ORD-20251115-000123",
    "status": "payment_processing",
    "subtotal": 99.98,
    "discount": 10.00,
    "shipping_cost": 15.50,
    "tax": 9.27,
    "total": 114.75,
    "items": [...],
    "shipping_address": {...},
    "payment": {
      "status": "processing",
      "payment_method": "credit_card"
    },
    "created_at": "2025-11-15T10:00:00Z"
  },
  "message": "Order created successfully. Processing payment..."
}
```

---

### **Pedidos**

```
GET    /api/v1/orders                   # List user's orders
GET    /api/v1/orders/:id               # Get order details
PUT    /api/v1/orders/:id/cancel        # Cancel order
POST   /api/v1/orders/:id/return        # Request return
GET    /api/v1/orders/:id/invoice       # Get invoice PDF
```

**GET /api/v1/orders/:id**
```json
Response (200):
{
  "id": "uuid",
  "order_number": "ORD-20251115-000123",
  "status": "delivered",
  "status_history": [
    {
      "status": "pending",
      "timestamp": "2025-11-15T10:00:00Z"
    },
    {
      "status": "paid",
      "timestamp": "2025-11-15T10:01:30Z"
    },
    {
      "status": "processing",
      "timestamp": "2025-11-15T11:00:00Z"
    },
    {
      "status": "shipped",
      "timestamp": "2025-11-16T09:00:00Z",
      "tracking_number": "BR123456789"
    },
    {
      "status": "delivered",
      "timestamp": "2025-11-18T14:30:00Z"
    }
  ],
  "items": [
    {
      "product_name": "Wireless Mouse",
      "variant_name": "Black",
      "sku": "WM-001-BLK",
      "unit_price": 49.99,
      "quantity": 2,
      "subtotal": 99.98,
      "vendor": {
        "store_name": "TechStore"
      }
    }
  ],
  "subtotal": 99.98,
  "discount": 10.00,
  "shipping_cost": 15.50,
  "tax": 9.27,
  "total": 114.75,
  "shipping_address": {...},
  "shipping_info": {
    "carrier": "Correios",
    "tracking_number": "BR123456789",
    "tracking_url": "https://...",
    "estimated_delivery": "2025-11-18T23:59:59Z",
    "delivered_at": "2025-11-18T14:30:00Z"
  },
  "payment": {
    "payment_method": "credit_card",
    "status": "succeeded",
    "paid_at": "2025-11-15T10:01:30Z"
  },
  "created_at": "2025-11-15T10:00:00Z"
}
```

---

### **Avaliações**

```
POST   /api/v1/reviews                  # Create review
GET    /api/v1/reviews/:id              # Get review
PUT    /api/v1/reviews/:id              # Update review
DELETE /api/v1/reviews/:id              # Delete review
POST   /api/v1/reviews/:id/helpful      # Mark review as helpful
```

**POST /api/v1/reviews**
```json
Request:
{
  "product_id": "uuid",
  "order_id": "uuid",
  "rating": 5,
  "title": "Excellent product!",
  "comment": "Very satisfied with the quality...",
  "images": [
    "https://cdn.example.com/review-1.jpg"
  ]
}

Response (201):
{
  "id": "uuid",
  "product_id": "uuid",
  "user": {
    "id": "uuid",
    "full_name": "Thiago B.",
    "avatar_url": null
  },
  "rating": 5,
  "title": "Excellent product!",
  "comment": "Very satisfied with the quality...",
  "images": [...],
  "is_verified_purchase": true,
  "helpful_count": 0,
  "created_at": "2025-11-20T15:00:00Z"
}
```

---

### **Vendedores (Vendor)**

```
POST   /api/v1/vendors                  # Register as vendor
GET    /api/v1/vendors/me               # Get vendor profile
PUT    /api/v1/vendors/me               # Update vendor profile
GET    /api/v1/vendors/:id              # Get vendor public profile
GET    /api/v1/vendors/:id/products     # Get vendor products
GET    /api/v1/vendors/me/orders        # Get vendor orders
GET    /api/v1/vendors/me/analytics     # Get vendor analytics
```

---

### **Admin**

```
GET    /api/v1/admin/products           # Manage all products
GET    /api/v1/admin/orders             # View all orders
GET    /api/v1/admin/users              # Manage users
GET    /api/v1/admin/vendors            # Manage vendors
GET    /api/v1/admin/analytics          # Platform analytics
PUT    /api/v1/admin/products/:id       # Update any product
PUT    /api/v1/admin/orders/:id/status  # Update order status
```

---

## 💳 INTEGRAÇÃO COM STRIPE

### **Payment Intent Flow**

```mermaid
sequenceDiagram
    participant Client
    participant API
    participant PaymentService
    participant Stripe
    participant OrderService
    participant Inventory
    
    Client->>API: POST /checkout/process
    API->>Inventory: Reserve stock
    Inventory-->>API: Stock reserved
    
    API->>PaymentService: Create payment intent
    PaymentService->>Stripe: Create PaymentIntent
    Stripe-->>PaymentService: client_secret
    
    PaymentService-->>API: Payment intent created
    API-->>Client: client_secret + order_id
    
    Client->>Stripe: Confirm payment (frontend)
    Stripe-->>Client: Payment confirmed
    
    Client->>API: Confirm order completion
    API->>PaymentService: Verify payment
    PaymentService->>Stripe: Retrieve PaymentIntent
    Stripe-->>PaymentService: Payment status
    
    alt Payment Successful
        PaymentService-->>API: Payment confirmed
        API->>OrderService: Update order to "paid"
        OrderService->>Inventory: Deduct stock
        OrderService->>Email: Send confirmation
        API-->>Client: Order confirmed
    else Payment Failed
        PaymentService-->>API: Payment failed
        API->>Inventory: Release reserved stock
        API->>OrderService: Update order to "failed"
        API-->>Client: Payment failed
    end
```

### **Implementação Stripe**

```rust
use stripe::{Client, CreatePaymentIntent, Currency, PaymentIntent};

pub struct StripeService {
    client: Client,
}

impl StripeService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(api_key),
        }
    }
    
    pub async fn create_payment_intent(
        &self,
        amount: i64,  // em centavos
        currency: Currency,
        metadata: HashMap<String, String>,
    ) -> Result<PaymentIntent, StripeError> {
        let mut params = CreatePaymentIntent::new(amount, currency);
        params.metadata = Some(metadata);
        params.automatic_payment_methods = Some(AutomaticPaymentMethodsParams {
            enabled: true,
            ..Default::default()
        });
        
        PaymentIntent::create(&self.client, params).await
    }
    
    pub async fn retrieve_payment_intent(
        &self,
        id: &str,
    ) -> Result<PaymentIntent, StripeError> {
        PaymentIntent::retrieve(&self.client, &PaymentIntentId::from_str(id)?).await
    }
    
    pub async fn create_refund(
        &self,
        charge_id: &str,
        amount: Option<i64>,
    ) -> Result<Refund, StripeError> {
        let mut params = CreateRefund::new();
        params.charge = Some(charge_id.to_string());
        params.amount = amount;
        
        Refund::create(&self.client, params).await
    }
}
```

---

## 📦 GESTÃO DE INVENTÁRIO

### **Stock Management com Transações**

```rust
pub struct InventoryService {
    db: PgPool,
}

impl InventoryService {
    /// Reservar estoque (durante checkout)
    pub async fn reserve_stock(
        &self,
        items: Vec<(Uuid, Option<Uuid>, i32)>, // (product_id, variant_id, quantity)
    ) -> Result<(), InventoryError> {
        let mut tx = self.db.begin().await?;
        
        for (product_id, variant_id, quantity) in items {
            if let Some(variant_id) = variant_id {
                // Variant stock
                let result = sqlx::query!(
                    "UPDATE product_variants 
                     SET stock_quantity = stock_quantity - $1 
                     WHERE id = $2 AND stock_quantity >= $1
                     RETURNING stock_quantity",
                    quantity,
                    variant_id
                )
                .fetch_optional(&mut *tx)
                .await?;
                
                if result.is_none() {
                    tx.rollback().await?;
                    return Err(InventoryError::InsufficientStock);
                }
            } else {
                // Product stock
                let result = sqlx::query!(
                    "UPDATE products 
                     SET stock_quantity = stock_quantity - $1 
                     WHERE id = $2 AND stock_quantity >= $1
                     RETURNING stock_quantity",
                    quantity,
                    product_id
                )
                .fetch_optional(&mut *tx)
                .await?;
                
                if result.is_none() {
                    tx.rollback().await?;
                    return Err(InventoryError::InsufficientStock);
                }
            }
            
            // Log inventory change
            sqlx::query!(
                "INSERT INTO inventory_logs 
                 (product_id, variant_id, type, quantity_change, quantity_after)
                 VALUES ($1, $2, 'sale', $3, (
                     SELECT COALESCE(
                         (SELECT stock_quantity FROM product_variants WHERE id = $2),
                         (SELECT stock_quantity FROM products WHERE id = $1)
                     )
                 ))",
                product_id,
                variant_id,
                -quantity
            )
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        Ok(())
    }
    
    /// Restaurar estoque (cancelamento)
    pub async fn release_stock(
        &self,
        order_id: Uuid,
    ) -> Result<(), InventoryError> {
        let mut tx = self.db.begin().await?;
        
        let items = sqlx::query!(
            "SELECT product_id, variant_id, quantity 
             FROM order_items 
             WHERE order_id = $1",
            order_id
        )
        .fetch_all(&mut *tx)
        .await?;
        
        for item in items {
            if let Some(variant_id) = item.variant_id {
                sqlx::query!(
                    "UPDATE product_variants 
                     SET stock_quantity = stock_quantity + $1 
                     WHERE id = $2",
                    item.quantity,
                    variant_id
                )
                .execute(&mut *tx)
                .await?;
            } else {
                sqlx::query!(
                    "UPDATE products 
                     SET stock_quantity = stock_quantity + $1 
                     WHERE id = $2",
                    item.quantity,
                    item.product_id
                )
                .execute(&mut *tx)
                .await?;
            }
            
            sqlx::query!(
                "INSERT INTO inventory_logs 
                 (product_id, variant_id, type, quantity_change, quantity_after, order_id)
                 VALUES ($1, $2, 'return', $3, (
                     SELECT COALESCE(
                         (SELECT stock_quantity FROM product_variants WHERE id = $2),
                         (SELECT stock_quantity FROM products WHERE id = $1)
                     )
                 ), $4)",
                item.product_id,
                item.variant_id,
                item.quantity,
                order_id
            )
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        Ok(())
    }
}
```

---

## 💰 SISTEMA DE CUPONS

### **Validação e Aplicação**

```rust
pub struct CouponService {
    db: PgPool,
}

impl CouponService {
    pub async fn validate_and_apply(
        &self,
        code: &str,
        user_id: Uuid,
        cart_total: Decimal,
        cart_items: &[CartItem],
    ) -> Result<CouponDiscount, CouponError> {
        let coupon = sqlx::query_as!(
            Coupon,
            "SELECT * FROM coupons 
             WHERE code = $1 
             AND is_active = TRUE
             AND valid_from <= NOW()
             AND (valid_until IS NULL OR valid_until >= NOW())",
            code
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or(CouponError::Invalid)?;
        
        // Verificar limite de uso
        if let Some(limit) = coupon.usage_limit {
            if coupon.used_count >= limit {
                return Err(CouponError::LimitReached);
            }
        }
        
        // Verificar uso por usuário
        let user_usage = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM orders 
             WHERE user_id = $1 AND coupon_id = $2",
            user_id,
            coupon.id
        )
        .fetch_one(&self.db)
        .await?
        .unwrap_or(0);
        
        if user_usage >= coupon.usage_limit_per_user {
            return Err(CouponError::UserLimitReached);
        }
        
        // Verificar valor mínimo
        if cart_total < coupon.min_purchase {
            return Err(CouponError::MinimumNotMet);
        }
        
        // Calcular desconto
        let discount = match coupon.type_.as_str() {
            "percentage" => {
                let discount = cart_total * (coupon.value / Decimal::from(100));
                if let Some(max) = coupon.max_discount {
                    discount.min(max)
                } else {
                    discount
                }
            }
            "fixed_amount" => coupon.value.min(cart_total),
            "free_shipping" => Decimal::ZERO, // handled separately
            _ => return Err(CouponError::InvalidType),
        };
        
        Ok(CouponDiscount {
            coupon_id: coupon.id,
            code: coupon.code,
            type_: coupon.type_,
            discount,
        })
    }
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
    container_name: ecommerce-db
    environment:
      POSTGRES_USER: ecommerce
      POSTGRES_PASSWORD: ecommerce_dev
      POSTGRES_DB: ecommerce
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ecommerce"]
      interval: 5s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    container_name: ecommerce-cache
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
    container_name: ecommerce-api
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy
    environment:
      DATABASE_URL: postgres://ecommerce:ecommerce_dev@postgres/ecommerce
      REDIS_URL: redis://redis:6379
      JWT_SECRET: your-secret-key-change-in-production
      STRIPE_SECRET_KEY: sk_test_...
      BASE_URL: http://localhost:8000
      RUST_LOG: info
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
- ✅ Docker (PostgreSQL + Redis)
- ✅ Migrations (users, vendors)
- ✅ Auth service (JWT)
- ✅ User/Vendor registration

### **Dia 54: Catálogo de Produtos**
- ✅ Products CRUD
- ✅ Categories system
- ✅ Product variants
- ✅ Image upload
- ✅ Search e filtros

### **Dia 55: Carrinho de Compras**
- ✅ Cart persistence
- ✅ Add/Update/Remove items
- ✅ Stock validation
- ✅ Price calculation
- ✅ Redis caching

### **Dia 56: Checkout e Pagamentos**
- ✅ Checkout flow
- ✅ Stripe integration
- ✅ Order creation
- ✅ Inventory reservation
- ✅ Email notifications

### **Dia 57: Gestão de Pedidos**
- ✅ Order states machine
- ✅ Order history
- ✅ Tracking info
- ✅ Cancellation/Returns
- ✅ Invoice generation

### **Dia 58: Reviews e Cupons**
- ✅ Review system
- ✅ Rating aggregation
- ✅ Coupon validation
- ✅ Discount calculation
- ✅ Wishlist

### **Dia 59: Testes e Docs**
- ✅ Unit tests
- ✅ Integration tests
- ✅ Payment tests (mock)
- ✅ API documentation
- ✅ Postman collection

### **Dia 60: Analytics e Deploy**
- ✅ Vendor analytics
- ✅ Sales reports
- ✅ Performance tuning
- ✅ Docker production
- ✅ CI/CD setup

---

## 🚀 QUICK START

```bash
# 1. Clone e setup
git clone <repo>
cd ecommerce-api
cp .env.example .env

# Edite .env e adicione suas chaves:
# STRIPE_SECRET_KEY=sk_test_...

# 2. Start services
docker-compose up -d

# 3. Run migrations
sqlx migrate run

# 4. Seed database (opcional)
cargo run --bin seed

# 5. Run API
cargo run

# 6. Test
curl http://localhost:8000/api/v1/health

# 7. Create test user
curl -X POST http://localhost:8000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123",
    "full_name": "Test User"
  }'

# 8. Browse products
curl http://localhost:8000/api/v1/products
```

---

## 📈 PERFORMANCE TARGETS

### **Latência**
- Product listing: < 100ms
- Cart operations: < 50ms
- Checkout process: < 500ms
- Payment processing: < 2s (dependente do Stripe)

### **Throughput**
- Concurrent users: 1,000 per instance
- Orders per minute: 500
- Product queries per second: 2,000

### **Disponibilidade**
- Uptime: 99.9%
- Cache hit rate: > 80%

---

## 💡 RECURSOS ADICIONAIS

**Crates Importantes:**
- [stripe-rust](https://docs.rs/stripe-rust/) - Stripe API
- [rust_decimal](https://docs.rs/rust_decimal/) - Precisão decimal para dinheiro
- [lettre](https://docs.rs/lettre/) - Email
- [image](https://docs.rs/image/) - Processamento de imagens

**Inspiração:**
- [Shopify](https://www.shopify.com/)
- [WooCommerce](https://woocommerce.com/)
- [Magento](https://magento.com/)

**APIs Externas:**
- [Stripe](https://stripe.com/docs/api) - Pagamentos
- [Correios](http://www.correios.com.br/para-voce/correios-de-a-a-z/pdf/calculador-remoto-de-precos-e-prazos/manual-de-implementacao-do-calculo-remoto-de-precos-e-prazos) - Cálculo de frete
- [ViaCEP](https://viacep.com.br/) - Consulta CEP

---

**Plataforma E-commerce completa! Multi-vendor + Payments + Inventory = Sistema robusto e escalável! 🛒💳🚀**
