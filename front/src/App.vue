<script setup>
import HelloWorld from './components/HelloWorld.vue'
</script>

<template>
  <nav class="navbar">
    <RouterLink to="/" class="nav-item">Home</RouterLink>
    <RouterLink to="/Login" class="nav-item">Login</RouterLink>
    <RouterLink to="/create-account" class="nav-item">Create Account</RouterLink>
  </nav>
  <main class="main-content">
    <router-view />
  </main>
</template>

<style scoped>
.navbar {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 1rem;
  background: linear-gradient(to right, #6a11cb, #2575fc); /* Fun gradient */
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.nav-item {
  margin: 0 1.5rem;
  font-size: 1.2rem;
  color: white;
  text-decoration: none;
  font-family: 'Montserrat', sans-serif;
  font-weight: bold;
  position: relative;
  cursor: pointer;
  transition: color 0.3s ease;
}

.nav-item:hover {
  color: #ffdd57; /* Highlight on hover */
}

/* Add underline animation */
.nav-item::after {
  content: "";
  position: absolute;
  width: 0;
  height: 2px;
  background-color: #ffdd57; /* Same as hover color */
  left: 50%;
  bottom: -3px;
  transition: width 0.3s ease, left 0.3s ease;
}

.nav-item:hover::after {
  width: 100%;
  left: 0;
}

/* Main Content Styling */
.main-content {
  padding: 2rem;
  font-family: 'Roboto', sans-serif;
  color: #333;
  line-height: 1.6;
  text-align: center;
}

/* Add some spacing for smaller devices */
@media (max-width: 768px) {
  .navbar {
    flex-wrap: wrap;
  }

  .nav-item {
    margin: 0.5rem 0.5rem;
  }
}
</style>
