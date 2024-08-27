<template>
  <nav class="navbar">
    <div class="navbar-container">
      <div class="navbar-brand">
        <a href="/">Quatuomotron</a>
      </div>
      <ul class="navbar-links">
        <li>
          <router-link to="/">Accueil</router-link>
        </li>
        <li>
          <router-link to="/services">Services</router-link>
        </li>
        <li>
          <router-link to="/about">À propos</router-link>
        </li>
        <li>
          <router-link to="/contact">Contact</router-link>
        </li>
      </ul>
    </div>
  </nav>
</template>

<script>
export default {
  name: 'Navbar',
};
</script>

<style scoped>
@import url("https://fonts.googleapis.com/css2?family=Rakkas&display=swap");

.navbar {
  background-color: #2f5f43;
  padding: 10px 20px;
}

.navbar-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.navbar-brand a {
  color: white;
  font-size: 42px;
  text-decoration: none;
  font-family: 'Rakkas', cursive;
}

.navbar-links {
  list-style: none;
  display: flex;
  gap: 15px;
}

.navbar-links li {
  margin: 0;
}

.navbar-links a {
  color: white;
  text-decoration: none;
  font-size: 18px;
}

.navbar-links a:hover {
  text-decoration: underline;
}
</style>