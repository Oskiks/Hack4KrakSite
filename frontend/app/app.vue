<script setup lang="ts">
import { setFavicon } from '~/utils/setFavicon'

useOgImage()

let originalTitle = ''

// --- Context menu ---
const ctx = reactive({ show: false, x: 0, y: 0 })

function onCtxMenu(e: MouseEvent) {
  const t = e.target as HTMLElement
  if (t.tagName === 'INPUT' || t.tagName === 'TEXTAREA' || t.isContentEditable)
    return
  e.preventDefault()
  ctx.x = e.clientX
  ctx.y = e.clientY
  ctx.show = true
}

function feedPigeon() {
  ctx.show = false
  window.dispatchEvent(new CustomEvent('h4k:feed-pigeons'))
}

function copyText() {
  const sel = window.getSelection()?.toString()
  if (sel)
    navigator.clipboard.writeText(sel).catch(() => {})
  ctx.show = false
}

// --- O→🫓 counter ---
const O_TEST_RE = /O/i
const O_REPLACE_RE = /O/gi

function replaceOWithObwarzanek() {
  const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT)
  const toReplace: Text[] = []
  for (let n = walker.nextNode() as Text | null; n !== null; n = walker.nextNode() as Text | null) {
    if (n.textContent && O_TEST_RE.test(n.textContent)) {
      toReplace.push(n)
    }
  }
  for (const textNode of toReplace) {
    const parent = textNode.parentElement
    if (!parent)
      continue
    const html = textNode.textContent!.replace(
      O_REPLACE_RE,
      '<img src="/img/obwarzanek.png" class="obwarzanek-inline" alt="O">',
    )
    const wrapper = document.createElement('span')
    wrapper.innerHTML = html
    const frag = document.createDocumentFragment()
    while (wrapper.firstChild) frag.appendChild(wrapper.firstChild)
    parent.replaceChild(frag, textNode)
  }
}

const VISIT_KEY = 'h4k-obwarzanek-count'

onMounted(() => {
  setFavicon()

  // --- Every 10th load: O → 🫓 ---
  const count = (Number.parseInt(localStorage.getItem(VISIT_KEY) || '0', 10) + 1) % 10
  localStorage.setItem(VISIT_KEY, String(count))
  if (count === 0)
    setTimeout(replaceOWithObwarzanek, 500)

  // --- Right-click context menu ---
  window.addEventListener('contextmenu', onCtxMenu)
  document.addEventListener('click', () => {
    ctx.show = false
  })
  window.addEventListener('scroll', () => {
    ctx.show = false
  }, { passive: true })
  window.addEventListener('resize', () => {
    ctx.show = false
  }, { passive: true })

  // --- Tab title changer ---
  document.addEventListener('visibilitychange', () => {
    if (document.hidden) {
      originalTitle = document.title
      document.title = '❗ WRACAJŻE TUTAJ ❗ 🥨'
    } else {
      document.title = originalTitle
    }
  })

  // --- F12 + Dzwon Zygmunta + Escape ---
  window.addEventListener('keydown', (e) => {
    if (e.key === 'F12') {
      // eslint-disable-next-line no-console
      console.log('%c🫓 Kupiłeś już obwarzanka?', 'font-size: 18px; color: #D08700; font-weight: bold;')
    }

    if (e.key === 'Escape')
      ctx.show = false
  })
})
</script>

<template>
  <NuxtRouteAnnouncer />
  <NuxtLoadingIndicator color="var(--ui-primary)" :height="2" />

  <UApp>
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </UApp>

  <PigeonEasterEgg />

  <!-- Right-click context menu -->
  <Teleport to="body">
    <Transition name="fade">
      <div
        v-if="ctx.show"
        class="fixed z-[99999] min-w-44 rounded-xl border border-[#333] bg-[#1e1e1e] py-1.5 shadow-2xl"
        :style="{ left: `${ctx.x}px`, top: `${ctx.y}px` }"
        @click.stop
        @contextmenu.prevent
      >
        <div class="px-3 pb-1 text-[10px] tracking-widest text-[#666] uppercase">
          Akcje
        </div>
        <button
          class="flex w-full items-center gap-2 px-3 py-1.5 text-left text-sm text-[#fafafa] hover:bg-[#d08700]/20"
          @click="copyText"
        >
          <span class="text-xs">📋</span> Kopiuj
        </button>
        <hr class="mx-2 my-1 border-[#333]">
        <button
          class="flex w-full items-center gap-2 px-3 py-1.5 text-left text-sm font-semibold text-[#d08700] hover:bg-[#d08700]/20"
          @click="feedPigeon"
        >
          <span class="text-xs">🕊️</span> Nakarm gołębia
        </button>
      </div>
    </Transition>
  </Teleport>
</template>

<style>
.obwarzanek-inline {
  display: inline;
  width: 1em;
  height: 1em;
  vertical-align: text-bottom;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.25s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
