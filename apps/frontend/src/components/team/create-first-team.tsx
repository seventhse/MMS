import type { UserInfo } from '~/services/auth'
import { CreateTeamForm } from './create-team-form'

export interface CreateFirstTeamProps {
  userInfo: UserInfo
}

// TODO: Wait import plan select
export async function CreateFirstTeam({ userInfo }: CreateFirstTeamProps) {
  return (
    <div className="flex items-center justify-center">
      <div className="max-w-xl w-full space-y-8 p-8 rounded-lg">
        <div>
          <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
            Welcome to MMS!
          </h2>
          <p className="mt-2 text-center text-sm text-gray-600">
            Let's create your first team to get started
          </p>
        </div>
        <div className="mt-8">
          <CreateTeamForm userInfo={userInfo} />
        </div>
        <div className="text-sm text-center text-gray-500 mt-4">
          You can add more team members after creating your team
        </div>
      </div>
    </div>
  )
}
