<script setup lang="ts">
interface PigeonData {
  id: number
  x: number
  y: number
  size: number
  rotation: number
  enterDelay: number
  enterDuration: number
  enterFromX: number
  enterFromY: number
  fleeX: number
  fleeY: number
  fleeDuration: number
}

type Phase = 'hidden' | 'entering' | 'landed' | 'fleeing'

const IDLE_TIMEOUT = 30000
const PIGEON_COUNT = 20
const DANGER_RADIUS = 200

const route = useRoute()
const { idle } = useIdleTracker(IDLE_TIMEOUT)

const phase = ref<Phase>('hidden')
const pigeons = ref<PigeonData[]>([])
let landTimer: ReturnType<typeof setTimeout> | null = null
let hideTimer: ReturnType<typeof setTimeout> | null = null
let lastMX = -1000
let lastMY = -1000

function findPerchSpots(): Array<{ x: number, y: number, w: number, max: number }> {
  const w = window.innerWidth
  const h = window.innerHeight

  const side = Math.random() < 0.5 ? 'left' : 'right'

  const tags = [
    { sel: 'h1', max: 10 },
    { sel: 'h2', max: 4 },
    { sel: 'h3, h4', max: 3 },
    { sel: '[class*="font-pixelify"]:not(nav *)', max: 3 },
  ]

  const used = new Set<Element>()
  const spots: Array<{ x: number, y: number, w: number, max: number }> = []

  for (const { sel, max } of tags) {
    for (const el of document.querySelectorAll<HTMLElement>(sel)) {
      if (used.has(el))
        continue

      let parent = el.parentElement
      let skip = false
      while (parent && parent !== document.body) {
        if (used.has(parent)) {
          skip = true
          break
        }
        parent = parent.parentElement
      }
      if (skip)
        continue

      const rect = el.getBoundingClientRect()
      if (rect.width < 10 || rect.height < 5)
        continue
      if (rect.top < 100 || rect.bottom > h - 10)
        continue
      const centerX = rect.left + rect.width / 2
      if (side === 'left' && centerX > w * 0.55)
        continue
      if (side === 'right' && centerX < w * 0.45)
        continue

      used.add(el)
      spots.push({ x: centerX, y: rect.top, w: rect.width, max })
    }
  }

  const bx = side === 'left' ? w * 0.2 : w * 0.8
  spots.push({ x: bx, y: h - 20, w: w * 0.25, max: 8 })
  spots.push({ x: bx + (side === 'left' ? 1 : -1) * w * 0.1, y: h - 70, w: w * 0.15, max: 6 })

  return spots
}

function pickAllLandings(): Array<{ x: number, y: number }> {
  const spots = findPerchSpots()
  const landings: Array<{ x: number, y: number }> = []
  const w = window.innerWidth
  const h = window.innerHeight

  if (spots.length === 0) {
    for (let i = 0; i < PIGEON_COUNT; i++)
      landings.push({ x: 30 + Math.random() * (w - 60), y: h - 70 - Math.random() * 120 })
    return landings
  }

  const shuffled = [...spots].sort(() => Math.random() - 0.5)
  const counts: number[] = Array.from({ length: shuffled.length }).fill(0) as number[]
  const SPREAD = 0.8

  for (let i = 0; i < PIGEON_COUNT; i++) {
    let best = -1
    let bestCount = Infinity
    for (let j = 0; j < counts.length; j++) {
      if ((counts[j]!) < shuffled[j]!.max && (counts[j]!) < bestCount) {
        bestCount = counts[j]!
        best = j
      }
    }

    if (best === -1) {
      landings.push({ x: 30 + Math.random() * (w - 60), y: h - 70 - Math.random() * 120 })
    } else {
      const nth = counts[best]!
      counts[best] = nth + 1
      const spot = shuffled[best]!
      const offset = nth % 2 === 0
        ? -(Math.random() * spot.w * SPREAD * 0.4 + spot.w * 0.1)
        : Math.random() * spot.w * SPREAD * 0.4 + spot.w * 0.1
      landings.push({
        x: spot.x + offset,
        y: spot.y,
      })
    }
  }

  return landings
}

function spawn() {
  if (phase.value !== 'hidden')
    return
  const w = window.innerWidth
  const h = window.innerHeight
  const landings = pickAllLandings()
  const items: PigeonData[] = []

  for (let i = 0; i < PIGEON_COUNT; i++) {
    const landing = landings[i]!
    const size = 34 + Math.random() * 24
    const landingX = landing.x
    const landingY = landing.y - size

    items.push({
      id: i,
      x: landingX,
      y: landingY,
      size,
      rotation: -5 + Math.random() * 10,
      enterDelay: Math.random() * 2500,
      enterDuration: 1400 + Math.random() * 2000,
      enterFromX: -200 + Math.random() * 400,
      enterFromY: -(landingY + 150 + Math.random() * 250),
      fleeX: -w * (0.2 + Math.random() * 0.5) + w / 2,
      fleeY: -(h + 200 + Math.random() * 300),
      fleeDuration: 600 + Math.random() * 700,
    })
  }

  pigeons.value = items
  phase.value = 'entering'

  const maxAnim = Math.max(...items.map(p => p.enterDelay + p.enterDuration))
  landTimer = setTimeout(() => {
    if (phase.value === 'entering') {
      phase.value = 'landed'
      if (isPosNearPigeons(lastMX, lastMY))
        scare()
    }
  }, maxAnim + 100)
}

function isPosNearPigeons(mx: number, my: number) {
  if (phase.value !== 'landed')
    return false
  const r2 = DANGER_RADIUS * DANGER_RADIUS
  for (const p of pigeons.value) {
    const cx = p.x + p.size / 2
    const cy = p.y + p.size / 2
    const dx = mx - cx
    const dy = my - cy
    if (dx * dx + dy * dy < r2)
      return true
  }
  return false
}

function onFeedPigeons() {
  spawn()
}

function scare() {
  if (phase.value === 'fleeing' || phase.value === 'hidden')
    return
  if (landTimer != null)
    clearTimeout(landTimer)
  phase.value = 'fleeing'

  hideTimer = setTimeout(() => {
    phase.value = 'hidden'
    pigeons.value = []
  }, 2000)
}

function onMouseMove(e: MouseEvent) {
  lastMX = e.clientX
  lastMY = e.clientY
  if (isPosNearPigeons(lastMX, lastMY))
    scare()
}

function onClick(e: MouseEvent) {
  lastMX = e.clientX
  lastMY = e.clientY
  if (isPosNearPigeons(lastMX, lastMY))
    scare()
}

function onGlobalInteraction() {
  if (phase.value === 'entering' || phase.value === 'landed')
    scare()
}

watch(idle, (value) => {
  if (value && route.path.startsWith('/panel'))
    spawn()
})

onMounted(() => {
  window.addEventListener('mousemove', onMouseMove, { passive: true })
  window.addEventListener('click', onClick, { passive: true })
  window.addEventListener('keydown', onGlobalInteraction, { passive: true })
  window.addEventListener('touchstart', onGlobalInteraction, { passive: true })
  window.addEventListener('wheel', onGlobalInteraction, { passive: true })
  window.addEventListener('h4k:feed-pigeons', onFeedPigeons)
})

onUnmounted(() => {
  if (landTimer != null)
    clearTimeout(landTimer)
  if (hideTimer != null)
    clearTimeout(hideTimer)
  window.removeEventListener('mousemove', onMouseMove)
  window.removeEventListener('click', onClick)
  window.removeEventListener('keydown', onGlobalInteraction)
  window.removeEventListener('touchstart', onGlobalInteraction)
  window.removeEventListener('wheel', onGlobalInteraction)
  window.removeEventListener('h4k:feed-pigeons', onFeedPigeons)
})
</script>

<template>
  <Teleport to="body">
    <div v-if="phase !== 'hidden'" class="pigeon-container" data-pigeon-easter-egg>
      <div
        v-for="p in pigeons"
        :key="p.id"
        class="pigeon"
        :class="phase"
        :style="{
          '--px': `${p.x}px`,
          '--py': `${p.y}px`,
          '--size': `${p.size}px`,
          '--rotation': `${p.rotation}deg`,
          '--enter-delay': `${p.enterDelay}ms`,
          '--enter-duration': `${p.enterDuration}ms`,
          '--enter-x': `${p.enterFromX}px`,
          '--enter-y': `${p.enterFromY}px`,
          '--flee-x': `${p.fleeX}px`,
          '--flee-y': `${p.fleeY}px`,
          '--flee-duration': `${p.fleeDuration}ms`,
        }"
      >
        <img
          src="/img/pigeon.png"
          alt=""
          class="pigeon-img"
          aria-hidden="true"
        >
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.pigeon-container {
  position: fixed;
  inset: 0;
  z-index: 9999;
  pointer-events: none;
}

.pigeon {
  position: absolute;
  left: var(--px);
  top: var(--py);
  width: var(--size);
  will-change: transform;
  filter: drop-shadow(0 0 6px rgba(255 255 255 / 0.25)) drop-shadow(0 0 2px rgba(255 255 255 / 0.4));
}

.pigeon-img {
  display: block;
  width: 100%;
  height: auto;
}

.pigeon-bagel {
  position: absolute;
  top: -8px;
  right: -8px;
  width: 40%;
  height: auto;
  pointer-events: none;
  filter: drop-shadow(0 0 4px rgba(255 255 255 / 0.5));
  animation: bagel-bounce 1.5s ease-in-out infinite;
}

@keyframes bagel-bounce {
  0%,
  100% {
    transform: translateY(0) rotate(0deg);
  }
  50% {
    transform: translateY(-4px) rotate(15deg);
  }
}

.pigeon.entering {
  animation: pigeon-fly-in var(--enter-duration) var(--enter-delay) ease-out both;
}

@keyframes pigeon-fly-in {
  from {
    transform: translateY(var(--enter-y)) translateX(var(--enter-x)) rotate(calc(var(--rotation) - 20deg));
    opacity: 0;
  }
  to {
    transform: translateY(0) translateX(0) rotate(var(--rotation));
    opacity: 1;
  }
}

.pigeon.landed {
  transform: rotate(var(--rotation));
  animation: pigeon-idle 2.5s ease-in-out infinite;
}

@keyframes pigeon-idle {
  0%,
  100% {
    transform: rotate(var(--rotation)) translateY(0);
  }
  50% {
    transform: rotate(var(--rotation)) translateY(-3px);
  }
}

.pigeon.fleeing {
  animation: pigeon-flee var(--flee-duration) ease-in forwards;
}

@keyframes pigeon-flee {
  from {
    transform: translateY(0) translateX(0) rotate(var(--rotation));
    opacity: 1;
  }
  to {
    transform: translateY(var(--flee-y)) translateX(var(--flee-x)) rotate(calc(var(--rotation) + 25deg));
    opacity: 0;
  }
}
</style>
