import '@mms/ui/global.css'

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className="w-full max-w-screen min-h-screen max-h-screen">
        {children}
      </body>
    </html>
  )
}
