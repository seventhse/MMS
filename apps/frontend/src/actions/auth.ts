'use server'

import { redirect } from 'next/navigation'
import { SignInRoute, SignOutRoute } from '~/constants/routes'
import type { AuthResponse, LoginFormSchema, RegisterFormSchema } from '~/services/auth'
import { getTeamsByUser, getUserInfo, login, register } from '~/services/auth'
import { clearSession, setToken, setUserInfo, setUserTeams } from './cache'

export async function refreshUserInfoAction() {
  const res = await getUserInfo()

  if (res.isError) {
    throw res.error
  }

  await setUserInfo(res.data!)
}

export async function refreshUserTeamsAction() {
  const res = await getTeamsByUser()

  if (res.isError) {
    throw res.error
  }

  await setUserTeams(res.data!)
}

export async function setAuthInfoAction(data: AuthResponse) {
  await setToken(data!)
  await refreshUserTeamsAction()
  await refreshUserInfoAction()
  redirect(SignInRoute)
}

export async function signUpAction(data: RegisterFormSchema) {
  const res = await register(data)
  if (res.isError) {
    throw res.error
  }
  await setAuthInfoAction(res.data!)
}

export async function singInAction(data: LoginFormSchema) {
  const res = await login(data)

  if (res.isError) {
    throw res.error
  }
  await setAuthInfoAction(res.data!)
}

export async function signOutAction() {
  try {
    await clearSession()
  }
  catch (e) {
    console.error('logout error: ', e)
  }
  redirect(SignOutRoute)
}
