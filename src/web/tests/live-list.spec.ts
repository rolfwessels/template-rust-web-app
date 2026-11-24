import { expect, test, type Page, type Response } from '@playwright/test'

const seed = ['river-spark', 'paper-kite']

function isInitialQuery(response: Response) {
  const request = response.request()
  return new URL(response.url()).pathname === '/graphql'
    && request.method() === 'POST'
    && request.postDataJSON()?.operationName === 'Strings'
}

async function waitForInitialSubscription(page: Page) {
  const socket = await page.waitForEvent('websocket', candidate =>
    new URL(candidate.url()).pathname === '/graphql/ws',
  )
  await socket.waitForEvent('framereceived', ({ payload }) =>
    typeof payload === 'string' && JSON.parse(payload).type === 'next',
  )
}

test('an addition appears in both browser sessions without a reload', async ({ browser }) => {
  const first = await browser.newPage()
  const second = await browser.newPage()

  try {
    const [firstQuery, secondQuery] = await Promise.all([
      first.waitForResponse(isInitialQuery),
      second.waitForResponse(isInitialQuery),
      waitForInitialSubscription(first),
      waitForInitialSubscription(second),
      first.goto('/'),
      second.goto('/'),
    ])
    for (const query of [firstQuery, secondQuery]) {
      expect(await query.json()).toEqual({ data: { strings: seed } })
    }

    const firstItems = first.getByRole('list', { name: 'Shared strings' }).getByRole('listitem')
    const secondItems = second.getByRole('list', { name: 'Shared strings' }).getByRole('listitem')
    await expect(firstItems).toHaveText(seed)
    await expect(secondItems).toHaveText(seed)
    await expect(first.getByText(/^v\d+\.\d+\.\d+/)).toBeVisible()
    await expect(first.getByText('2 items', { exact: true })).toBeVisible()
    await expect(second.getByText('2 items', { exact: true })).toBeVisible()

    let navigations = 0
    for (const page of [first, second]) {
      page.on('framenavigated', frame => {
        if (frame === page.mainFrame()) navigations += 1
      })
    }

    await first.getByRole('button', { name: 'Add random string' }).click()
    await expect(firstItems).toHaveCount(seed.length + 1)
    const added = await firstItems.last().innerText()
    expect(added).toMatch(/^[a-z0-9]{10}$/i)
    await expect(secondItems).toHaveText([...seed, added])
    await expect(first.getByText('3 items', { exact: true })).toBeVisible()
    await expect(second.getByText('3 items', { exact: true })).toBeVisible()
    expect(navigations).toBe(0)
  } finally {
    await Promise.all([first.close(), second.close()])
  }
})
