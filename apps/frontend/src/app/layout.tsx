import { Loading } from '@mms/ui'
import { Suspense } from 'react'
import '@mms/ui/global.css'

export const metadata = {
  title: 'MMS',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className="w-full max-w-screen h-screen max-h-screen bg-white dark:bg-black">
        <Suspense fallback={<Loading loading text="In Load Resource..." />}>
          {children}
        </Suspense>
      </body>
    </html>
  )
}
