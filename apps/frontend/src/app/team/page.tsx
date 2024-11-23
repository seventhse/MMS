import { redirect } from 'next/navigation'
import { getUserInfo, getUserTeams } from '~/actions/cache'
import { CreateFirstTeam } from '~/components/team/create-first-team'
import { SelectTeam } from '~/components/team/select-team'
import { SignOutRoute } from '~/constants/routes'

export default async function TeamPage() {
  // TODO: Check if a default team exists
  // - If a default team exists, navigate to that team
  // - If no default team exists, navigate to the team selection page
  // - If there are no teams, navigate to the create team page
  const userInfo = await getUserInfo()

  if (!userInfo) {
    redirect(SignOutRoute)
  }

  const teams = await getUserTeams()

  if (!teams?.length) {
    return <CreateFirstTeam userInfo={userInfo} />
  }

  if (!userInfo?.defaultTeamId) {
    return <SelectTeam />
  }

  redirect('/')
}
