'use client'

import { zodResolver } from '@hookform/resolvers/zod'
import {
  AlertDestructive,
  Button,
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
  Input,
  Loading,
  useForm,
} from '@mms/ui'
import Link from 'next/link'
import { setAuthInfoAction } from '~/actions/auth'
import { Routes } from '~/constants/routes'
import { useFetch } from '~/hooks/use-fetch'
import type { LoginFormSchema } from '~/services/auth'
import { login, loginFormSchema } from '~/services/auth'

export function LoginForm() {
  const form = useForm<LoginFormSchema>({
    mode: 'onBlur',
    reValidateMode: 'onBlur',
    shouldFocusError: true,
    criteriaMode: 'firstError',
    resolver: zodResolver(loginFormSchema),
    defaultValues: {
      email: 'season.sevent@icloud.com',
      password: 'Seventhse233.',
    },
  })

  const { isLoading, action, isError, error } = useFetch(login, { onSuccess: async (data) => {
    await setAuthInfoAction(data!)
  } })

  return (
    <Form {...form}>
      <Loading loading={isLoading} text="In validation info...">
        <form className="space-y-3" onSubmit={form.handleSubmit(action)}>
          <AlertDestructive visible={isError}>{ error }</AlertDestructive>
          <FormField
            control={form.control}
            name="email"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Email</FormLabel>
                <FormControl>
                  <Input placeholder="please input your email" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="password"
            render={({ field }) => (
              <FormItem>
                <FormLabel className="w-full flex items-center justify-between">
                  <span className="inline-block mr-auto">Password</span>
                  <Link className="ml-auto inline-block text-sm underline" href={Routes.FORGET}>Forget your password?</Link>
                </FormLabel>
                <FormControl>
                  <Input type="password" placeholder="please input your password" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          <Button className="w-full" type="submit">
            Login
          </Button>
        </form>
      </Loading>
    </Form>
  )
}
