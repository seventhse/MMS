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
import { setAuthInfoAction } from '~/actions/auth'
import { useFetch } from '~/hooks/use-fetch'
import {
  register,
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

  const { isLoading, action, isError, error } = useFetch(register, {
    onSuccess: async (data) => {
      await setAuthInfoAction(data!)
    },
  })

  return (
    <Form {...form}>
      <Loading loading={isLoading} text="In register...">
        <form className="space-y-3" onSubmit={form.handleSubmit(action)}>
          <AlertDestructive visible={isError}>{ error }</AlertDestructive>
          <FormField
            control={form.control}
            name="username"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Username</FormLabel>
                <FormControl>
                  <Input placeholder="please input your username" {...field} />
                </FormControl>
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
                <FormMessage />
              </FormItem>
            )}
          />
          <Button className="w-full" type="submit">
            Sign up with Email
          </Button>
        </form>
      </Loading>
    </Form>
  )
}

export default RegisterFrom
