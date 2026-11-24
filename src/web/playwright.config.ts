import { defineConfig } from '@playwright/test'
import { fileURLToPath } from 'node:url'

const baseURL = 'http://127.0.0.1:18081'
const binaryName = process.platform === 'win32' ? 'template-rust-web-app.exe' : 'template-rust-web-app'
const binary = fileURLToPath(new URL('../../target/release/' + binaryName, import.meta.url))

export default defineConfig({
  testDir: './tests',
  outputDir: '../../target/browser-test-results',
  use: { baseURL, trace: 'retain-on-failure' },
  webServer: {
    command: JSON.stringify(binary),
    env: { LISTEN_ADDR: new URL(baseURL).host },
    url: baseURL + '/healthz',
  },
})
