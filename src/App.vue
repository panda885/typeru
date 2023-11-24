<script lang="ts">
import * as state from './helpers/state'

export default {
  data() {
    return {
      input: [] as state.Letter[][],
      caretX: 0,
      caretY: 0
    }
  },
  mounted() {
    state.getInput().then(input => {
      this.input = input
      this.$nextTick(this.updateCaretPosition)
    })

    addEventListener('keydown', this.keydown)
  },
  unmounted() {
    removeEventListener('keydown', this.keydown)
  },
  methods: {
    async keydown(event: KeyboardEvent) {
      if (event.ctrlKey || event.metaKey || event.altKey) {
        return
      }

      if (event.key == "Backspace") {
        this.input = await state.backspace()
      } else if (event.key.length == 1) {
        this.input = await state.insert(event.key)
      } else {
        return
      }

      this.$nextTick(this.updateCaretPosition)
    },
    async updateCaretPosition() {
      const word = document.querySelectorAll('.word')[await state.getCursor()]
      const letters = word.querySelectorAll('.letter:not(.suggestion)')
      if (letters.length > 0) {
        const rect = letters[letters.length - 1].getBoundingClientRect()
        this.caretX = rect.x + rect.width
        this.caretY = rect.y
      } else {
        const rect = word.getBoundingClientRect()
        this.caretX = rect.x
        this.caretY = rect.y
      }
    }
  }
}
</script>

<template>
  <div class="container">
    <div class="caret" :style="{ left: caretX + 'px', top: caretY + 'px' }" />
    <div class="text">
      <span class="word" v-for="word in input">
        <span class="letter" v-for="letter in word" :class="{ [letter.status]: true }">
          {{ letter.character }}
        </span>
      </span>
    </div>
  </div>
</template>
