from pathlib import Path
content = """# Dodo Payments Backend Assignment

## What I am building

- A small backend in Rust for managing customers, invoices, and payment attempts.
- I am using this project to practise backend concepts that matter in a payment system:
  - keeping invoice and payment states correct
  - handling multiple requests safely
  - storing data in PostgreSQL
  - handling payment success and failure
- The payment provider is mocked for now. This project does not charge real money.

## Tech used

- Rust
- Axum for HTTP APIs
- Tokio for async work
- PostgreSQL for storing data
- SQLx for database queries and migrations
- Serde for JSON
- Tracing for logs

## What is implemented so far

- [x] Rust application setup with Axum
- [x] PostgreSQL connection and migrations
- [x] Customer API
  - Create a customer
  - Get a customer by ID
  - Prevent duplicate email addresses
- [x] Invoice API
  - Create an invoice
  - Get an invoice by ID
  - Validate that the amount is positive
  - Store currency as a three-letter code
- [x] Basic invoice state model
  - `pending`
  - `processing`
  - `paid`
  - `failed`
  - `cancelled`
- [x] Start a payment attempt for an invoice
- [x] Mock payment provider
  - Amount `9999` is used to simulate a failed payment
  - Other amounts simulate a successful payment
- [x] Save the payment result and update the invoice status
- [x] Basic protection against starting two payment attempts for the same invoice at the same time, using a conditional database update

## API endpoints

| Method | Endpoint | Purpose |
|---|---|---|
| `GET` | `/health` | Check that the server is running |
| `POST` | `/customers` | Create a customer |
| `GET` | `/customers/{id}` | Get a customer |
| `POST` | `/invoices` | Create an invoice |
| `GET` | `/invoices/{id}` | Get an invoice |
| `POST` | `/invoices/{id}/payments` | Start a payment attempt |

## How the payment flow works

1. A request comes in to pay an invoice.
2. The backend checks the invoice in PostgreSQL and tries to move it from `pending` or `failed` to `processing`.
3. The status change is conditional, so another request cannot claim the same invoice while it is already `processing`.
4. The backend creates a payment record.
5. The mock provider returns a success or failure result.
6. The backend saves the payment result and updates the invoice to `paid` or `failed`.
7. The API returns the updated payment information.

## Database tables

- `customers`
  - Stores customer ID, email, name, and creation time.
- `invoices`
  - Stores customer, amount, currency, status, and timestamps.
- `payments`
  - Stores each payment attempt, its status, and provider reference.
- `idempotency_keys`
  - Table created for the planned idempotency work. The full idempotency flow is not implemented yet.

Amounts are stored as integers in the smallest unit of the currency, for example paise for INR.

## Setup

### Requirements

- Rust and Cargo
- PostgreSQL
- SQLx CLI, if you want to run migrations from the command line

### Environment variables

Create a `.env` file in the project root:

```env
DATABASE_URL=postgres://dodo_user:change_this_password@localhost:5432/dodo_payments
RUST_LOG=debug
