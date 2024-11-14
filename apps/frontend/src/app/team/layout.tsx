import type { PropsWithChildren } from 'react'

export default function TeamLayout({ children }: PropsWithChildren) {
  return (
    <div className="w-full min-h-screen">
      {children}
    </div>
  )
}
