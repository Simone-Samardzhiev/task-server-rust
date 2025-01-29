# task-server-rust

Web server that will store task with features like, register
login for users and get, add, update, delete tasks for tasks.

## Table of content

- [About the project](#about-the-project)
- [Installation](#installation)
- [API](#api)

## About the project

Web server used to help user stay organized by storing tasks. It includes

- JWT-based authentication
- Task creation, updating and deleting
- Database integration with postgres

## Installation

Follow the steps to build and run the server locally.

### Prerequisites

1. Rust(1.84.0)
2. PostgresSQL
3. Git

### Steps

1. **Clone the repository**.

```bash
git clone https://github.com/your-username/server-go.git
cd task-server-rust
```

2. **Add .env file**

```ini
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
SERVER_SOCKET=127.0.0.1:8080
JWT_SECRET=secret
```

3. **Build and run**

```bash
cargo build --release
cd tatget
cd release
./server
```

## API

### 1. **POST /users/register**

The endpoint allows users to register.

#### **Request Body**

The body of the request should contain user credentials

```json
{
  "email": "exmaple@email.com",
  "password": "password123"
}
```

### 2. **POST /users/login**

The endpoint allows user to receive JWT refresh and access token.

#### **Request Body**

The body of the request should contain user credentials

```json
{
  "email": "exmaple@email.com",
  "password": "password123"
}
```

#### **Response**

If the credential the server will return **Status Code Unauthorized**.  
If not the response will be like:

```json
{
  "refresh_token": "token",
  "access_token": "token"
}
```

### 3. **GET /users/refresh**

The endpoint allows user to send refresh to token, for a new refresh and access token.

#### **Header**

Authorization: Bearer + refresh token

#### **Response**

If the token is expired the server will return **Status Code Unauthorized**.  
If not the response will be like:

```json
{
  "refresh_token": "token",
  "access_token": "token"
}
```

### 4. **GET /tasks/get**

The endpoint allows user to get all their tasks.

#### **Header**

Authorization: Bearer + refresh token

#### **Response**

If the token is expired the server will return **Status Code Unauthorized**.  
If not the response will be like:

```json
[
  {
    "id": "ffafdd8a-20ba-452f-b5b4-37d98b091ba0",
    "name": "Task name",
    "description": "Task description",
    "type": 1,
    "due_date": "2025-02-01T17:30:00+02:00",
    "date_completed": "0001-01-01T01:33:16+01:33",
    "date_deleted": "0001-01-01T01:33:16+01:33"
  }
]
```

### 5. **POST /tasks/add**

The endpoint allows user to add a new task.

#### **Header**

Authorization: Bearer + refresh token

#### **Request body**

The body should contain the task information

```json
{
  "name": "Name",
  "description": "Description",
  "type": 1,
  "due_date": "2025-02-01T17:30:00+02:00"
}
```

#### **Response**

If the token is expired the server will return **Status Code Unauthorized**.  
If not the response will be like:

```json
{
  "id": "ffafdd8a-20ba-452f-b5b4-37d98b091ba0",
  "name": "Name",
  "description": "Description",
  "type": 1,
  "due_date": "2025-02-01T17:30:00+02:00",
  "date_completed": null,
  "date_deleted": null
}
```

### 6. **PUT /tasks/update**

The endpoint allows user to update an existing token.

#### **Header**

Authorization: Bearer + refresh token

#### **Request body**

The body should contain the token information

```json
    {
  "id": "ffafdd8a-20ba-452f-b5b4-37d98b091ba0",
  "name": "Task name",
  "description": "Task description",
  "type": 1,
  "due_date": "2025-02-01T17:30:00+02:00",
  "date_completed": "0001-01-01T01:33:16+01:33",
  "date_deleted": "0001-01-01T01:33:16+01:33"
}
```

#### **Response**

If the token is expired the server will return **Status Code Unauthorized**.  
If the task is found the server will return **Status Code OK**
If the task is not found the server will return **Status Code Not Found**

### 7. **DELETE /tasks/delete/{id}**

The endpoint allows user to delete a task.

#### **Header**

Authorization: Bearer + refresh token

#### **Params**

**id** The id of the token

#### **Response**

If the token is expired the server will return **Status Code Unauthorized**.  
If the task is found the server will return **Status Code OK**
If the task is not found the server will return **Status Code Not Found**