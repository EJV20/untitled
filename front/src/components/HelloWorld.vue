<script setup>
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

function goToCreateAccount() {
  router.push('/create-account')
}

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


</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
