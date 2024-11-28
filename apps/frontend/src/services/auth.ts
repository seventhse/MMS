import { z } from 'zod'
import type { Status } from '~/constants/enum'
import type { Role } from '~/constants/permission'
import { get, post } from '~/lib/request'

/**
 * Response structure for authentication operations
 */
export interface AuthResponse {
  expire: number // Token expiration time
  token: string // Authentication token
  refresh?: string
}

/**
 * Payload structure for checking username/email existence
 */
export interface CheckPayload {
  key: 'username' | 'email'
  value: string
}

/**
 * User information structure
 */
export interface UserInfo {
  userId: string // UUID in Rust, string in TypeScript
  uniqueId: string
  email: string
  username: string
  displayName?: string | null // Optional field
  avatar?: string | null // Optional field
  defaultTeamId?: string | null // UUID in Rust, string in TypeScript
  status: Status // Enum mapping to Rust's `Status`
  createdAt?: string | null // String representation of date, formatted
  updatedAt?: string | null // String representation of date, formatted
}

/**
 * Payload structure for updating user information
 */
export interface UpdateUserInfoPayload {
  email?: string | null
  username?: string | null
  avatar?: string | null
  displayName?: string | null
  defaultTeamId?: string | null
}

/**
 * Team information for a user
 */
export interface UserTeamItem {
  teamId: string
  teamName: string
  teamAvatar: string
  teamUniqueId: string
  teamNamespace: string
  description: string
  role: Role
  joinedAt: string
}

/**
 * Schema for user registration form validation
 */
export const registerFormSchema = z.object({
  email: z.string({
    required_error: 'Email is must input.',
  })
    .email('Please enter a valid email address')
    .refine(async (value) => {
      const exists = await checkEmailExists(value)
      return !exists?.data
    }, 'This email address is already registered'),
  username: z.string()
    .min(1, { message: 'Please enter a username' })
    .max(12, { message: 'Username cannot exceed 12 characters' })
    .refine(async (value) => {
      const exists = await checkUsernameExists(value)
      return !exists?.data
    }, 'This username is already taken'),
  password: z.string()
    .min(6, { message: 'Password must be at least 6 characters.' })
    .regex(/[A-Z]/, { message: 'Password must contain at least one uppercase letter.' })
    .regex(/[a-z]/, { message: 'Password must contain at least one lowercase letter.' })
    .regex(/\d/, { message: 'Password must contain at least one number.' })
    .regex(/[^A-Z0-9]/i, { message: 'Password must contain at least one special character.' }),
})

export type RegisterFormSchema = z.infer<typeof registerFormSchema>

/**
 * Schema for login form validation
 */
export const loginFormSchema = z.object({
  email: z.string({
    required_error: 'Email is required',
  })
    .email('Please enter a valid email address')
    .refine(async (value) => {
      const exists = await checkEmailExists(value)
      return exists?.data
    }, 'This email address is not registered'),
  password: z.string({
    required_error: 'Password is required',
  })
    .min(1, 'Password cannot be empty')
    .max(32, 'Password cannot exceed 32 characters'),
})

export type LoginFormSchema = z.infer<typeof loginFormSchema>

/**
 * Check if username or email exists
 * @param payload - Check payload containing key and value
 * @returns Promise with boolean result
 */
export async function check(payload: CheckPayload) {
  return await get<boolean>('/auth/check', {
    [payload.key]: payload.value,
  })
}

/**
 * Check if email exists in the system
 * @param email - Email to check
 * @returns Promise with check result
 */
export async function checkEmailExists(email: string) {
  return await check({
    key: 'email',
    value: email,
  })
}

/**
 * Check if username exists in the system
 * @param username - Username to check
 * @returns Promise with check result
 */
export async function checkUsernameExists(username: string) {
  return await check({
    key: 'username',
    value: username,
  })
}

/**
 * Get current user information
 * @returns Promise with user information
 */
export async function getUserInfo() {
  return await get<UserInfo>('/auth/info')
}

/**
 * Get user projects
 * @returns Promise with projects data
 */
export async function getTeamsByUser() {
  return await get<UserTeamItem[]>('/auth/teams')
}

/**
 * Log in a user
 * @param data - Login form data
 * @returns Promise with authentication response
 */
export async function login(data: LoginFormSchema) {
  return await post<AuthResponse>('/auth/login', data)
}

/**
 * Register a new user
 * @param data - Registration form data
 * @returns Promise with authentication response
 */
export async function register(data: RegisterFormSchema) {
  return await post<AuthResponse>('/auth/register', data)
}

/**
 * Log out the current user
 * @returns Promise with logout result
 */
export async function logout() {
  return await post('/auth/logout')
}

/**
 * Update user information
 * @param data - User information update payload
 * @returns Promise with update result
 */
export async function updateUserInfo(data: UpdateUserInfoPayload) {
  return await post('/auth/update-info', data)
}

/**
 * Reset authentication token
 * @returns Promise with new authentication response
 */
export async function resetToken() {
  return await get<AuthResponse>('/auth/reset-token')
}
