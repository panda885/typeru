<script lang="ts">
import * as state from './helpers/state'

export default {
  data() {
    return {
      text: "",
      input: [] as string[],
      caretX: 0,
      caretY: 0
    }
  },
  mounted() {
    state.getText().then(text => this.text = text)
    
    addEventListener('keydown', this.keydown)

    this.updateCaretPosition()
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
    updateCaretPosition() {
      const letters = document.querySelectorAll('.word')
      if (letters.length > 0) {
        const rect = letters[letters.length - 1].getBoundingClientRect()
        this.caretX = rect.x + rect.width
        this.caretY = rect.y
      } else {
        const text = document.querySelector('.text')
        if (!text) return
        const rect = text.getBoundingClientRect()
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
        <span class="letter" v-for="letter in word">
          {{ letter }}
        </span>
      </span>
    </div>
  </div>
</template>
