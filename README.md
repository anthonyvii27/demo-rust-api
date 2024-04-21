# Book Management API

This is a simple API for creating, listing, updating, and deleting books.

## How to Run

To run the project locally, follow these steps:

1. **Clone the repository:**
   ```bash
   git clone https://github.com/anthonyvii27/demo-rust-api.git
   ```

2. **Navigate to the project directory:**
   ```bash
   cd demo-rust-api
   ```

3. **Run the project with Cargo:**
   ```bash
   cargo run
   ```

4. **Access the API endpoints:**
  - Once the server is running, you can access the API endpoints using CURL commands or your preferred HTTP client.

## Endpoints

### 1. Create a new book

Creates a new book with the provided information.

- **URL**: `/books`
- **HTTP Method**: `POST`
- **Request Body (JSON)**:
  ```json
  {
    "name": "Book Name",
    "author": "Book Author",
    "number_of_pages": 300,
    "category": "ScienceFiction"
  }
  ```
- **Example CURL**:
  ```bash
  curl -X POST \
    http://localhost:8080/books \
    -H 'Content-Type: application/json' \
    -d '{
      "name": "Book Name",
      "author": "Book Author",
      "number_of_pages": 300,
      "category": "ScienceFiction"
    }'
  ```

### 2. Get information of a book by ID

Retrieves the information of a specific book based on the provided ID.

- **URL**: `/books/{id}`
- **HTTP Method**: `GET`
- **URL Parameters**: `{id}` - Book ID
- **Example CURL**:
  ```bash
  curl -X GET \
    http://localhost:8080/books/123 \
    -H 'Content-Type: application/json'
  ```

### 3. Update information of a book by ID

Updates the information of a specific book based on the provided ID.

- **URL**: `/books/{id}`
- **HTTP Method**: `PUT`
- **URL Parameters**: `{id}` - Book ID
- **Request Body (JSON)**:
  ```json
  {
    "name": "New Book Name",
    "author": "New Book Author",
    "number_of_pages": 400,
    "category": "Fantasy"
  }
  ```
- **Example CURL**:
  ```bash
  curl -X PUT \
    http://localhost:8080/books/123 \
    -H 'Content-Type: application/json' \
    -d '{
      "name": "New Book Name",
      "author": "New Book Author",
      "number_of_pages": 400,
      "category": "Fantasy"
    }'
  ```

### 4. Delete a book by ID

Deletes a specific book based on the provided ID.

- **URL**: `/books/{id}`
- **HTTP Method**: `DELETE`
- **URL Parameters**: `{id}` - Book ID
- **Example CURL**:
  ```bash
  curl -X DELETE \
    http://localhost:8080/books/123 \
    -H 'Content-Type: application/json'
  ```