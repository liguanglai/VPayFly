# 系统架构设计文档

## 1. 简介

本文档描述了虚拟商品贩卖平台的系统架构。该平台旨在提供一个安全、高效的虚拟商品（如网盘链接）自动售卖和交付系统。

## 2. 总体架构

本系统采用经典的前后端分离架构 (Client-Server Architecture)。

*   **前端 (Frontend)**: 基于 React 的单页应用 (SPA)。分为面向公众的“购买端”和面向管理员的“管理后台”。
*   **后端 (Backend)**: 基于 Rust 语言构建的高性能 RESTful API 服务。
*   **数据库 (Database)**: 使用关系型数据库 PostgreSQL 存储商品、订单和管理员数据。
*   **外部服务 (External Services)**:
    *   **支付网关**: 集成支付宝、微信支付、Wise 和 Apple Pay 处理在线支付。
    *   **邮件服务**: 使用第三方 SMTP 服务或 API (如 SendGrid, Mailgun) 发送包含下载链接的邮件。

```mermaid
graph TD
    User[买家] -->|HTTPS| Frontend_Public[前端 - 购买页面 (React)]
    Admin[管理员] -->|HTTPS| Frontend_Admin[前端 - 管理后台 (React + Ant Design)]
    
    Frontend_Public -->|REST API| Backend[后端 API 服务 (Rust)]
    Frontend_Admin -->|REST API| Backend
    
    Backend -->|SQL| DB[(PostgreSQL 数据库)]
    
    Backend -->|API| Payment[支付网关 (Alipay, WeChat, Wise, Apple Pay)]
    Payment -->|Webhook| Backend
    
    Backend -->|SMTP/API| Email[邮件服务]
    Email -->|Email| User
```

## 3. 组件详情

### 3.1 前端应用 (Frontend)

*   **技术栈**: React, Vite, TypeScript.
*   **模块划分**:
    *   **Storefront (商店前台)**:
        *   设计风格: 极简主义。
        *   功能: 商品列表展示、商品详情页、简单的结账流程（输入邮箱、选择支付方式）。
        *   安全: **严禁**在前端代码或 API 响应中暴露商品的真实下载链接。
    *   **Admin Panel (管理后台)**:
        *   UI 库: Ant Design。
        *   功能: 商品增删改查 (CRUD)、订单查看、管理员登录。
        *   安全: 受 JWT 认证保护。

### 3.2 后端服务 (Backend)

*   **技术栈**: Rust.
*   **框架建议**: Axum 或 Actix-web (两者均具备高性能和成熟的生态)。
*   **核心职责**:
    *   提供 RESTful API 接口。
    *   处理业务逻辑（订单创建、支付状态流转）。
    *   **安全性**: 负责验证支付回调的签名，确保交易真实性；负责保管商品链接，仅在订单完成后通过邮件发送。
    *   身份验证: 使用 JWT 对管理接口进行鉴权。

### 3.3 数据存储 (Storage)

*   **PostgreSQL**: 用于持久化存储。
*   **数据模型**: 主要包含 `Users` (Admin), `Products` (商品), `Orders` (订单)。

### 3.4 部署架构 (Deployment)

*   **容器化**: 建议使用 Docker 将前端（构建为静态资源由 Nginx 托管）和后端（Rust 二进制文件）容器化。
*   **反向代理**: 使用 Nginx 或 Caddy 作为反向代理，处理 SSL/TLS 终止，并将请求转发给后端 API 或服务静态文件。

## 4. 关键流程架构考虑

### 4.1 自动发货流程
系统不提供在线查看链接的功能，而是通过邮件发送。这减少了前端被抓包导致链接泄露的风险，同时也验证了用户邮箱的有效性。

### 4.2 支付回调处理
支付成功依赖于支付网关的异步 Webhook 通知。后端必须设计幂等性处理机制，防止重复通知导致重复发货或数据错误。

### 4.3 安全性设计
*   **链接保密**: 商品的 `download_link` 字段仅在数据库中明文存储（或加密存储），绝不通过 `GET /products` 接口返回给前端。
*   **输入验证**: 对邮箱格式、金额等进行严格的后端验证。
