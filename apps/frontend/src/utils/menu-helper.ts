import type { MenuItem } from '~/constants/menu'
import type { Role } from '~/constants/permission'

export function filterMenuByRole(
  menu: MenuItem[],
  role: Role,
  customCallback?: (item: MenuItem) => void,
): MenuItem[] {
  const filteredMenu = menu.reduce((acc: MenuItem[], item: MenuItem) => {
    const { includeRole, excludeRole } = item

    if (customCallback) {
      customCallback(item)
    }

    // If no role restrictions, include the item
    if (!includeRole && !excludeRole) {
      acc.push({
        ...item,
        children: item.children ? filterMenuByRole(item.children, role, customCallback) : undefined,
      })
      return acc
    }

    // Check include roles
    if (includeRole && !includeRole.includes(role)) {
      return acc
    }

    // Check exclude roles
    if (excludeRole && excludeRole.includes(role)) {
      return acc
    }

    // Add item with filtered children if it passes role checks
    acc.push({
      ...item,
      children: item.children ? filterMenuByRole(item.children, role, customCallback) : undefined,
    })

    return acc
  }, [])

  return filteredMenu
}

export function findMenuParents(menus: MenuItem[], item: MenuItem) {
  const parents = []
  let currentItem = item

  while (currentItem?.parent) {
    const parentItem = menus.find(menuItem => menuItem.title === currentItem.parent)
    if (parentItem) {
      parents.unshift(parentItem)
      currentItem = parentItem
    }
    else {
      break
    }
  }
  return parents
}
