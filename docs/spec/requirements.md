# Software Requirements Specification (SRS)

## 1. Overview

### 1.1 Project Description
The Virtual Goods Sales Platform is a web application designed to facilitate the sale of digital products, specifically download links for resources hosted on cloud storage services like Baidu Cloud and Quark Cloud. The system automates the delivery process: upon successful payment, the purchased product link is automatically sent to the buyer's email address.

### 1.2 Scope
The system consists of a public-facing storefront for buyers and a secured administration panel for sellers/admins.
*   **Storefront**: Allows users to browse products, purchase them using various payment methods, and receive delivery via email.
*   **Admin Panel**: Allows administrators to manage the product catalog (create, read, update, delete) and monitor orders.

### 1.3 Key Objectives
*   Simplify the purchase process for virtual goods.
*   Automate the delivery of digital content to reduce manual intervention.
*   Support multiple popular payment gateways (WeChat Pay, Alipay, Wise, Apple Pay).

---

## 2. User Stories

### 2.1 Buyer Stories
*   **US-01**: As a buyer, I want to view a list of available virtual products so that I can choose what to buy.
*   **US-02**: As a buyer, I want to click on a product to see its details (description, price) so that I know what I am purchasing.
*   **US-03**: As a buyer, I want to be able to enter my email address during checkout so that I can receive the product link.
*   **US-04**: As a buyer, I want to select my preferred payment method (WeChat Pay, Alipay, Wise, Apple Pay) so that I can pay conveniently.
*   **US-05**: As a buyer, I want to receive an email containing the product link immediately after my payment is confirmed so that I can access my purchase.

### 2.2 Administrator Stories
*   **US-06**: As an admin, I want to log in to a secure backend area so that I can manage the shop.
*   **US-07**: As an admin, I want to view a list of all products (including published and unpublished) to manage inventory.
*   **US-08**: As an admin, I want to add new products with a name, description, price, and the download link so that I can sell new items.
*   **US-09**: As an admin, I want to edit existing product details so that I can correct errors or update pricing.
*   **US-10**: As an admin, I want to publish or unpublish products so that I can control what is visible to buyers.
*   **US-11**: As an admin, I want to delete products that are no longer needed.

---

## 3. Functional Requirements

### 3.1 Storefront (Public)
*   **FR-01 Product Listing**: The system shall display a grid or list of all products currently marked as "published".
*   **FR-02 Product Detail**: The system shall provide a dedicated page or modal for each product displaying its name, full description, and price.
*   **FR-03 Order Creation**: The system shall allow users to initiate an order by providing a valid email address.
*   **FR-04 Payment Integration**:
    *   The system must integrate with Alipay, WeChat Pay, Wise, and Apple Pay.
    *   The system shall redirect the user to the payment gateway or display the necessary payment QR code/widget.
*   **FR-05 Order Fulfillment**:
    *   Upon receiving a "success" callback from the payment gateway, the system shall automatically trigger an email.
    *   The email must contain the specific `link` associated with the purchased product.

### 3.2 Admin Panel (Protected)
*   **FR-06 Authentication**: Access to the admin panel must be secured via a login screen (Username/Password).
*   **FR-07 Dashboard/List View**: The admin interface shall use Ant Design components to list products in a table format.
*   **FR-08 Product Management**:
    *   Admins shall be able to input: Name, Description, Price, Currency, and the secret **Download Link**.
    *   Admins shall be able to toggle a boolean flag `is_published`.

### 3.3 Backend Service
*   **FR-09 API Structure**: The backend shall be implemented in **Rust** (using frameworks like Actix-web or Axum).
*   **FR-10 Database**: The system shall use a relational database (e.g., PostgreSQL) to store `products` and `orders`.
*   **FR-11 Webhooks**: The backend shall expose endpoints to receive asynchronous payment notifications (webhooks) from payment providers to update order status.

---

## 4. Non-Functional Requirements

### 4.1 Performance
*   **Response Time**: API endpoints should respond within 200ms under normal load.
*   **Concurrency**: The system should handle concurrent purchase requests for the same product without error.

### 4.2 Security
*   **Data Protection**: User email addresses should be stored securely.
*   **Admin Access**: The admin panel must use strong authentication (e.g., JWT).
*   **Payment Security**: The system must verify signatures from payment gateways to prevent spoofed payment notifications.
*   **Link Secrecy**: The product download links must not be exposed in the frontend API response for the product list; they are only retrieved server-side upon payment completion.

### 4.3 Usability
*   **Simplicity**: The buyer interface should be "minimalist" (as requested), focusing on the easiest path to purchase.
*   **Admin UI**: The admin panel should leverage Ant Design for a standard, professional look and feel.

### 4.4 Reliability
*   **Email Delivery**: The system should use a reliable transactional email provider (e.g., SendGrid, Mailgun) to ensure links do not land in spam.

---

## 5. Edge Cases

*   **EC-01 Payment Timeout**: User initiates payment but does not complete it. The order remains 'pending' and should eventually be treated as abandoned.
*   **EC-02 Invalid Email**: User enters a typo in the email (e.g., `user@gamil.com`). The system should attempt basic validation, but if delivery fails, the system logs the error.
*   **EC-03 Payment Failure**: Payment gateway returns a failure status. The user should be notified, and no email should be sent.
*   **EC-04 Double Payment**: Rare case where a user might pay twice for the same order ID. System should log this for manual refund processing.
*   **EC-05 Out of Stock/Link Dead**: Although virtual goods don't run out, the link might become invalid (e.g., Baidu Cloud link expires). Admins need to be able to update links easily.

---

## 6. Acceptance Criteria

*   **AC-01**: A user can successfully navigate to the site, select a product, pay via a simulated payment gateway, and receive an email with the correct link.
*   **AC-02**: An admin can log in, create a new product, and see it appear on the public storefront immediately.
*   **AC-03**: An admin can unpublish a product, and it immediately disappears from the public storefront.
*   **AC-04**: The product download link is **never** visible in the browser's network tab during the browsing/shopping phase.
*   **AC-05**: The solution is implemented using Rust for the backend and React (with Ant Design for admin) for the frontend.
