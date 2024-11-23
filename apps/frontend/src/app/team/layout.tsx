import type { PropsWithChildren } from 'react'

export const metadata = {
  title: 'Team - mms',
}

export default function TeamLayout({ children }: PropsWithChildren) {
  return (
    <div className="flex flex-col w-full min-h-screen">
      <header className="shadow-md">
        <div className="container mx-auto px-4 py-3">
          <div className="text-primary text-2xl font-bold">MMS</div>
        </div>
      </header>
      <main className="flex-1 size-full mx-auto px-4 py-8 container">
        {children}
      </main>
    </div>
  )
}
