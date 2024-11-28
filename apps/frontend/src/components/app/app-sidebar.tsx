'use client'

import type { ComponentProps } from 'react'
import { Sidebar, SidebarContent, SidebarFooter, SidebarHeader } from '@mms/ui'
import dynamic from 'next/dynamic'
import { usePathname } from 'next/navigation'
import { TEAM_MENUS } from '~/constants/menu'
import type { UserInfo, UserTeamItem } from '~/services/auth'
import { filterMenuByRole } from '~/utils/menu-helper'
import { NavUser } from './nav-user'
import { TeamSwitcher } from './team-switcher'

const DynamicNavMenus = dynamic(() => import('./nav-menus'), { ssr: false })

export interface AppSidebarProps extends ComponentProps<typeof Sidebar> {
  info: UserInfo
  teams: UserTeamItem[]
  currentTeam: UserTeamItem
}
export function AppSidebar(props: AppSidebarProps) {
  const { info, teams, currentTeam, ...restProps } = props
  const pathname = usePathname()
  const routePrefix = `/${currentTeam?.teamNamespace}`
  const fullPath = pathname.slice(pathname.indexOf(currentTeam?.teamNamespace))

  const teamMenus = filterMenuByRole(TEAM_MENUS, currentTeam!.role, (item) => {
    if (item.url && !item.url.startsWith(routePrefix)) {
      item.originUrl ||= item.url
      const newUrl = `${routePrefix}${item.originUrl.startsWith('/') ? item.url : `/${item.url}`}`
      item.url = newUrl
      item.isActive = item.children ? !!fullPath?.includes(item.originUrl) : newUrl === pathname
    }
  })

  return (
    <Sidebar {...restProps}>
      <SidebarHeader>
        <TeamSwitcher currentTeam={currentTeam!} teams={teams || []} />
      </SidebarHeader>
      <SidebarContent>
        <DynamicNavMenus label="Team" items={teamMenus} />
      </SidebarContent>
      <SidebarFooter>
        <NavUser user={info!} />
      </SidebarFooter>
    </Sidebar>
  )
}
