export default function WorkspaceLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <div className="w-full max-w-screen min-h-screen max-h-screen">
      {children}
    </div>
  )
}
