import type { IconName } from '@mms/ui'
import type { Role } from './permission'
import { TeamRoutes } from './routes'

export interface MenuItem {
  title: string
  url: string
  isActive?: boolean
  includeRole?: Role[]
  excludeRole?: Role[]
  icon?: IconName
  children?: MenuItem[]
}

export const TEAM_MENUS: MenuItem[] = [
  {
    title: 'Dashboard',
    url: TeamRoutes.DASHBOARD,
    icon: 'LayoutDashboard',
  },
  {
    title: 'Setting',
    url: '',
    icon: 'Settings',
    excludeRole: ['Guest'],
    children: [
      {
        title: 'Profile',
        url: TeamRoutes.SETTING_PROFILE,
      },
      {
        title: 'Member',
        url: TeamRoutes.SETTING_MEMBER,
      },
    ],
  },
]
