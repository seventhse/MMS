import { type ClassValue, clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'

export * from './hooks/use-latest'
export * from './hooks/use-on-mount'

export * from 'lodash-es'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
