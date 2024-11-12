import { Button, Input, Label } from '@mms/ui'

export default function ForgetPage() {
  return (
    <>
      <h3 className="text-2xl mb-6">Forget Password?</h3>

      <div className="my-6">
        <p className="text-left text-sm">
          Enter the email address you used when you joined and we’ll send you instructions to reset your password.
        </p>

        <p className="mt-3 text-left text-sm">
          For security reasons, we do NOT store your password. So rest assured that we will never send your password via email.
        </p>
      </div>

      <div className="grid gap-4">
        <div className="grid gap-2">
          <Label htmlFor="email">Email Address</Label>
          <Input
            type="email"
            required
          />
        </div>
        <Button type="submit" size="lg">
          Send Reset Instructions
        </Button>
      </div>
    </>
  )
}
