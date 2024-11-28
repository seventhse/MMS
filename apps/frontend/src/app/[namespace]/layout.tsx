import type { PropsWithChildren } from 'react'
import {
  Loading,
  SidebarProvider,
  SidebarTrigger,
} from '@mms/ui'
import { redirect } from 'next/navigation'
import { Suspense } from 'react'
import { getTeamInfo, getUserInfo, getUserTeams } from '~/actions/cache'
import { AppBreadcrumb } from '~/components/app/app-breadcrumb'
import { AppSidebar } from '~/components/app/app-sidebar'
import { AuthGuardRoutes, SignOutRoute } from '~/constants/routes'

export default async function TeamLayout({ children }: PropsWithChildren) {
  const info = await getUserInfo()
  const teams = await getUserTeams()
  const currentTeam = await getTeamInfo()

  if (!info) {
    redirect(SignOutRoute)
  }

  if (!currentTeam || !teams) {
    redirect(AuthGuardRoutes.TEAM)
  }

  return (
    <SidebarProvider className="w-screen h-screen">
      <AppSidebar info={info} teams={teams} currentTeam={currentTeam} />
      <main className="flex-1 p-3 flex flex-col overflow-hidden">
        <header className="mb-3 space-x-3 flex items-center">
          <SidebarTrigger />
          <AppBreadcrumb teamNamespace={currentTeam.teamNamespace} />
        </header>
        <section className="size-full rounded-lg overflow-y-auto">
          <Suspense fallback={<Loading loading />}>
            {children}
          </Suspense>
        </section>
      </main>
    </SidebarProvider>
  )
}
