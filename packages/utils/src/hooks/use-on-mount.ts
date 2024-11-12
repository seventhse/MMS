'use client'
import { useEffect } from 'react'
import { useLatest } from './use-latest'

export function useOnMount(fn: () => void) {
  const fnRef = useLatest(fn)

  useEffect(() => {
    fnRef.current?.()
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])
}
