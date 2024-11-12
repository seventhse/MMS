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
  useForm,
} from '@mms/ui'
import { singIn } from '~/actions/auth'
import type { LoginFormSchema } from '~/api/auth'
import { loginFormSchema } from '~/api/auth'

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

  async function onSubmit(value: LoginFormSchema) {
    await singIn(value).catch((e) => {
      form.setError('password', {
        message: e.message,
      })
    })
  }

  return (
    <Form {...form}>
      <form className="space-y-3" onSubmit={form.handleSubmit(onSubmit)}>
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
              <FormLabel>Password</FormLabel>
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
    </Form>
  )
}
