'use client'

import {
  Avatar,
  AvatarFallback,
  AvatarImage,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from '@mms/ui'

import {
  ChevronsUpDown,
  LogOut,
} from 'lucide-react'
import { useMemo } from 'react'
import { signOutAction } from '~/actions/auth'
import type { UserInfo } from '~/services/auth'

interface NavUserProps {
  user: UserInfo
}

function Info({ user }: NavUserProps) {
  const username = useMemo(() => user?.displayName || user?.username, [user])

  return (
    <>
      <Avatar className="h-8 w-8 rounded-lg">
        <AvatarImage className="size-full" src={user?.avatar || ''} alt={username} />
        <AvatarFallback className="rounded">{username.slice(0, 1)}</AvatarFallback>
      </Avatar>
      <div className="grid flex-1 text-left text-sm leading-tight">
        <span className="truncate font-semibold">{username}</span>
        <span className="truncate text-xs">{user?.email}</span>
      </div>
    </>
  )
}

export function NavUser({ user }: NavUserProps) {
  const { isMobile } = useSidebar()
  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <SidebarMenuButton
              size="lg"
              className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
            >
              <Info user={user} />
              <ChevronsUpDown className="ml-auto size-4" />
            </SidebarMenuButton>
          </DropdownMenuTrigger>
          <DropdownMenuContent
            className="w-[--radix-dropdown-menu-trigger-width] min-w-56 rounded-lg"
            side={isMobile ? 'bottom' : 'right'}
            align="end"
            sideOffset={4}
          >
            <DropdownMenuLabel className="p-0 font-normal">
              <div className="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
                <Info user={user} />
              </div>
            </DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuItem onClick={signOutAction}>
              <LogOut />
              Log out
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarMenuItem>
    </SidebarMenu>
  )
}
