export enum AuthRoutes {
  LOGIN = '/auth/login',
  REGISTER = '/auth/register',
  FORGET = '/auth/forget',
}

export enum AuthGuardRoutes {
  TEAM = '/team',
}

export const SignInRoute = AuthGuardRoutes.TEAM
export const SignOutRoute = AuthRoutes.LOGIN

export const Routes = {
  ...AuthRoutes,
  ...AuthGuardRoutes,
} as const
