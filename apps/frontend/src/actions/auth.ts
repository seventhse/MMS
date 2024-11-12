'use server'

import { redirect } from 'next/navigation'
import type { LoginFormSchema, RegisterFormSchema } from '~/api/auth'
import { login, register } from '~/api/auth'
import { Routes } from '~/constants/routes'
import { createSession, removeSession } from '~/lib/session'

export async function signUp(data: RegisterFormSchema) {
  const res = await register(data)
  if (res.data) {
    await createSession(res.data)
    redirect(Routes.DASHBOARD)
  }
}

export async function singIn(data: LoginFormSchema) {
  const res = await login(data)
  if (res.isError) {
    throw res.error
  }
  await createSession(res.data!)
  redirect(Routes.DASHBOARD)
}

export async function signOut() {
  await removeSession()
  redirect(Routes.LOGIN)
}
