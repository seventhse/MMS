'use server'

import { redirect } from 'next/navigation'
import { Routes } from '~/constants/routes'
import type { CreateTeamSchema } from '~/services/team'
import { createTeam } from '~/services/team'
import { refreshUserTeamsAction } from './auth'
import { getUserTeams, setTeamInfo } from './cache'

export async function refreshTeamInfoAction(teamId?: string) {
  const teams = await getUserTeams()
  if (!teams?.length) {
    redirect(Routes.TEAM)
  }

  const teamItem = teamId ? teams.find(item => item.teamId === teamId) : teams[0]

  if (!teamItem) {
    redirect(Routes.TEAM)
  }

  setTeamInfo(teamItem)
  redirect(`/${teamItem.teamNamespace}`)
}

export async function createFirstTeamAction(formData: CreateTeamSchema) {
  const res = await createTeam(formData)

  if (res.isError) {
    return res
  }

  await refreshUserTeamsAction()
  await refreshTeamInfoAction()
}

export async function createTeamAction() {

}
