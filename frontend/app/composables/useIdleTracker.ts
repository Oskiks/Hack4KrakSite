import type { UseIdleReturn } from '@vueuse/core'
import { useIdle } from '@vueuse/core'

export default function useIdleTracker(
  timeout: number = 60000,
  options: Omit<Parameters<typeof useIdle>[1], 'initialState'> = {},
): UseIdleReturn {
  return useIdle(timeout, { initialState: false, ...options })
}
