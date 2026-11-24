import { ApolloProvider } from '@apollo/client/react'
import { Theme } from '@radix-ui/themes'
import '@radix-ui/themes/styles.css'
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { App } from './App'
import { apolloClient } from './graphql'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <Theme radius="small">
      <ApolloProvider client={apolloClient}>
        <App />
      </ApolloProvider>
    </Theme>
  </StrictMode>,
)
