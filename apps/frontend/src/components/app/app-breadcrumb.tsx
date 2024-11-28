'use client'

import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from '@mms/ui'
import { usePathname } from 'next/navigation'
import { Fragment } from 'react'
import { Menus } from '~/constants/menu'
import { findMenuParents } from '~/utils/menu-helper'

export interface AppBreadcrumbProps {
  teamNamespace: string
}

export function AppBreadcrumb({ teamNamespace }: AppBreadcrumbProps) {
  const pathname = usePathname()
  const item = Menus.find((item) => {
    return item.url.endsWith('/') ? item.url === `${pathname}/` : item.url === pathname
  })

  const parents = item ? findMenuParents(Menus, item) : []

  return (
    <Breadcrumb>
      <BreadcrumbList>
        <BreadcrumbItem>
          <BreadcrumbLink href="/">{ teamNamespace }</BreadcrumbLink>
        </BreadcrumbItem>

        {parents.map(parent => (
          <Fragment key={parent.title + parent.url}>
            <BreadcrumbSeparator />
            <BreadcrumbItem>
              <BreadcrumbLink href={parent.url}>{parent.title}</BreadcrumbLink>
            </BreadcrumbItem>
          </Fragment>
        ))}

        <BreadcrumbSeparator />
        <BreadcrumbItem>
          <BreadcrumbPage>{item?.title}</BreadcrumbPage>
        </BreadcrumbItem>
      </BreadcrumbList>
    </Breadcrumb>
  )
}
