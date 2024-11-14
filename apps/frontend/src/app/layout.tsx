import { Loading } from '@mms/ui'
import { Suspense } from 'react'
import '@mms/ui/global.css'

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className="w-full max-w-screen h-screen max-h-screen bg-primary-foreground">
        <Suspense fallback={<Loading loading text="In Load Resource..." />}>
          {children}
        </Suspense>
      </body>
    </html>
  )
}
