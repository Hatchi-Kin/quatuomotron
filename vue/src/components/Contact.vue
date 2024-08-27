<template>
  <div class="contact">
    <h1>Contactez-nous</h1>
    <p>Vous pouvez nous contacter via notre dépôt GitHub :</p>
    <ul>
      <li>
        <a href="https://github.com/Hatchi-Kin/quatuomotron" target="_blank" rel="noopener noreferrer">
          GitHub Repository
        </a>
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  name: 'Contact',
};
</script>

<style scoped>
.contact {
  padding: 20px;
}

h1 {
  color: #2f5f43;
}

ul {
  list-style-type: none;
  padding: 0;
}

li {
  font-size: 18px;
  margin: 10px 0;
}

a {
  color: #45738b;
  text-decoration: none;
}

a:hover {
  text-decoration: underline;
}
</style>