'use client'

import { Icon } from '@mms/ui'
import { useRouter } from 'next/navigation'

export function Back() {
  const router = useRouter()

  const handleBack = () => {
    router.back()
  }

  return (
    <span className="inline-block leading-none" onClick={handleBack}>
      <Icon name="ChevronLeft" size={24} className="inline-block cursor-pointer" />
    </span>
  )
}
