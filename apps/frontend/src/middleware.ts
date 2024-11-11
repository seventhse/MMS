import type { NextRequest } from 'next/server'
import { NextResponse } from 'next/server'
import { Routes } from './constants/routes'
import { getSession } from './lib/session'

// 1. Specify protected and public routes
const publicRoutes: string[] = [Routes.LOGIN, Routes.FORGET, Routes.REGISTER, '/']
const protectedRoutes: string[] = [Routes.DASHBOARD, Routes.WORKSPACE]

export default async function middleware(req: NextRequest) {
  const path = req.nextUrl.pathname
  const isPublicRoute = publicRoutes.includes(path)
  const isProtectedRoute = protectedRoutes.includes(path)
  const session = await getSession()

  if (isProtectedRoute && !session) {
    return NextResponse.redirect(new URL(Routes.LOGIN, req.nextUrl))
  }

  if (
    isPublicRoute && session && !(path.startsWith(Routes.DASHBOARD) || path.startsWith(Routes.WORKSPACE))
  ) {
    return NextResponse.redirect(new URL(Routes.DASHBOARD, req.nextUrl))
  }

  return NextResponse.next()
}

// Routes Middleware should not run on
export const config = {
  matcher: ['/((?!api|_next/static|_next/image|.*\\.png$).*)'],
}
