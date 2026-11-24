# Working in this template

- Keep dependencies pointing toward the application core. Put GraphQL, HTTP, and the in-memory adapter in the server crate.
- Start each behavior change with a failing test. Make the smallest change that passes, then refactor with the tests green.
- Keep the sample to one page. Use Radix Themes components and spacing props for layout and styling.
- Run `make test-rust` for a fast check and `make verify` for all checks and the two-browser test. See the README for Chromium setup.
