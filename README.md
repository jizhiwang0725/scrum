# Recipe Finder

This is a full-stack web application

## Tech Stack

**Frontend:**

- **Core Framework:** React 19 + TypeScript
- **Build Tool:** Vite (Lightning-fast cold starts and HMR)
- **Styling:** Tailwind CSS v4 (Utility-first CSS)
- **Component Library:** shadcn/ui (Accessible, highly customizable UI foundation)

**Backend:**

- **Programming Language**: Rust

## Project Structure

- `/frontend`: *(Work in progress)* Contains the user interface and client-side logic
- `/backend`: *(Work in progress)* Will contain the API services and database logic

## Getting Started :rocket:

Currently, the repository contains the initial frontend setup. Follow these steps to run the fronted application locally:

:warning: Please ensure you have [Node.js](https://nodejs.org/) installed on your local machine before proceeding

### 1. Navigate to the frontend directory

From the project root directory, enter the frontend directory, where `package.json` is located.

```bash
cd frontend
```

### 2. Install Dependencies

Run the following command to install all required package:

```bash
npm install
```

### 3. Start the Development Server

To spin up the local development environment, run:

```bash
npm run dev
```

Once the server is running, open your browser and navigate to the local URL displayed in your terminal (typically`http://localhost:5173`).

### 4. Start the Backend

Keep the frontend terminal running. Open another terminal at the repository root.

#### Install Rust and Cargo

If Rust is not installed, use the following command on macOS or Linux:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the installer prompts and choose the default installation.
Restart your terminal afterwards, then check:

```bash
rustc --version
cargo --version
```

If macOS reports a missing compiler or linker, run:

```bash
xcode-select --install
```

For Windows and other installation details, see the
[official Rust installation guide](https://doc.rust-lang.org/book/ch01-01-installation.html).

#### Run the Backend

From the repository root:

```bash
cargo run --manifest-path backend/Cargo.toml
```

Cargo downloads the dependencies and builds the backend.
The first run may take longer.

The default backend address is `http://127.0.0.1:3000`.
Keep this terminal running. Press Control+C to stop the server.

### 5. Check Frontend and Backend Communication

With both services running, open the browser developer console on the
frontend page and run:

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

Expected: status `200` and body `{status: 'ok'}`.

Run this on the frontend page to check browser cross-origin access.
This checks health-endpoint communication; the complete recipe-search
flow still needs the recipe API to be connected.

## Backend Configuration

Run these commands from the repository root.
Stop the backend before restarting it with different settings.
The environment-variable examples use macOS or Linux shell syntax.

### Backend Address

To use another backend port:

```bash
BIND_ADDR="127.0.0.1:3001" cargo run --manifest-path backend/Cargo.toml
```

Update the fetch URL when using a different backend port.

### Frontend Origins and CORS

The default allowed frontend origins are:

- `http://localhost:5173`
- `http://127.0.0.1:5173`

If Vite uses port 5174, restart the backend with:

```bash
FRONTEND_ORIGINS="http://localhost:5174,http://127.0.0.1:5174" cargo run --manifest-path backend/Cargo.toml
```

This replaces the default list. Separate multiple origins with commas.
Use the actual frontend origin, without a path or trailing slash.
Explicit ports must be between 1 and 65535.
Invalid configuration prevents startup.

CORS allows `GET`, `HEAD`, `OPTIONS`, and the `Accept` request header.
Credentials are not enabled.
Preflight responses return `204` with no body, as required by
[docs/API.md](docs/API.md).

Origins outside the list receive no `Access-Control-Allow-Origin` header.
The browser blocks those pages from reading the response.
CORS does not block non-browser clients such as curl.

### Request Logs

Logs show the request method, path, status code, and response time.
The request logs do not include query parameters, headers, or bodies.

To explicitly show INFO-level logs:

```bash
RUST_LOG=info cargo run --manifest-path backend/Cargo.toml
```

## Backend Checks

Keep the backend running and use another terminal.
These examples use the default backend port, 3000.

### Health Endpoint

```bash
curl -i http://127.0.0.1:3000/api/health
```

Expected: `200 OK` and `{"status":"ok"}`.

### CORS Preflight

```bash
curl -i -X OPTIONS http://127.0.0.1:3000/api/health \
  -H 'Origin: http://localhost:5173' \
  -H 'Access-Control-Request-Method: GET' \
  -H 'Access-Control-Request-Headers: accept'
```

Expected: `204 No Content`, an empty body, and
`Access-Control-Allow-Origin: http://localhost:5173`.

### Origin Outside the Allowed List

```bash
curl -i http://127.0.0.1:3000/api/health \
  -H 'Origin: http://localhost:5999'
```

Expected: no `Access-Control-Allow-Origin` header.
A `200` response to curl is normal.

### Invalid Origin Configuration

In an idle terminal, run each command separately:

```bash
FRONTEND_ORIGINS="http://localhost:99999" cargo run --manifest-path backend/Cargo.toml
```

```bash
FRONTEND_ORIGINS="http://999.999.999.999:5173" cargo run --manifest-path backend/Cargo.toml
```

Both should report `InvalidInput` and exit before opening a listening port.
These environment-variable settings apply only to their respective commands.

## :world_map: Route Map

View the complete and up-to-date visual flow diagram here: :point_right: **[View Route Map Diagram](https://docs.google.com/drawings/d/1-igfDToXf8JBYtcTK64aQ7PzHcB-HuxCFKBrXh2Zr28/edit)**

