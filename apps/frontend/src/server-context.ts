import { cache } from 'react'
import 'server-only'

const requestContext = cache(() => {
  return new Map<string, string>()
})

export function setRequestContext(key: string, value: string) {
  return requestContext().set(key, value)
}

export const getRequestContext = (key: string) => requestContext().get(key)
