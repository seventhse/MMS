'use client'

import { Tabs, TabsList, TabsTrigger } from '@mms/ui'
import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { SETTING_MENUS } from '~/constants/menu'

export function SettingTabs() {
  const pathname = usePathname()

  return (
    <Tabs defaultValue={pathname} className="w-full max-w-[768px] mb-3 overflow-hidden">
      <TabsList className="w-full rounded-full h-[45px]  overflow-x-auto">
        {
          SETTING_MENUS.map((item) => {
            const url = item.url
            return (
              <TabsTrigger key={url} value={url} asChild className="flex-1 h-[35px] rounded-full">
                <Link href={url}>{item.title}</Link>
              </TabsTrigger>
            )
          })
        }
      </TabsList>
    </Tabs>
  )
}
