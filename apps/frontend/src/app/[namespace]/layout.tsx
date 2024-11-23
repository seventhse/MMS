import type { PropsWithChildren } from 'react'
import { SidebarProvider, SidebarTrigger } from '@mms/ui'
import { AppSidebar } from '~/components/app/app-sidebar'

export default async function TeamLayout({ children }: PropsWithChildren) {
  return (
    <SidebarProvider>
      <AppSidebar />
      <main className="flex-1 p-3 flex flex-col">
        <header className="mb-3 space-x-3">
          <SidebarTrigger />
        </header>
        <section className="size-full rounded-lg">
          {children}
        </section>
      </main>
    </SidebarProvider>
  )
}
