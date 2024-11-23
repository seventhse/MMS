import type { ComponentProps } from 'react'
import { Sidebar, SidebarContent, SidebarFooter, SidebarHeader } from '@mms/ui'
import { getTeamInfo, getUserInfo, getUserTeams } from '~/actions/cache'
import { TEAM_MENUS } from '~/constants/menu'
import { filterMenuByRole } from '~/utils/menu-helper'
import { NavMenus } from './nav-menus'
import { NavUser } from './nav-user'
import { TeamSwitcher } from './team-switcher'

export interface AppSidebarProps extends ComponentProps<typeof Sidebar> {
}

export async function AppSidebar(props: AppSidebarProps) {
  const info = await getUserInfo()
  const teams = await getUserTeams()
  const currentTeam = await getTeamInfo()

  const routePrefix = `/${currentTeam?.teamNamespace}`

  const teamMenus = filterMenuByRole(TEAM_MENUS, currentTeam!.role, (item) => {
    if (item.url && !item.url.startsWith(routePrefix)) {
      item.url = `${routePrefix}${item.url.startsWith('/') ? item.url : `/${item.url}`}`
    }
  })

  return (
    <Sidebar {...props}>
      <SidebarHeader>
        <TeamSwitcher currentTeam={currentTeam!} teams={teams || []} />
      </SidebarHeader>
      <SidebarContent>
        <NavMenus label="Team" items={teamMenus} />
      </SidebarContent>
      <SidebarFooter>
        <NavUser user={info!} />
      </SidebarFooter>
    </Sidebar>
  )
}
