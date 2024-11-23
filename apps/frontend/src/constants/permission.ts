export const Roles = ['Owner', 'Admin', 'Manager', 'Member', 'Guest'] as const

export type Role = typeof Roles[number]
