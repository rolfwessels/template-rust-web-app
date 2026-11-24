import { useMutation, useQuery, useSubscription } from '@apollo/client/react'
import { Button, Callout, Card, Container, Flex, Heading, Spinner, Text } from '@radix-ui/themes'
import { ADD_RANDOM_STRING, STRINGS, STRINGS_CHANGED } from './graphql'

export function App() {
  const query = useQuery(STRINGS)
  const subscription = useSubscription(STRINGS_CHANGED, { skip: !query.data })
  const [addRandomString, mutation] = useMutation(ADD_RANDOM_STRING)

  const strings = subscription.data?.stringsChanged ?? query.data?.strings
  const error = query.error ?? subscription.error ?? mutation.error

  return (
    <Container size="2" p="6">
      <Card size="3">
        <Flex direction="column" gap="4">
          <Heading as="h1">Shared strings</Heading>

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
            <ul aria-label="Shared strings">
              {strings.map((value, index) => <li key={index}>{value}</li>)}
            </ul>
          )}

          <Flex direction="column" align="start" gap="2">
            <Button
              disabled={!subscription.data}
              loading={mutation.loading}
              onClick={() => void addRandomString().catch(() => undefined)}
            >
              Add random string
            </Button>
            {query.data && !subscription.data && !subscription.error && (
              <Text role="status" size="2" color="gray">Connecting live updates…</Text>
            )}
          </Flex>
        </Flex>
      </Card>
    </Container>
  )
}
