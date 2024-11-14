'use server'
import { redirect } from 'next/navigation'
import { SignInRoute, SignOutRoute } from '~/constants/routes'
import { createSession, removeSession } from '~/lib/session'
import type { LoginFormSchema, RegisterFormSchema } from '~/services/auth'
import { login, register } from '~/services/auth'

export async function signUp(data: RegisterFormSchema) {
  const res = await register(data)
  if (res.data) {
    await createSession(res.data)
    redirect(SignInRoute)
  }
}

export async function singIn(data: LoginFormSchema) {
  const res = await login(data)
  if (res.isError) {
    throw res.error
  }
  await createSession(res.data!)
  redirect(SignInRoute)
}

export async function signOut() {
  try {
    await removeSession()
  }
  catch (e) {
    console.error('logout error: ', e)
  }
  redirect(SignOutRoute)
}
