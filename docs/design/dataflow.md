# 数据流设计文档 (Data Flow)

## 1. 购买流程数据流

此流程描述用户从浏览商品到收到发货邮件的完整数据流转。

1.  **商品展示**
    *   `Frontend` 请求 `GET /api/products`。
    *   `Backend` 查询 `Database` (`SELECT * FROM products WHERE is_published = true`)。
    *   `Database` 返回商品数据（**过滤掉 download_link**）。
    *   `Backend` 返回 JSON 列表给 `Frontend`。

2.  **创建订单**
    *   用户在 `Frontend` 输入邮箱并点击支付。
    *   `Frontend` 发送 `POST /api/orders` (Payload: `product_id`, `email`, `payment_method`)。
    *   `Backend` 在 `Database` 的 `orders` 表中插入一条记录，状态为 `pending`。
    *   `Backend` 根据 `payment_method` 调用对应的 `Payment Gateway SDK` 获取支付参数（如 URL 或 Token）。
    *   `Backend` 返回支付参数给 `Frontend`。

3.  **支付处理**
    *   `Frontend` 引导用户跳转到支付网关页面。
    *   用户在支付网关完成支付。
    *   支付网关向 `Backend` 的 `POST /api/webhooks/...` 发送异步通知。

4.  **订单完成与发货**
    *   `Backend` 接收 Webhook，验证签名。
    *   `Backend` 更新 `Database` 中对应订单的状态为 `paid`。
    *   `Backend` 查询 `Database` 获取对应商品的 `download_link`。
    *   `Backend` 调用 `Email Service`，将包含 `download_link` 的邮件发送给 `email`。
    *   `Email Service` 将邮件投递给用户。

## 2. 管理流程数据流

1.  **商品上架**
    *   管理员在 `Admin Frontend` 填写商品信息（含链接）。
    *   `Admin Frontend` 发送 `POST /api/admin/products` (携带 JWT)。
    *   `Backend` 验证 JWT。
    *   `Backend` 将完整商品信息（含链接）写入 `Database`。

2.  **查看销售记录**
    *   管理员请求 `GET /api/admin/orders`。
    *   `Backend` 查询 `Database` 的 `orders` 表。
    *   `Backend` 返回订单列表给 `Admin Frontend`。
