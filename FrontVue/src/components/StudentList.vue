<template>
  <div>
    <h1>Liste des Élèves</h1>
    <ul>
      <li v-for="student in students" :key="student.id">
        {{ student.nom }} - {{ student.prenom }} - {{ student.email }}
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  data() {
    return {
      students: []
    };
  },
  created() {
    this.fetchStudents();
  },
  methods: {
    async fetchStudents() {
      try {
        const response = await fetch('http://localhost:8000/api/students');  // Assurez-vous que l'URL correspond à votre backend en Rust
        const data = await response.json();
        this.students = data;
      } catch (error) {
        console.error('Error fetching students:', error);
      }
    }
  }
};
</script>

<style scoped>
/* Ajoutez vos styles ici */
</style>
