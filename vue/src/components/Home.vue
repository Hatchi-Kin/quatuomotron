<template>
  <div class="home">
    <h1>Bienvenue sur notre projet</h1>
    <p>Cette application est construite avec les technologies suivantes :</p>
    <ul>
      <li>Vue.js 3 pour le front-end</li>
      <li>Vite pour le build et le développement</li>
      <li>Rust pour le backend</li>
      <li>Vue Router pour la navigation</li>
    </ul>
    <p>
      <strong>But du projet :</strong> Ce projet a pour objectif de faciliter la création de groupes d'étudiants de manière aléatoire et de les enregistrer pour une utilisation future.
    </p>
    <p>
      <strong>But inavoué du projet :</strong> Le but réel du projet est de convaincre un employeur (préférablement suisse et préférablement dans des locaus tout près d'un lac) d'offir aux 4 développeurs de cette application un contrat en CDI et un salaire annuel de 100k €.
    </p>
      
    <p>
      <strong>Lien vers le dépôt GitHub :</strong> <a href="https://github.com/Hatchi-Kin/quatuomotron" target="_blank">Voir le repository sur GitHub</a>
    </p>
  </div>
</template>

<script>
export default {
  name: 'Home',
};
</script>

<style scoped>
.home {
  padding: 20px;
}

h1 {
  color: #2f5f43;
}

ul {
  list-style-type: disc;
  padding-left: 20px;
}

li {
  font-size: 18px;
  margin-bottom: 10px;
}

p {
  font-size: 18px;
  line-height: 1.6;
}

a {
  color: #45738b;
  text-decoration: none;
}

a:hover {
  text-decoration: underline;
}
</style>
