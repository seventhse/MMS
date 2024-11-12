import { Button } from '@mms/ui'
import { signOut } from '~/actions/auth'

export default async function DashboardPage() {
  async function doLogout() {
    'use server'
    await signOut()
  }

  return (
    <>
      dashboard page
      <form action={doLogout}>
        <Button type="submit">Logout</Button>
      </form>
    </>
  )
}
