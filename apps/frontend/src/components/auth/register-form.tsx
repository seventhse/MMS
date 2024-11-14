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
import { signUp } from '~/actions/auth'
import {
  registerFormSchema,
  type RegisterFormSchema,
} from '~/services/auth'

function RegisterFrom() {
  const form = useForm<RegisterFormSchema>({
    mode: 'onBlur',
    reValidateMode: 'onBlur',
    shouldFocusError: true,
    criteriaMode: 'firstError',
    resolver: zodResolver(registerFormSchema),
    defaultValues: {
      email: '',
      username: '',
      password: '',
    },
  })

  async function onSubmit(value: RegisterFormSchema) {
    await signUp(value)
  }

  return (
    <Form {...form}>
      <form className="space-y-3" onSubmit={form.handleSubmit(onSubmit)}>
        <FormField
          control={form.control}
          name="username"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Username</FormLabel>
              <FormControl>
                <Input placeholder="please input your username" {...field} />
              </FormControl>
              <FormDescription>
                Choose a username that will identify you on the platform.
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
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
          Sign up with Email
        </Button>
      </form>
    </Form>
  )
}

export default RegisterFrom
