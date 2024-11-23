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

import { ChevronsUpDown, Plus } from 'lucide-react'
import { refreshTeamInfoAction } from '~/actions/team'
import type { UserTeamItem } from '~/services/auth'

export interface TeamSwitcherProps {
  currentTeam: UserTeamItem
  teams: UserTeamItem[]
}

function TeamItem({ team }: { team: UserTeamItem }) {
  return (
    <>
      <Avatar className="h-8 w-8 rounded-lg">
        <AvatarImage className="size-full" src={team?.teamAvatar || ''} alt={team.teamName} />
        <AvatarFallback className="rounded">{team.teamName.slice(0, 1)}</AvatarFallback>
      </Avatar>
      <div className="grid flex-1 text-left text-sm leading-tight">
        <span className="truncate font-semibold">
          {team?.teamName}
        </span>
      </div>
    </>
  )
}

export function TeamSwitcher({ currentTeam, teams }: TeamSwitcherProps) {
  const { isMobile } = useSidebar()

  const handleSwitchTeam = async (item: UserTeamItem) => {
    await refreshTeamInfoAction(item.teamId)
  }

  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <SidebarMenuButton
              size="lg"
              className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
            >
              <TeamItem team={currentTeam} />
              <ChevronsUpDown className="ml-auto" />
            </SidebarMenuButton>
          </DropdownMenuTrigger>
          <DropdownMenuContent
            className="w-[--radix-dropdown-menu-trigger-width] min-w-56 rounded-lg"
            align="start"
            side={isMobile ? 'bottom' : 'right'}
            sideOffset={4}
          >
            <DropdownMenuLabel className="text-xs text-muted-foreground">
              Teams
            </DropdownMenuLabel>
            {teams.map(team => (
              <DropdownMenuItem
                key={team.teamId}
                className="gap-2 p-2"
                onClick={() => {
                  handleSwitchTeam(team)
                }}
              >
                <TeamItem team={team} />
              </DropdownMenuItem>
            ))}
            <DropdownMenuSeparator />
            <DropdownMenuItem className="gap-2 p-2">
              <div className="flex size-6 items-center justify-center rounded-md border bg-background">
                <Plus className="size-4" />
              </div>
              <div className="font-medium text-muted-foreground">Add team</div>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarMenuItem>
    </SidebarMenu>
  )
}
