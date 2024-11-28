'use server'

import { redirect, RedirectType } from 'next/navigation'
import { SignInRoute, SignOutRoute } from '~/constants/routes'
import type { AuthResponse } from '~/services/auth'
import { getTeamsByUser, getUserInfo } from '~/services/auth'
import { clearSession, setToken, setUserInfo, setUserTeams } from './cache'

export async function refreshUserInfoAction() {
  const res = await getUserInfo()

  if (res.isError) {
    return res
  }

  await setUserInfo(res.data!)
}

export async function refreshUserTeamsAction() {
  const res = await getTeamsByUser()

  if (res.isError) {
    return res
  }

  await setUserTeams(res.data!)
}

export async function setAuthInfoAction(data: AuthResponse) {
  await setToken(data!)
  await refreshUserTeamsAction()
  await refreshUserInfoAction()
  redirect(SignInRoute, RedirectType.replace)
}

export async function signOutAction() {
  try {
    await clearSession()
    return
  }
  catch (e) {
    console.error('logout error: ', e)
  }
  redirect(SignOutRoute, RedirectType.replace)
}
