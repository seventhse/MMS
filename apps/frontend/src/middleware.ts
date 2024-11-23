import type { NextRequest } from 'next/server'
import { NextResponse } from 'next/server'
import { getToken, getUserInfo, getUserTeams, setTeamInfo } from './actions/cache'
import { AuthGuardRoutes, AuthRoutes, SignInRoute, SignOutRoute } from './constants/routes'

const publicRoutes: string[] = Object.values(AuthRoutes)
const protectedRoutes: string[] = Object.values(AuthGuardRoutes)

export default async function middleware(req: NextRequest) {
  const path = req.nextUrl.pathname
  const isPublicRoute = publicRoutes.includes(path)
  const isProtectedRoute = protectedRoutes.includes(path)
  const token = await getToken()

  if (isProtectedRoute && !token) {
    return NextResponse.redirect(new URL(SignOutRoute, req.nextUrl))
  }

  if (isPublicRoute && !token && !path.startsWith(SignInRoute)) {
    return NextResponse.redirect(new URL(SignInRoute, req.nextUrl))
  }

  // Default route check
  if (path === '/' && token) {
    const userInfo = await getUserInfo()
    const teams = await getUserTeams()

    if (!userInfo?.defaultTeamId || !teams?.length) {
      return NextResponse.redirect(new URL(SignOutRoute, req.nextUrl))
    }

    const teamItem = teams.find(item => item.teamId === userInfo.defaultTeamId)

    if (!teamItem) {
      return NextResponse.redirect(new URL(SignOutRoute, req.nextUrl))
    }

    await setTeamInfo(teamItem)

    return NextResponse.redirect(new URL(`/${teamItem.teamNamespace}`, req.nextUrl))
  }

  return NextResponse.next()
}

// Routes Middleware should not run on
export const config = {
  matcher: ['/((?!api|_next/static|_next/image|.*\\.png$).*)'],
}
