export enum AuthRoutes {
  LOGIN = '/auth/login',
  REGISTER = '/auth/register',
  FORGET = '/auth/forget',
}

export enum AuthGuardRoutes {
  TEAM = '/team',
}

export enum TeamRoutes {
  DASHBOARD = '/dashboard',
  SETTING_PROFILE = '/setting/profile',
  SETTING_MEMBER = '/setting/member',
}

export const SignInRoute = '/'
export const SignOutRoute = AuthRoutes.LOGIN

export const Routes = {
  ...AuthRoutes,
  ...AuthGuardRoutes,
  ...TeamRoutes,
} as const
