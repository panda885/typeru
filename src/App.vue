<script lang="ts">
import * as state from './helpers/state'

export default {
  data() {
    return {
      text: "",
      input: "",
      caretPosition: 0,
    }
  },
  mounted() {
    state.getText().then(text => this.text = text)
    
    addEventListener('keydown', this.keydown)
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

      this.$nextTick(() => {
        this.caretPosition = [...document.querySelectorAll('.letter.correct, .letter.mistake')]
          .map(el => el.clientWidth)
          .reduce((a, b) => a + b, 0)
      })
    }
  }
}
</script>

<template>
  <div class="container">
    <div class="caret" :style="{ left: caretPosition + 'px' }" />
    <div class="text">
      <span class="letter" v-for="(letter, index) in text" :class="{ correct: input.length > index && letter == input[index], mistake: input.length > index && letter != input[index], space: letter == ' ' }">
        {{ letter.replace(" ", "..") }}
      </span>
    </div>
  </div>
</template>
