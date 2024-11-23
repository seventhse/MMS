import type { ResponseCookie } from 'next/dist/compiled/@edge-runtime/cookies'
import { cookies } from 'next/headers'
import { REFRESH_TOKEN_SESSION_KEY, SELECTED_TEAM_SESSION_KEY, TOKEN_SESSION_KEY, USER_INFO_SESSION_KEY, USER_TEAMS_SESSION_KEY } from '~/constants/session'
import type { AuthResponse, UserInfo, UserTeamItem } from '~/services/auth'
import 'server-only'

export const DEFAULT_SESSION_CONFIG: Partial<ResponseCookie> = {
  httpOnly: true,
  secure: true,
  sameSite: 'lax',
  path: '/',
}

export async function setSession(key: string, data: unknown, options?: Partial<ResponseCookie>) {
  if (!key || typeof key !== 'string') {
    throw new Error('Invalid session key')
  }
  const cookie = await cookies()
  cookie.set(key, JSON.stringify(data), {
    ...DEFAULT_SESSION_CONFIG,
    ...(options || {}),
  })
}

export async function getSession<T>(key: string): Promise<T | null> {
  if (!key || typeof key !== 'string') {
    throw new Error('Invalid session key')
  }
  const cookie = await cookies()
  if (!cookie.has(key)) {
    return null
  }

  const data = cookie.get(key)

  try {
    return JSON.parse(data?.value || '') as T
  }
  catch {
    return data?.value as T
  }
}

export async function removeSession(key: string) {
  if (!key || typeof key !== 'string') {
    throw new Error('Invalid session key')
  }
  const cookie = await cookies()
  if (cookie.has(key)) {
    cookie.delete(key)
  }
}

export async function clearSession() {
  const cache = await cookies()
  if (!cache) {
    throw new Error('Failed to access cookies')
  }
  for (const cookie of cache.getAll()) {
    cache.delete(cookie.name)
  }
}

export async function setUserInfo(data: UserInfo) {
  await setSession(USER_INFO_SESSION_KEY, data)
}
export async function getUserInfo() {
  return await getSession<UserInfo>(USER_INFO_SESSION_KEY)
}

export async function setToken(data: AuthResponse) {
  const expire = data.expire
  await setSession(TOKEN_SESSION_KEY, data.token, {
    expires: new Date(Date.now() + expire - 60 * 1000),
  })
  await setSession(REFRESH_TOKEN_SESSION_KEY, data.refresh || data.token, {
    expires: new Date(Date.now() + expire),
  })
}
export async function getToken() {
  const token = await getSession<string>(TOKEN_SESSION_KEY)
  if (token)
    return token

  return await getSession<string>(REFRESH_TOKEN_SESSION_KEY)
}

export async function setTeamInfo(data: UserTeamItem) {
  await setSession(SELECTED_TEAM_SESSION_KEY, data)
}
export async function getTeamInfo() {
  return await getSession<UserTeamItem>(SELECTED_TEAM_SESSION_KEY)
}

export async function setUserTeams(data: UserTeamItem[]) {
  await setSession(USER_TEAMS_SESSION_KEY, data)
}

export async function getUserTeams() {
  return await getSession<UserTeamItem[]>(USER_TEAMS_SESSION_KEY)
}
