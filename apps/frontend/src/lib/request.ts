'use server'

import { redirect } from 'next/navigation'
import { getToken } from '~/actions/cache'
import { Routes } from '~/constants/routes'

// eslint-disable-next-line node/prefer-global/process
const BASE_URL = process.env.BASE_API_URL || 'http://127.0.0.1:10086/api/v1'

export type RequestData<T = unknown> = Record<string, T>
export interface RequestOptions {
  data: RequestData
}

export interface BaseResponse<T> {
  code: number
  message: null | string
  data: T | null
  isError?: boolean
  error?: string
}

function paramsToString(params: RequestData<string>) {
  return Object.entries(params).reduce((pre, [key, value]) => {
    const encodedKey = encodeURIComponent(key)
    const encodedValue = encodeURIComponent(value)
    if (pre) {
      return `${pre}&${encodedKey}=${encodedValue}`
    }
    return `${encodedKey}=${encodedValue}`
  }, '')
}

function composeUrl(url: string, params?: RequestData<string>) {
  let baseUrl = `${BASE_URL}${url}`
  if (params) {
    baseUrl += `?${paramsToString(params)}`
  }
  return baseUrl
}

export async function request<T = any>(
  method: 'GET' | 'POST',
  url: string,
  requestData?: RequestData<string> | any,
): Promise<BaseResponse<T>> {
  const isGet = method?.toUpperCase() === 'GET'
  url = composeUrl(url, isGet ? requestData : undefined)

  const token = await getToken()

  const options: RequestInit & { headers: Record<string, string> } = {
    method,
    headers: {
      'Content-Type': 'application/json',
    },
    cache: 'no-store',
  }

  if (token) {
    options.headers.Authorization = `Bearer ${token}`
  }

  if (!isGet) {
    options.body = JSON.stringify(requestData)
  }

  const res = await fetch(url, options)

  if (res.status !== 200) {
    return {
      code: res.status,
      message: 'Server error, please try again later',
      data: null,
      isError: true,
      error: 'Server error!',
    }
  }

  const responseData = (await res.json()) as unknown as BaseResponse<T>

  if (responseData.code !== 200) {
    if (responseData.code === 401) {
      redirect(Routes.LOGIN)
    }

    // Promise.reject(responseData.message)
    return {
      ...responseData,
      isError: true,
      error: responseData.message || '',
    }
  }

  return responseData
}

export async function get<T = any>(url: string, params?: RequestData<string>): Promise<BaseResponse<T>> {
  return request<T>('GET', url, params)
}

export async function post<T = any>(url: string, data?: any): Promise<BaseResponse<T>> {
  return request<T>('POST', url, data)
}
