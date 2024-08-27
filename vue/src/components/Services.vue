<template>
  <div class="services">
    <h1>Nos Services</h1>
    <p>Ici, vous pouvez afficher la liste des étudiants, leur nombre et créer des groupes de 2 ou de 3 aléatoirement :</p>
    <student-list></student-list>
  </div>
</template>

<script>
import StudentList from './StudentList.vue';

export default {
  name: 'Services',
  components: {
    StudentList,
  },
};
</script>

<style scoped>
.services {
  padding: 20px;
}

h1 {
  color: #42b983;
}

p {
  font-size: 18px;
  margin-bottom: 20px;
}
</style>
