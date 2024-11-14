import { cookies } from 'next/headers'
import { getUserInfo, resetToken } from '~/services/auth'
import type { AuthResponse, UserInfo } from '~/services/auth'
import 'server-only'

const SESSION_KEY = 'SESSION_KEY'
const SESSION_CASE_KEY = 'SESSION_CASE_KEY'
const SESSION_INFO_KEY = 'SESSION_INFO_KEY'

export async function createInfoSession() {
  const { isError, data } = await getUserInfo()

  if (isError) {
    return
  }

  const cookie = await cookies()

  cookie.set(SESSION_INFO_KEY, JSON.stringify(data), {
    httpOnly: true,
    secure: true,
    sameSite: 'lax',
    path: '/',
  })
}

export async function getInfoSession(): Promise<UserInfo | null> {
  const cookie = await cookies()

  const info = cookie.get(SESSION_INFO_KEY)

  if (!info?.value) {
    return null
  }

  return JSON.parse(info.value)
}

export async function createSession(payload: AuthResponse) {
  const cookie = await cookies()

  cookie.set(SESSION_KEY, payload.token, {
    httpOnly: true,
    secure: true,
    expires: new Date(Date.now() + payload.expire - 60 * 1000),
    sameSite: 'lax',
    path: '/',
  })
  cookie.set(SESSION_CASE_KEY, payload.token, {
    httpOnly: true,
    secure: true,
    expires: new Date(Date.now() + payload.expire),
    sameSite: 'lax',
    path: '/',
  })

  await createInfoSession()
}

async function getToken() {
  const cookie = await cookies()
  const sessionCookie = cookie.get(SESSION_CASE_KEY)
  return sessionCookie?.value || null
}

export async function getSession() {
  const cookie = await cookies()
  const sessionCookie = cookie.get(SESSION_KEY)

  if (sessionCookie?.value) {
    return sessionCookie.value
  }

  const token = await getToken()

  if (token) {
    const { isError, data } = await resetToken()
    if (!isError) {
      createSession(data!)
    }
    return token
  }

  return null
}

export async function removeSession() {
  const cookie = await cookies()
  cookie.delete(SESSION_KEY)
  cookie.delete(SESSION_CASE_KEY)
  cookie.delete(SESSION_INFO_KEY)
}
