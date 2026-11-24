import { defineConfig } from '@playwright/test'
import { resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const baseURL = 'http://127.0.0.1:18081'
const binary = process.platform === 'win32' ? 'template-rust-web-app.exe' : './template-rust-web-app'
const workspace = fileURLToPath(new URL('../../', import.meta.url))
const target = resolve(workspace, process.env.CARGO_TARGET_DIR || 'target')

export default defineConfig({
  testDir: './tests',
  outputDir: resolve(target, 'browser-test-results'),
  use: { baseURL, trace: 'retain-on-failure' },
  webServer: {
    command: binary,
    cwd: resolve(target, 'release'),
    env: { LISTEN_ADDR: new URL(baseURL).host },
    url: baseURL + '/healthz',
  },
})
