import { useOnMount } from '@mms/utils'
import { useState } from 'react'

export interface Options<T, P extends any[]> {
  defaultParams?: P
  defaultData?: T
  manual?: boolean
  onError?: (e: Error) => void
  onSuccess?: (data?: T | void) => void
}

export function useFetch<T = unknown, P extends any[] = any[]>(
  service: (...params: P) => Promise<T>,
  options?: Options<T, P>,
) {
  const { manual = true, defaultParams, defaultData, onSuccess, onError } = options || {}

  const [isLoading, setIsLoading] = useState(false)
  const [isError, setIsError] = useState(false)
  const [error, setError] = useState('')
  const [data, setData] = useState<T | null>(defaultData as T || null)

  async function action(...params: P): Promise<T | void> {
    setIsLoading(true)
    setIsError(false)
    setError('')

    try {
      const res = await service(...params)
      setData(res)
      onSuccess?.(res)
      return res
    }
    catch (e) {
      const error = e as Error
      setError(error.message)
      setIsError(true)
      onError?.(error)
      return undefined
    }
    finally {
      setIsLoading(false)
    }
  }

  useOnMount(() => {
    if (!manual) {
      action(...(defaultParams || ([] as any)))
    }
  })

  return {
    isLoading,
    isError,
    error,
    action,
    data,
  }
}
