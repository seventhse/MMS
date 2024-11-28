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
import { createFirstTeamAction } from '~/actions/team'
import { useFetch } from '~/hooks/use-fetch'
import type { UserInfo } from '~/services/auth'
import type { CreateTeamSchema } from '~/services/team'
import { createTeamSchema } from '~/services/team'

export interface CreateTeamFormProps {
  userInfo?: UserInfo
}

export function CreateTeamForm({ userInfo }: CreateTeamFormProps) {
  const { isLoading, action, isError, error } = useFetch(createFirstTeamAction)

  const form = useForm<CreateTeamSchema>({
    mode: 'onBlur',
    reValidateMode: 'onBlur',
    shouldFocusError: true,
    criteriaMode: 'firstError',
    resolver: zodResolver(createTeamSchema),
    defaultValues: {
      teamName: `${userInfo?.username || ''}'s team`,
      teamNamespace: `${userInfo?.username || ''}`,
      teamAvatar: '',
      description: '',
    },
  })

  return (
    <Form {...form}>
      <Loading loading={isLoading}>
        <form className="space-y-3" onSubmit={form.handleSubmit(action)}>
          <AlertDestructive visible={isError}>{error}</AlertDestructive>
          <FormField
            control={form.control}
            name="teamName"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Team Name</FormLabel>
                <FormControl>
                  <Input placeholder="Enter team name" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="teamNamespace"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Namespace</FormLabel>
                <FormControl>
                  <Input placeholder="Enter team namespace" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="teamAvatar"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Team Avatar URL</FormLabel>
                <FormControl>
                  <Input placeholder="Enter avatar URL" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="description"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Description</FormLabel>
                <FormControl>
                  <Input placeholder="Enter team description" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          <Button className="w-full" type="submit">
            Create Team
          </Button>
        </form>
      </Loading>
    </Form>
  )
}
