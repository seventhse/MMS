import {
  Button,
  Divider,
} from '@mms/ui'
import Link from 'next/link'
import { LoginForm } from '~/components/auth/login-form'
import { Routes } from '~/constants/routes'

export default function LoginPage() {
  return (
    <>
      <h3 className="text-2xl mb-6">Sign in to MMS</h3>

      <Button variant="outline" size="lg" className="w-full">
        Login with Github
      </Button>

      <Divider>Or sign in with email</Divider>

      <LoginForm />

      <div className="mt-4 text-center text-sm">
        Don&apos;t have an account ? &nbsp;
        <Link href={Routes.REGISTER} className="underline">
          Sign up
        </Link>
      </div>
    </>
  )
}
