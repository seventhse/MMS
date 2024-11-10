import {
  Button,
  Divider,
  Input,
  Label,
} from '@mms/ui'
import Link from 'next/link'
import { Routes } from '~/constants/routes'

export default function LoginPage() {
  return (
    <>
      <h3 className="text-2xl mb-6">Sign in to MMS</h3>

      <Button variant="outline" size="lg" className="w-full">
        Login with Github
      </Button>

      <Divider>Or sign in with email</Divider>

      <div className="grid gap-4">
        <div className="grid gap-2">
          <Label htmlFor="email">Email</Label>
          <Input
            id="email"
            type="email"
            placeholder="m@example.com"
            required
          />
        </div>
        <div className="grid gap-2">
          <div className="flex items-center">
            <Label htmlFor="password">Password</Label>
            <Link href={Routes.FORGET} className="ml-auto inline-block text-sm underline">
              Forgot your password?
            </Link>
          </div>
          <Input id="password" type="password" required />
        </div>
        <Button type="submit" className="w-full">
          Login
        </Button>
      </div>

      <div className="mt-4 text-center text-sm">
        Don&apos;t have an account ? &nbsp;
        <Link href={Routes.REGISTER} className="underline">
          Sign up
        </Link>
      </div>
    </>
  )
}
