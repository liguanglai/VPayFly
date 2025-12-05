# 数据库存储模式 (Storage Schema)

本系统使用关系型数据库 PostgreSQL。

## 1. Tables (表结构)

### 1.1 `products` (商品表)

存储所有虚拟商品的信息。

| Column Name | Data Type | Constraints | Description |
| :--- | :--- | :--- | :--- |
| `id` | `UUID` | `PRIMARY KEY`, `DEFAULT gen_random_uuid()` | 商品唯一标识 |
| `name` | `VARCHAR(255)` | `NOT NULL` | 商品名称 |
| `description` | `TEXT` | | 商品详细描述 |
| `price` | `DECIMAL(10, 2)` | `NOT NULL` | 价格 |
| `currency` | `VARCHAR(3)` | `NOT NULL`, `DEFAULT 'CNY'` | 货币代码 (ISO 4217) |
| `download_link` | `TEXT` | `NOT NULL` | **核心数据**: 网盘下载链接 (不对外公开) |
| `is_published` | `BOOLEAN` | `DEFAULT false` | 上架状态 |
| `created_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | 创建时间 |
| `updated_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | 更新时间 |

### 1.2 `orders` (订单表)

记录所有交易流水。

| Column Name | Data Type | Constraints | Description |
| :--- | :--- | :--- | :--- |
| `id` | `UUID` | `PRIMARY KEY`, `DEFAULT gen_random_uuid()` | 订单唯一标识 |
| `product_id` | `UUID` | `FOREIGN KEY REFERENCES products(id)` | 购买的商品ID |
| `customer_email` | `VARCHAR(255)` | `NOT NULL` | 接收链接的买家邮箱 |
| `status` | `VARCHAR(20)` | `NOT NULL`, `CHECK (status IN ('pending', 'paid', 'failed', 'cancelled'))` | 订单状态 |
| `amount` | `DECIMAL(10, 2)` | `NOT NULL` | 实际支付金额 (快照) |
| `currency` | `VARCHAR(3)` | `NOT NULL` | 实际支付货币 (快照) |
| `payment_method` | `VARCHAR(50)` | | 支付方式 (e.g., 'alipay') |
| `external_txn_id` | `VARCHAR(255)` | | 支付网关的交易流水号 |
| `created_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | 创建时间 |
| `updated_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | 更新时间 |

### 1.3 `admins` (管理员表)

存储后台管理员账户。

| Column Name | Data Type | Constraints | Description |
| :--- | :--- | :--- | :--- |
| `id` | `UUID` | `PRIMARY KEY`, `DEFAULT gen_random_uuid()` | 管理员ID |
| `username` | `VARCHAR(50)` | `UNIQUE`, `NOT NULL` | 登录用户名 |
| `password_hash` | `VARCHAR(255)` | `NOT NULL` | 加密后的密码 (Argon2 or bcrypt) |
| `created_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | 创建时间 |

## 2. 索引 (Indexes)

*   `products`: `is_published` (加速前台查询)
*   `orders`: `customer_email`, `status`, `created_at` (用于后台筛选和统计)
*   `orders`: `external_txn_id` (加速 Webhook 处理时的查找)
