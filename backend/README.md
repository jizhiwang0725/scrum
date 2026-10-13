# Backend startup and checks

## Start the backend

Install Rust with Cargo first.

From the repository root:

```bash
cd backend
cargo run
```

The default backend address is `http://127.0.0.1:3000`.

Keep the terminal running. Press Control+C to stop the server.

To use another port:

```bash
BIND_ADDR=127.0.0.1:3001 cargo run
```

## Frontend addresses and CORS

These frontend origins are allowed by default:

- `http://localhost:5173`
- `http://127.0.0.1:5173`

To use a different frontend port, restart the backend with:

```bash
FRONTEND_ORIGINS=http://localhost:5174 cargo run
```

Multiple origins can be separated by commas. Use the actual frontend
origin, without a path or trailing slash. This setting replaces the
default list. Invalid origin configuration prevents startup.

CORS allows GET, HEAD, OPTIONS and the Accept request header.
Credentials are not enabled. Preflight responses return 204 with no body.

An origin outside the list receives no Access-Control-Allow-Origin header.
The browser then blocks that page from reading the response.
CORS does not block non-browser clients such as curl.

## Request logs

Logs show the request method, path, status code and response time.
Query parameters, headers and bodies are not included in the default logs.

To show INFO-level logs explicitly:

```bash
RUST_LOG=info cargo run
```

## Check the health endpoint

In another terminal:

```bash
curl -i http://127.0.0.1:3000/api/health
```

Expected: 200 OK and `{"status":"ok"}`.

## Check CORS preflight

```bash
curl -i -X OPTIONS http://127.0.0.1:3000/api/health \
  -H 'Origin: http://localhost:5173' \
  -H 'Access-Control-Request-Method: GET' \
  -H 'Access-Control-Request-Headers: accept'
```

Expected: 204 No Content, an empty body, and an
Access-Control-Allow-Origin header matching `http://localhost:5173`.

## Check an origin outside the list

```bash
curl -i http://127.0.0.1:3000/api/health \
  -H 'Origin: http://localhost:5999'
```

Expected: no Access-Control-Allow-Origin header.
A 200 response to curl is normal.

## Check from the frontend page

Install Node.js and npm first. In another terminal, from the repository root:

```bash
cd frontend
npm install
npm run dev
```

Open the frontend URL printed by Vite. Check that its origin is allowed.

In that frontend page's browser console, run:

```javascript
fetch('http://127.0.0.1:3000/api/health', {
  headers: { Accept: 'application/json' },
})
  .then(async response => {
    console.log('status:', response.status);
    console.log('body:', await response.json());
  })
  .catch(console.error);
```

Expected: status 200 and body `{status: 'ok'}`.

Run this from the frontend page, not the backend health page,
to check cross-origin access.

## Current scope

The backend currently has health and temporary debug endpoints.
The formal `/api/recipes` endpoint is separate work.
This browser check verifies communication with the health endpoint,
not the complete recipe-search flow.

For the full interface agreement, see [API.md](../docs/API.md).