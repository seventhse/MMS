import { useToast } from '@mms/ui/hooks'
import { useOnMount } from '@mms/utils'
import { useState } from 'react'
import type { BaseResponse } from '~/lib/request'

export interface Options<T, P extends any[]> {
  defaultParams?: P
  defaultData?: T
  manual?: boolean
  onError?: () => void
  onSuccess?: (data?: T | null | undefined) => void | Promise<void>
  notify?: boolean
}

export function useFetch<T = unknown, P extends any[] = any[]>(
  service: (...params: P) => Promise<BaseResponse<T> | undefined | null>,
  options?: Options<T, P>,
) {
  const { toast } = useToast()
  const { manual = true, defaultParams, defaultData, onSuccess, onError, notify } = options || {}

  const [isLoading, setIsLoading] = useState(false)
  const [isError, setIsError] = useState(false)
  const [error, setError] = useState<string>('')
  const [data, setData] = useState<T | null>(defaultData as T || null)

  function errorInterceptor(message: string) {
    setError(message)
    setIsError(true)
    onError?.()
    if (notify) {
      toast({
        duration: 5000,
        description: message,
      })
    }
  }

  const clearError = () => {
    setIsError(false)
    setError('')
  }

  async function action(...params: P): Promise<T | null | undefined> {
    setIsLoading(true)
    clearError()
    try {
      const res = await service(...params)

      if (!res) {
        onSuccess?.()
        return
      }

      if (res?.isError) {
        errorInterceptor(res?.error || res?.message || '')
        return
      }

      setData(res?.data)
      onSuccess?.(res?.data)

      return res.data
    }
    catch (e) {
      const error = e as Error
      if (error.message.includes('NEXT_REDIRECT')) {
        return
      }
      errorInterceptor(error.message || 'Server error, please try again')
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
    clearError,
  }
}
