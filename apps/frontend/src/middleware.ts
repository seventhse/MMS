import type { NextRequest } from 'next/server'
import { NextResponse } from 'next/server'
import { AuthGuardRoutes, SignInRoute, SignOutRoute } from './constants/routes'
import { getSession } from './lib/session'

const publicRoutes: string[] = Object.values(AuthGuardRoutes)
const protectedRoutes: string[] = Object.values(AuthGuardRoutes)

export default async function middleware(req: NextRequest) {
  const path = req.nextUrl.pathname
  const isPublicRoute = publicRoutes.includes(path)
  const isProtectedRoute = protectedRoutes.includes(path)
  const session = await getSession()

  if (isProtectedRoute && !session) {
    return NextResponse.redirect(new URL(SignOutRoute, req.nextUrl))
  }

  if (isPublicRoute && session && !path.startsWith(SignInRoute)) {
    return NextResponse.redirect(new URL(SignInRoute, req.nextUrl))
  }

  return NextResponse.next()
}

// Routes Middleware should not run on
export const config = {
  matcher: ['/((?!api|_next/static|_next/image|.*\\.png$).*)'],
}
