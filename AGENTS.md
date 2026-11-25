# Agent Instructions

Read [README.md](README.md) for project setup and commands.

## What we value

- Build the simplest solution that works. Prefer clear names, small functions, and straightforward control flow.
- Keep changes focused and reversible. Add abstractions when the code has a concrete need for them.

## Conventions and boundaries

- Ask before adding external dependencies or changing database migrations.
- Put detailed workflows in skills or nested `AGENTS.md` files.
- Branch from `main` using `feature/<name>` for features and `bug/<name>` for fixes. Open pull requests against `main`.
- Use **What**, **Why**, **Decisions**, **Testing**, and **Risks** as pull request headings.
- Keep the application core in `src/core/` independent of frameworks, storage implementations, and delivery layers. Put HTTP, GraphQL, and storage adapters in `src/server/`.
- Use `hotfix/<name>` for urgent release fixes and tag releases on `main`.

## Verification and handoff

- For a behavior change, start with a failing test, make it pass with the smallest change, and then refactor.
- Run `make verify` for code changes. It includes tests, the dependency audit, and a two-browser smoke test for live list updates. See the README for scanner and browser setup.
- Exercise the real entry point after meaningful behavior changes and before opening or updating the pull request. Check the output and relevant side effects.
- Mock external systems at their boundaries. Confirm what test or dry-run modes skip, preserve the coverage of failing checks, and report any verification gaps.
- Before opening a pull request, get an independent review of correctness and security when a fresh reviewer is available.
