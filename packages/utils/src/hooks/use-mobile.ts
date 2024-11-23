'use client'

import { useCallback, useEffect, useState } from 'react'
import { isMobileDevice } from '../client'

export function useIsMobile() {
  const [isMobile, setIsMobile] = useState(false)

  const handleResize = useCallback(() => {
    setIsMobile(window.innerWidth <= 640 || isMobileDevice())
  }, [])

  useEffect(() => {
    handleResize()
    window.addEventListener('resize', handleResize)
    return () => window.removeEventListener('resize', handleResize)
  }, [handleResize])

  return isMobile
}
