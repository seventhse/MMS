import { post } from '~/lib/request'

export function createTeam() {
  return post('/team/create')
}
