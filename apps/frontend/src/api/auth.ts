import { z } from 'zod'
import { get, post } from '~/lib/request'

export const registerFormSchema = z.object({
  email: z.string({
    required_error: 'Email is must input.',
  })
    .email('Please input a valid email!')
    .refine(async (val) => {
      const res = await checkEmailExists(val)
      return !res.data
    }, {
      message: 'The email already exist.',
    }),
  username: z.string()
    .min(1, { message: 'Username is required.' })
    .max(12, { message: 'Username must be at most 12 characters.' })
    .refine(async (val) => {
      const res = await checkUsernameExists(val)
      return !res.data
    }, {
      message: 'The username already exist.',
    }),
  password: z.string()
    .min(1, { message: 'Password is required.' })
    .min(6, { message: 'Password must be at least 6 characters.' })
    .regex(/[A-Z]/, { message: 'Password must contain at least one uppercase letter.' })
    .regex(/[a-z]/, { message: 'Password must contain at least one lowercase letter.' })
    .regex(/\d/, { message: 'Password must contain at least one number.' })
    .regex(/[^A-Z0-9]/i, { message: 'Password must contain at least one special character.' }),
})

export type RegisterFormSchema = z.infer<typeof registerFormSchema>

export const loginFormSchema = z.object({
  email: z.string().email('Please enter a valid email address').refine(async (val) => {
    const res = await checkEmailExists(val)
    return res.data
  }, {
    message: 'This email address does not exist in our system',
  }),
  password: z.string({
    required_error: 'Please enter your password',
  }),
})

export type LoginFormSchema = z.infer<typeof loginFormSchema>

export interface AuthResponse {
  expire: number
  token: string
}

export interface CheckPayload {
  key: 'username' | 'email'
  value: string
}

export async function register(data: RegisterFormSchema) {
  return await post<AuthResponse>('/auth/register', data)
}

export async function check(payload: CheckPayload) {
  return await get<boolean>('/auth/check', {
    [payload.key]: payload.value,
  })
}

export async function checkEmailExists(email: string) {
  return await check({
    key: 'email',
    value: email,
  })
}

export async function checkUsernameExists(username: string) {
  return await check({
    key: 'username',
    value: username,
  })
}

export async function login(data: LoginFormSchema) {
  return await post<AuthResponse>('/auth/login', data)
}
