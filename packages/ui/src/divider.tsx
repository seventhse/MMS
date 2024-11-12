import type { PropsWithChildren } from 'react'
import { Slot } from '@radix-ui/react-slot'

export interface DividerProps extends PropsWithChildren {
  asChild?: boolean
}

function Divider({ children, asChild }: DividerProps) {
  const Comp = asChild ? Slot : 'span'
  return (
    <div className="relative my-6">
      <div className="absolute inset-0 flex items-center">
        <span className="w-full border-t"></span>
      </div>
      {
        !!children
        && (
          <div className="relative flex justify-center text-xs">
            <Comp className="bg-background px-2 text-muted-foreground">{children}</Comp>
          </div>
        )
      }
    </div>
  )
}

export { Divider }
