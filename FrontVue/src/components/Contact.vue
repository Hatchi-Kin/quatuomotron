<template>
  <div class="contact">
    <h1>Contactez-nous</h1>
    <p>Vous pouvez nous contacter via les moyens suivants :</p>
    <ul>
      <li>Email : contact@monapplication.com</li>
      <li>Téléphone : +33 1 23 45 67 89</li>
      <li>Adresse : 123 Rue de Vue.js, 75000 Paris</li>
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
  color: #42b983;
}

ul {
  list-style-type: none;
  padding: 0;
}

li {
  font-size: 18px;
  margin: 10px 0;
}
</style>
