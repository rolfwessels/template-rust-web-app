import { useMutation, useQuery, useSubscription } from '@apollo/client/react'
import { Badge, Button, Callout, Card, Code, Container, Flex, Heading, Separator, Spinner, Text } from '@radix-ui/themes'
import { ADD_RANDOM_STRING, STRINGS, STRINGS_CHANGED } from './graphql'

const version = import.meta.env.VITE_APP_VERSION

export function App() {
  const query = useQuery(STRINGS)
  const subscription = useSubscription(STRINGS_CHANGED, { skip: !query.data })
  const [addRandomString, mutation] = useMutation(ADD_RANDOM_STRING)

  const strings = subscription.data?.stringsChanged ?? query.data?.strings
  const error = query.error ?? subscription.error ?? mutation.error
  const count = strings?.length ?? 0

  return (
    <Container asChild size="2" px={{ initial: '4', sm: '6' }} py={{ initial: '6', sm: '9' }}>
      <main>
        <Card asChild size={{ initial: '3', sm: '4' }} variant="classic">
          <section aria-labelledby="list-title">
            <Flex direction="column" gap="4">
              <Flex asChild direction="column" gap="2">
                <header>
                  <Flex align="center" justify="between" gap="3" wrap="wrap">
                    <Heading as="h1" id="list-title" size="6">Shared strings</Heading>
                    <Badge color="gray" variant="soft">
                      {version && version !== 'dev' ? `v${version}` : 'Development'}
                    </Badge>
                  </Flex>
                  <Text as="p" size="2" color="gray">New strings appear in every connected tab.</Text>
                </header>
              </Flex>

              <Separator size="4" />

              {query.loading && (
                <Flex role="status" gap="2" align="center">
                  <Spinner />
                  <Text>Loading strings…</Text>
                </Flex>
              )}

              {error && (
                <Callout.Root role="alert" color="red">
                  <Callout.Text>{error.message}</Callout.Text>
                </Callout.Root>
              )}

              {strings && (
                <Flex asChild direction="column" gap="2" m="0" p="0">
                  <ul aria-label="Shared strings" aria-live="polite">
                    {strings.map((value, index) => (
                      <Card asChild key={index} size="1" variant="surface">
                        <li><Code size="3" variant="ghost">{value}</Code></li>
                      </Card>
                    ))}
                  </ul>
                </Flex>
              )}
              {strings && count === 0 && <Text color="gray">The list is empty. Add a string to get started.</Text>}

              {query.data && !subscription.data && !subscription.error && (
                <Text role="status" size="2" color="gray">Connecting live updates…</Text>
              )}

              <Separator size="4" />

              <Flex asChild align="center" justify="between" gap="3" wrap="wrap">
                <footer>
                  <Button
                    disabled={!subscription.data}
                    loading={mutation.loading}
                    onClick={() => void addRandomString().catch(() => undefined)}
                  >
                    Add random string
                  </Button>
                  {strings && <Text size="2" color="gray">{count} {count === 1 ? 'item' : 'items'}</Text>}
                </footer>
              </Flex>
            </Flex>
          </section>
        </Card>
      </main>
    </Container>
  )
}
