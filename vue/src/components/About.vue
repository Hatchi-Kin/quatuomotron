<template>
  <div class="about">
    <h1>À propos de nous</h1>
    <p>Bienvenue sur la page À propos de notre application.</p>
    <p>
      Cette application est conçue pour vous offrir une gestion simple et efficace de vos listes d'étudiants. Vous pouvez afficher la liste complète des étudiants, voir le nombre total d'étudiants, et générer des groupes aléatoires de 2 ou 3 étudiants.
    </p>
    <h2>Technologies Utilisées</h2>
    <ul>
      <li><strong>Front-end :</strong> Vue.js 3</li>
      <li><strong>Build et Développement :</strong> Vite</li>
      <li><strong>Back-end :</strong> Rust</li>
      <li><strong>Navigation :</strong> Vue Router</li>
    </ul>
    <h2>Fonctionnalités</h2>
    <ul>
      <li>Afficher la liste complète des étudiants</li>
      <li>Voir le nombre total d'étudiants</li>
      <li>Générer des groupes aléatoires de 2 ou 3 étudiants</li>
    </ul>
    <h2>Objectif du Projet</h2>
    <p>
      Ce projet a pour objectif de fournir une interface moderne et performante pour gérer et classer des listes d'étudiants. Il permet de simplifier l'organisation des étudiants en groupes pour des projets ou des activités.
    </p>
    <h2>Contributeurs</h2>
    <p>
      Ce projet a été développé par une équipe passionnée de développeurs utilisant les dernières technologies pour offrir la meilleure expérience utilisateur possible.
    </p>
    <h2>Liens Utiles</h2>
    <p>
      <strong>Lien vers le dépôt GitHub :</strong> <a href="https://github.com/Hatchi-Kin/quatuomotron" target="_blank">Voir le repository sur GitHub</a>
    </p>
  </div>
</template>

<script>
export default {
  name: 'About',
};
</script>

<style scoped>
.about {
  padding: 20px;
}

h1, h2 {
  color: #2f5f43;
}

p, ul {
  font-size: 18px;
  line-height: 1.6;
}

ul {
  list-style-type: disc;
  margin-left: 20px;
}

a {
  color: #45738b;
  text-decoration: none;
}

a:hover {
  text-decoration: underline;
}
</style>