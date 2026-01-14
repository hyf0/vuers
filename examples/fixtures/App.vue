<template>
  <div class="app">
    <h1>{{ title }}</h1>
    <p>Count: {{ count }}</p>
    <button @click="increment">+1</button>
    <ChildComponent :message="greeting" />
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import ChildComponent from './ChildComponent.vue'

const count = ref(0)
const title = 'Hello Vue'

const greeting = computed(() => `Count is ${count.value}`)

function increment() {
  count.value++
}
</script>

<style scoped>
.app {
  font-family: sans-serif;
  padding: 20px;
}

h1 {
  color: #42b883;
}

button {
  background: #42b883;
  color: white;
  border: none;
  padding: 8px 16px;
  cursor: pointer;
}
</style>
