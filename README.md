# Template Rust Web App

A small Rust and React web app template. Two browser sessions share one in-memory list. Add a random string in either session and both update live.

## Features

- Axum and async-graphql provide a query, mutation, and WebSocket subscription.
- A framework-independent Rust application core defines the list use cases and storage port.
- React, Radix Themes, and Apollo Client render the list and receive live updates.
- One Rust release binary serves the API and the embedded frontend.

## Getting started

Use Rust 1.100 or later, Node.js 24, and pnpm 10.34.7. The Makefile can use Corepack when pnpm is available through it. Install Docker to use the provided development or release containers.

```sh
make start
```

Open [localhost:8080](http://127.0.0.1:8080) in two tabs. Click **Add random string** in one tab and watch the new string appear in both. The server starts with two sample strings.

To work in Docker, run `make build up`, then run `make start` from the dev shell. Use `make down` to stop the dev container.

## Make commands

| Command | Purpose |
| --- | --- |
| `make version` | Print the Git-derived version used for binaries and Docker tags. |
| `make install` | Install frontend dependencies from the pnpm lockfile. |
| `make start` | Build the frontend and start the local server. |
| `make test-rust` | Run the fast application and GraphQL tests. |
| `make test` | Run Rust and TypeScript checks and the Rust tests. |
| `make build-release` | Build the frontend, then embed it in the Rust release binary. |
| `make verify` | Run checks and the two-browser smoke test. |
| `make docker-build` | Build and tag the runtime image. |
| `make docker-run` | Run the runtime image locally with Compose. |

## Folder structure

- `src/core/` holds the list port and use cases.
- `src/server/` holds the GraphQL, HTTP, and in-memory adapters.
- `src/web/` holds the React app and browser smoke test. Vite writes the production bundle to `src/web/dist/`.
- `AGENTS.md` describes the dependency direction, test-first workflow, and verification commands.

## Testing

Use `make test-rust` for a fast feedback loop. Install Chromium once for the browser test and run the full verification.

```sh
make install
(cd src/web && corepack pnpm exec playwright install chromium)
make verify
```

On a Linux machine that needs Chromium system packages, add `--with-deps` to the Chromium install command. Use `pnpm` in place of `corepack pnpm` if you installed it directly.

## Deployment

The root Dockerfile has separate targets for development and production. `make docker-run` uses the non-root runtime stage and forwards port 8080 to localhost.

Run `make build-release` to produce `target/release/template-rust-web-app`. Set `LISTEN_ADDR` to override the default `127.0.0.1:8080`. Set `RUST_LOG` to control structured log output.

- `GET /healthz` reports server health.
- `POST /graphql` handles queries and mutations.
- `/graphql/ws` accepts subscriptions using `graphql-transport-ws`.

## License

MIT.
