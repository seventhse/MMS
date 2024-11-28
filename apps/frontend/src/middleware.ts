import type { NextRequest } from 'next/server'
import { NextResponse } from 'next/server'
import { getToken, getUserInfo, getUserTeams, setTeamInfo } from './actions/cache'
import { AuthGuardRoutes, AuthRoutes, Routes, SignInRoute, SignOutRoute } from './constants/routes'

const publicRoutes: string[] = Object.values(AuthRoutes)
const protectedRoutes: string[] = Object.values(AuthGuardRoutes)

export default async function middleware(req: NextRequest) {
  const path = req.nextUrl.pathname
  const isPublicRoute = publicRoutes.includes(path)
  const isProtectedRoute = protectedRoutes.includes(path)
  const token = await getToken()

  const redirect = (path: string) => {
    const response = NextResponse.redirect(new URL(path, req.nextUrl))
    return response
  }

  if (isProtectedRoute && !token) {
    return redirect(SignOutRoute)
  }

  if (isPublicRoute && !token && !path.startsWith(SignInRoute)) {
    return redirect(SignInRoute)
  }

  const userInfo = await getUserInfo()
  const teams = await getUserTeams()

  // Default route check
  if (path === '/' && token) {
    if (!userInfo?.defaultTeamId || !teams?.length) {
      return redirect(Routes.TEAM)
    }

    const teamItem = teams.find(item => item.teamId === userInfo.defaultTeamId)

    if (!teamItem) {
      return redirect(Routes.TEAM)
    }

    await setTeamInfo(teamItem)

    return redirect(`/${teamItem.teamNamespace}`)
  }

  return NextResponse.next()
}

// Routes Middleware should not run on
export const config = {
  matcher: ['/((?!api|actions|_next/static|_next/image|.*\\.png$).*)'],
  missing: [
    { type: 'header', key: 'next-router-prefetch' },
    { type: 'header', key: 'purpose', value: 'prefetch' },
  ],
}
