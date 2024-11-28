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
  parent?: string
  originUrl?: string
}

export const SETTING_MENUS: MenuItem[] = [
  {
    title: 'General',
    url: TeamRoutes.SETTING,
  },
  {
    title: 'Members',
    url: TeamRoutes.SETTING_MEMBER,
  },
  {
    title: 'Billing',
    url: TeamRoutes.SETTING_BILL,
  },
  {
    title: 'Notifications',
    url: TeamRoutes.SETTING_NOTIFY,
  },
  {
    title: 'Activity Log',
    url: TeamRoutes.SETTING_LOG,
  },
].map((item) => {
  return {
    ...item,
    parent: 'Setting',
  }
})

export const TEAM_MENUS: MenuItem[] = [
  {
    title: 'Dashboard',
    url: TeamRoutes.DASHBOARD,
    icon: 'LayoutDashboard',
  },
  {
    title: 'Setting',
    url: TeamRoutes.SETTING,
    icon: 'Settings',
    excludeRole: ['Guest'],
    children: SETTING_MENUS,
  },
]

export const Menus = [
  ...SETTING_MENUS,
  ...TEAM_MENUS,
]
