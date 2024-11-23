'use client'

import { zodResolver } from '@hookform/resolvers/zod'
import {
  Button,
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
  Input,
  Loading,
  useForm,
} from '@mms/ui'
import Link from 'next/link'
import { singInAction } from '~/actions/auth'
import { Routes } from '~/constants/routes'
import { useFetch } from '~/hooks/use-fetch'
import type { LoginFormSchema } from '~/services/auth'
import { loginFormSchema } from '~/services/auth'

export function LoginForm() {
  const form = useForm<LoginFormSchema>({
    mode: 'onBlur',
    reValidateMode: 'onBlur',
    shouldFocusError: true,
    criteriaMode: 'firstError',
    resolver: zodResolver(loginFormSchema),
    defaultValues: {
      email: '',
      password: '',
    },
  })

  const { isLoading, action } = useFetch(singInAction)

  return (
    <Form {...form}>
      <Loading loading={isLoading} text="In validation info...">
        <form className="space-y-3" onSubmit={form.handleSubmit(action)}>
          <FormField
            control={form.control}
            name="email"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Email</FormLabel>
                <FormControl>
                  <Input placeholder="please input your email" {...field} />
                </FormControl>
                <FormDescription>
                  Your email will be used for account verification and recovery.
                </FormDescription>
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
                <FormDescription>
                  Choose a secure password with at least 6 characters.
                </FormDescription>
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
