import type { PropsWithChildren } from 'react'
import { SettingTabs } from '~/components/app/setting-tabs'

export default async function SettingLayout({ children }: PropsWithChildren) {
  return (
    <>
      <SettingTabs />
      {children}
    </>
  )
}
