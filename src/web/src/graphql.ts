import { ApolloClient, ApolloLink, HttpLink, InMemoryCache, gql, type TypedDocumentNode } from '@apollo/client'
import { GraphQLWsLink } from '@apollo/client/link/subscriptions'
import { OperationTypeNode } from 'graphql'
import { createClient } from 'graphql-ws'

const websocketUrl = new URL('/graphql/ws', window.location.href)
websocketUrl.protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:'

const http = new HttpLink({ uri: '/graphql' })
const websocket = new GraphQLWsLink(createClient({ url: websocketUrl.href }))

export const apolloClient = new ApolloClient({
  cache: new InMemoryCache(),
  link: ApolloLink.split(
    ({ operationType }) => operationType === OperationTypeNode.SUBSCRIPTION,
    websocket,
    http,
  ),
})

export const STRINGS: TypedDocumentNode<{ strings: string[] }> = gql`
  query Strings {
    strings
  }
`

export const ADD_RANDOM_STRING: TypedDocumentNode<{ addRandomString: string }> = gql`
  mutation AddRandomString {
    addRandomString
  }
`

export const STRINGS_CHANGED: TypedDocumentNode<{ stringsChanged: string[] }> = gql`
  subscription StringsChanged {
    stringsChanged
  }
`
