export function useIdleTracker(timeout = 30000) {
  const isIdle = ref(false)
  let idleTimer: ReturnType<typeof setTimeout> | null = null

  function reset() {
    if (idleTimer != null)
      clearTimeout(idleTimer)
    isIdle.value = false
    idleTimer = setTimeout(() => {
      isIdle.value = true
    }, timeout)
  }

  function start() {
    reset()
    window.addEventListener('mousemove', reset, { passive: true })
    window.addEventListener('click', reset, { passive: true })
    window.addEventListener('keydown', reset, { passive: true })
    window.addEventListener('touchstart', reset, { passive: true })
    window.addEventListener('scroll', reset, { passive: true })
    window.addEventListener('wheel', reset, { passive: true })
  }

  function stop() {
    if (idleTimer != null)
      clearTimeout(idleTimer)
    idleTimer = null
    isIdle.value = false
    window.removeEventListener('mousemove', reset)
    window.removeEventListener('click', reset)
    window.removeEventListener('keydown', reset)
    window.removeEventListener('touchstart', reset)
    window.removeEventListener('scroll', reset)
    window.removeEventListener('wheel', reset)
  }

  onMounted(() => start())
  onUnmounted(() => stop())

  return { isIdle }
}
