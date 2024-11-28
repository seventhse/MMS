export const HEADER_PATH_KEY = 'x-current-path'

export enum AuthRoutes {
  LOGIN = '/auth/login',
  REGISTER = '/auth/register',
  FORGET = '/auth/forget',
}

export enum AuthGuardRoutes {
  TEAM = '/team',
}

export enum TeamRoutes {
  DASHBOARD = '/',
  SETTING = '/setting',
  SETTING_MEMBER = '/setting/member',
  SETTING_NOTIFY = '/setting/notify',
  SETTING_BILL = '/setting/bill',
  SETTING_LOG = '/setting/log',
}

export const SignInRoute = '/'
export const SignOutRoute = AuthRoutes.LOGIN

export const Routes = {
  ...AuthRoutes,
  ...AuthGuardRoutes,
  ...TeamRoutes,
} as const
