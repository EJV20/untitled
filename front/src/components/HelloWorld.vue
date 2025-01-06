<script setup>
import { onMounted, ref } from 'vue'

defineProps({
  msg: String,
})

const count = ref(0)

const message = ref('')

onMounted(async () => {
  try {
    const response = await fetch('http://localhost:3030/api/hello')
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }
    const data = await response.json()
    message.value = data.message || 'No message received'
  } catch (error) {
    console.error('Error fetching data:', error)
    message.value = 'Failed to fetch data'
  }
})
</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <button type="button" @click="count++">count is {{ count }}</button>
    <p>
      Edit
      <code>components/HelloWorld.vue</code> to test HMR
    </p>
  </div>

  <p>
    Check out
    <a href="https://vuejs.org/guide/quick-start.html#local" target="_blank"
      >create-vue</a
    >, the official Vue + Vite starter
  </p>
  <p>
    Learn more about IDE Support for Vue in the
    <a
      href="https://vuejs.org/guide/scaling-up/tooling.html#ide-support"
      target="_blank"
      >Vue Docs Scaling up Guide</a
    >.
  </p>
  <p class="read-the-docs">Click on the Vite and Vue logos to learn more</p>

  <h1>API Response</h1>
  <p>{{ message }}</p>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
