import { z } from 'zod'
import type { Status } from '~/constants/enum'
import type { Role } from '~/constants/permission'
import { Roles } from '~/constants/permission'
import { get, post } from '~/lib/request'

export interface TeamItem {
  teamId: string // UUID
  teamUniqueId: string
  teamName: string
  teamAvatar?: string // Optional
  teamNamespace: string
  description?: string // Optional
  createdAt?: string // Optional, formatted as 'YYYY-MM-DD HH:mm:ss'
  updatedAt?: string // Optional, formatted as 'YYYY-MM-DD HH:mm:ss'
}

/**
 * User information for a team
 */
export interface TeamUserItem {
  userId: string
  username: string
  displayName?: string | null
  email: string
  avatar?: string | null
  role: Role
  status: Status
  joinedAt: string
  leftedAt?: string | null
}

//
// Schemas for API Validation
//

// Create Team Schema
export const createTeamSchema = z.object({
  teamName: z.string().min(1, 'Team name is required.'),
  teamNamespace: z
    .string()
    .min(1, 'Namespace is required.')
    .regex(/^[A-Z0-9]+$/i, 'Only alphanumeric characters are allowed.'),
  teamAvatar: z.string().optional(),
  description: z.string().optional(),
})
export type CreateTeamSchema = z.infer<typeof createTeamSchema>

// Leave Team Schema
export const leaveTeamSchema = z.object({
  teamId: z.string().uuid('Invalid team ID.'),
  userId: z.string().uuid('Invalid user ID.'),
})
export type LeaveTeamSchema = z.infer<typeof leaveTeamSchema>

// Join Team Schema
export const joinTeamSchema = z.object({
  teamId: z.string().uuid('Invalid team ID.'),
  userId: z.string().uuid('Invalid user ID.'),
  role: z.enum(Roles),
})
export type JoinTeamSchema = z.infer<typeof joinTeamSchema>
//
// API Requests
//

/**
 * Get All Team List
 * Fetch all teams in the system.
 */
export async function getTeamList() {
  return await get<TeamItem[]>('/team/list')
}

/**
 * Get Team Detail
 * Fetch detailed information for a specific team.
 * @param teamId The unique ID of the team.
 */
export async function getTeamDetail(teamId: string) {
  return await get<TeamItem>(`/team/detail/${teamId}`)
}

/**
 * Get All Users by Team
 * Fetch all users associated with a specific team.
 * @param teamId The unique ID of the team.
 */
export async function getTeamUsers(teamId: string) {
  return await get<TeamUserItem>(`/team/users/${teamId}`)
}

/**
 * Check Namespace Existence
 * Check if a namespace already exists.
 * @param namespace The namespace to check.
 */
export async function checkNamespaceByTeam(namespace: string) {
  return await get<boolean>(`/team/check/namespace/${namespace}`)
}

/**
 * Create Team
 * Create a new team with the provided details.
 * @param data The data for creating the team.
 */
export async function createTeam(data: CreateTeamSchema) {
  return await post('/team/create', data)
}

/**
 * Leave Team
 * Leave a specific team as a user.
 * @param data The data for leaving the team.
 */
export async function leaveTeam(data: LeaveTeamSchema) {
  return await post('/team/left-team', data)
}

/**
 * Join Team
 * Join a specific team with a given role.
 * @param data The data for joining the team.
 */
export async function joinTeam(data: JoinTeamSchema) {
  return await post('/team/join-team', data)
}

/**
 * Update Team Info
 * Update the information of an existing team.
 * @param teamId The unique ID of the team.
 * @param data The updated team details.
 */
export async function updateTeamInfo(teamId: string, data: Partial<CreateTeamSchema>) {
  return await post(`/team/update/${teamId}`, data)
}

/**
 * Delete Team
 * Delete a specific team by its ID.
 * @param teamId The unique ID of the team.
 */
export async function deleteTeam(teamId: string) {
  return await post(`/team/delete/${teamId}`)
}
