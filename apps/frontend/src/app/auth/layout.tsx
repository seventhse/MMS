import type { ReactNode } from 'react'

export default function AuthLayout({ children }: { children: ReactNode }) {
  return (
    <div className="w-full min-h-screen flex">
      <aside className="hidden lg:block flex-1 bg-slate-950">

      </aside>
      <main className="flex-1 flex items-center justify-center">
        <div className="w-[95%] sm:w-[360px]">
          {children}
        </div>
      </main>
    </div>
  )
}
