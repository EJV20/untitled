   // src/router.js
   import { createRouter, createWebHistory } from 'vue-router'
   import HelloWorld from './components/HelloWorld.vue'
   import CreateAccount from './components/CreateAccount.vue'

   const routes = [
     { path: '/', name: 'HelloWorld', component: HelloWorld },
     { path: '/create-account', name: 'CreateAccount', component: CreateAccount },
   ]

   const router = createRouter({
     history: createWebHistory(),
     routes,
   })

   export default router