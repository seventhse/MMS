import type { HTMLAttributes, ReactNode } from 'react'
import { cn } from '@mms/utils'
import { Slot } from '@radix-ui/react-slot'

export interface LoadingProps extends HTMLAttributes<HTMLElement> {
  loading?: boolean
  asChild?: boolean
  children?: ReactNode
  text?: string
  maskClass?: string
  spin?: ReactNode
}

export function Spin() {
  return (
    <svg
      className="animate-spin size-5 text-primary"
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
    >
      <circle
        className="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        strokeWidth="4"
      >
      </circle>
      <path
        className="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      >
      </path>
    </svg>
  )
}

export function Loading({ loading, asChild, children, text = 'loading...', maskClass, spin, ...restProps }: LoadingProps) {
  const Comp = asChild ? Slot : 'div'

  return (
    <Comp {...restProps} className={cn('relative size-full', restProps.className)}>
      {children}
      {
        loading && (
          <div
            className={cn('absolute top-0 left-0 size-full bg-white/60 dark:bg-slate-800/60 text z-50 flex justify-center items-center', maskClass)}
          >
            <div className="flex items-center gap-x-1">
              {spin || <Spin />}
              {text}
            </div>
          </div>
        )
      }
    </Comp>
  )
}
