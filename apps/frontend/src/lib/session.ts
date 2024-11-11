import { cookies } from 'next/headers'
import type { AuthResponse } from '~/api/auth'
import 'server-only'

const SESSION_KEY = 'SESSION_KEY'

export async function createSession(payload: AuthResponse) {
  const cookie = await cookies()

  cookie.set(SESSION_KEY, payload.token, {
    httpOnly: true,
    secure: true,
    expires: new Date(Date.now() + payload.expire),
    sameSite: 'lax',
    path: '/',
  })
}

export async function getSession() {
  const cookie = await cookies()
  const sessionCookie = cookie.get(SESSION_KEY)

  return sessionCookie?.value || null
}

export async function removeSession() {
  const cookie = await cookies()
  cookie.delete(SESSION_KEY)
}
