import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"
export * from "lodash-es"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

