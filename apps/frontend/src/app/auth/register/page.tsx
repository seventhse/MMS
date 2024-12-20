import {
  Button,
  Divider,
} from '@mms/ui'
import Link from 'next/link'
import RegisterFrom from '~/components/auth/register-form'
import { Routes } from '~/constants/routes'

export const metadata = {
  title: 'Register - mms',
}

export default function RegisterPage() {
  return (
    <>
      <h3 className="text-2xl mb-6">Sign up to MMS</h3>

      <RegisterFrom />

      <Divider>Or</Divider>

      <Button
        variant="outline"
        size="lg"
        className="w-full"
      >
        Sign up with Github
      </Button>

      <p className="px-8 text-center text-sm text-muted-foreground mt-6">
        By clicking continue, you agree to our&nbsp;
        <a className="underline underline-offset-4 hover:text-primary" href="/terms">
          Terms of Service
        </a>
        &nbsp;and&nbsp;
        <a className="underline underline-offset-4 hover:text-primary" href="/privacy">
          Privacy Policy
        </a>
        .
      </p>

      <div className="mt-4 text-center text-sm">
        Already have an account?&nbsp;
        <Link href={Routes.LOGIN} className="underline">
          Sign In
        </Link>
      </div>
    </>
  )
}
