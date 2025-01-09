<template>
  <button @click="goToHelloWorld">Back to Hello World</button>
  <div class="create-account">
    <h1>Create Account</h1>
    <form @submit.prevent="submitForm">
      <div>
        <label for="username">Username:</label>
        <input
          type="text"
          id="username"
          v-model="form.username"
          placeholder="Enter your username"
          required
        />
      </div>
      <div>
        <label for="password">Password:</label>
        <input
          type="password"
          id="password"
          v-model="form.password"
          placeholder="Enter your password"
          required
        />
      </div>
      <button type="submit" :disabled="loading">Create Account</button>
    </form>

    <!-- Display messages -->
    <div class="message" v-if="message">
      <p :class="{'success': success, 'error': !success}">{{ message }}</p>
    </div>
  </div>
</template>

<script>
import { useRouter } from 'vue-router'

export default {
  name: "CreateAccount",
  data() {
    return {
      form: {
        username: "",
        password: "",
      },
      loading: false, // Shows a loading spinner if needed
      message: "", // Message to display success or failure
      success: false, // Tracks if the response was successful
    };
  },
  methods: {
    // Submit form and send POST request to the backend
    async submitForm() {
      this.loading = true;
      this.message = ""; // Clear any previous messages

      try {
        // Send POST request to the backend
        const response = await fetch("http://localhost:3030/api/create_account", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(this.form),
        });

        // Parse the response as JSON
        const data = await response.json();

        // Update the message and success based on the response
        if (response.ok && data.success) {
          this.message = data.message;
          this.success = true;
        } else {
          this.message = data.message || "Failed to create account.";
          this.success = false;
        }
      } catch (error) {
        console.error("Error:", error);
        this.message = "Unable to create an account. Please try again later.";
        this.success = false;
      } finally {
        this.loading = false; // Stop loading
      }
    },
    goToHelloWorld() {
      const router = useRouter()
      router.push('/')
    },
  },
};
</script>

<style scoped>
.create-account {
  max-width: 400px;
  margin: 0 auto;
  font-family: Arial, sans-serif;
  text-align: center;
}

form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

input {
  padding: 0.5rem;
  font-size: 1rem;
  width: 100%;
  box-sizing: border-box;
}

button {
  padding: 0.75rem;
  font-size: 1rem;
  background-color: #4caf50;
  color: white;
  border: none;
  cursor: pointer;
}

button:disabled {
  background-color: #9e9e9e;
  cursor: not-allowed;
}

.message {
  margin-top: 1rem;
  font-size: 1rem;
}

.success {
  color: green;
}

.error {
  color: red;
}
</style>