# API 接口设计文档

## 1. 概述

本 API 遵循 RESTful 风格，所有请求和响应体均使用 JSON 格式。
*   基础路径: `/api/v1`
*   日期时间格式: ISO 8601 (e.g., `2023-10-27T10:00:00Z`)

## 2. 公开接口 (Public Endpoints)

无需认证，供前台商店使用。

### 2.1 获取商品列表
*   **Endpoint**: `GET /products`
*   **Description**: 获取所有状态为“已上架”的商品。**注意：此接口绝不返回下载链接。**
*   **Response**:
    ```json
    [
      {
        "id": "uuid-string",
        "name": "Baidu Cloud Resource Pack 1",
        "description": "20GB of resources...",
        "price": 9.99,
        "currency": "CNY",
        "created_at": "..."
      }
    ]
    ```

### 2.2 获取商品详情
*   **Endpoint**: `GET /products/:id`
*   **Description**: 获取单个商品的详细信息。
*   **Response**:
    ```json
    {
      "id": "uuid-string",
      "name": "...",
      "description": "...",
      "price": 9.99,
      "currency": "CNY"
    }
    ```

### 2.3 创建订单 (发起支付)
*   **Endpoint**: `POST /orders`
*   **Description**: 用户提交邮箱并选择支付方式，系统创建待支付订单并返回支付跳转链接或二维码数据。
*   **Request**:
    ```json
    {
      "product_id": "uuid-string",
      "email": "user@example.com",
      "payment_method": "alipay" // enum: alipay, wechat, wise, apple_pay
    }
    ```
*   **Response**:
    ```json
    {
      "order_id": "uuid-string",
      "status": "pending",
      "payment_url": "https://gateway.alipay.com/..." // 前端重定向至此链接
    }
    ```

### 2.4 支付 Webhook (回调)
*   **Endpoint**: `POST /webhooks/:provider` (e.g., `/webhooks/alipay`)
*   **Description**: 接收支付网关的异步通知。
*   **Request**: (格式取决于具体支付提供商)
*   **Response**: `200 OK` (确认收到)

---

## 3. 管理接口 (Admin Endpoints)

需要 `Authorization: Bearer <token>` 头。

### 3.1 管理员登录
*   **Endpoint**: `POST /auth/login`
*   **Request**:
    ```json
    {
      "username": "admin",
      "password": "secret_password"
    }
    ```
*   **Response**:
    ```json
    {
      "token": "jwt_token_string",
      "expires_in": 3600
    }
    ```

### 3.2 获取所有商品 (管理端)
*   **Endpoint**: `GET /admin/products`
*   **Description**: 获取所有商品，包括未上架的，且包含下载链接字段。
*   **Response**:
    ```json
    [
      {
        "id": "...",
        "name": "...",
        "price": 10.00,
        "is_published": true,
        "download_link": "https://pan.baidu.com/s/xxxx", // 管理员可见
        "created_at": "..."
      }
    ]
    ```

### 3.3 创建商品
*   **Endpoint**: `POST /admin/products`
*   **Request**:
    ```json
    {
      "name": "New Resource",
      "description": "Details...",
      "price": 19.99,
      "currency": "CNY",
      "download_link": "https://pan.baidu.com/s/newlink",
      "is_published": false
    }
    ```
*   **Response**: `201 Created` (包含新创建的商品对象)

### 3.4 更新商品
*   **Endpoint**: `PUT /admin/products/:id`
*   **Request**: (包含需要更新的字段)
*   **Response**: `200 OK`

### 3.5 删除商品
*   **Endpoint**: `DELETE /admin/products/:id`
*   **Response**: `204 No Content`

### 3.6 查看订单列表 (可选)
*   **Endpoint**: `GET /admin/orders`
*   **Description**: 查看历史销售记录。
*   **Response**: 分页返回订单列表。
