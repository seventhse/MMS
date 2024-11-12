'use client'
import type { MutableRefObject } from 'react'
import { useRef } from 'react'

export function useLatest<T = (...args: any) => any>(fn: T): MutableRefObject<T> {
  const fnRef = useRef(fn)

  fnRef.current = fn

  return fnRef
}
