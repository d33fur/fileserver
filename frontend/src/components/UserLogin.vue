<template>
  <div>
    <h1>Login</h1>
    <form @submit.prevent="login">
      <div>
        <label for="username">Username:</label>
        <input v-model="username" type="text" id="username" required />
      </div>
      <div>
        <label for="password">Password:</label>
        <input v-model="password" type="password" id="password" required />
      </div>
      <button type="submit">Login</button>
    </form>
  </div>
</template>

<script>
export default {
  name: 'UserLogin',  // Изменено с 'Login' на 'UserLogin'
  data() {
    return {
      username: '',
      password: ''
    }
  },
  methods: {
    async login() {
      const response = await fetch('http://127.0.0.1:8080/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          username: this.username,
          password: this.password
        })
      });

      const data = await response.json();

      if (response.ok) {
        alert('Login successful');
        this.$router.push('/');
      } else {
        alert('Login failed: ' + data.message);
      }
    }
  }
}
</script>
