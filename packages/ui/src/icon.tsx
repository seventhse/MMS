'use client'
import type { LucideIcon } from 'lucide-react'
import { icons } from 'lucide-react'

export type IconName = keyof typeof icons

export interface IconProps {
  name?: IconName
  className?: string
  size?: string | number
  strokeWidth?: string | number
}

export function Icon({
  name,
  className,
  size = 24,
  strokeWidth = 2,
}: IconProps) {
  if (!name) {
    return
  }

  const LIcon = icons[name] as LucideIcon

  return (
    <LIcon
      className={className}
      size={size}
      strokeWidth={strokeWidth}
    />
  )
}
