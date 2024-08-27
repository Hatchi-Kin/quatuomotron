<template>
  <div class="student-list">

    <div class="controls">
      <label for="route-select" class="styled-label">Choisissez une action :</label>
      <select v-model="selectedRoute" @change="fetchData" class="styled-select">
        <option value="count">Afficher le nombre d'étudiants</option>
        <option value="people">Afficher tous les étudiants</option>
        <option value="groups/2">Générer des groupes de 2</option>
        <option value="groups/3">Générer des groupes de 3</option>
      </select>
    </div>

    <div v-if="error" class="error">
      <p>Erreur : {{ error }}</p>
    </div>

    <div v-if="groups.length > 0">
      <div v-for="(group, groupIndex) in groups" :key="groupIndex" class="group">
        <h3 class="group-header">Groupe {{ groupIndex + 1 }}</h3>
        <ul>
          <li v-for="(student, studentIndex) in group.members" :key="studentIndex">
            <strong>{{ student.nom }} {{ student.prenom }}</strong> - {{ student.email }}
          </li>
        </ul>
      </div>
    </div>

    <ul v-else-if="students.length > 0">
      <li v-for="(student, index) in students" :key="index">
        <strong>{{ student.nom }} {{ student.prenom }}</strong> - {{ student.email }}
      </li>
    </ul>

    <p v-else-if="count !== null">
      Nombre d'étudiants : {{ count }}
    </p>

    <p v-else>
      Aucune donnée disponible.
    </p>
  </div>
</template>

<script>
export default {
  name: "StudentList",
  data() {
    return {
      students: [],
      groups: [],
      count: null,
      selectedRoute: "count",
      error: null,
    };
  },
  methods: {
    async fetchData() {
      this.error = null;
      this.students = [];
      this.groups = [];
      this.count = null;

      let url = `http://localhost:8000/${this.selectedRoute}`;

      try {
        const response = await fetch(url);
        if (!response.ok) {
          throw new Error("Erreur lors de la récupération des données");
        }

        const data = await response.json();

        if (this.selectedRoute === "count") {
          this.count = data;
        } else if (this.selectedRoute.startsWith("groups/")) {
          this.groups = data;
        } else {
          this.students = data;
        }
      } catch (error) {
        this.error = error.message;
      }
    },
  },
  mounted() {
    this.fetchData();
  },
};
</script>

<style scoped>
.controls {
  margin-bottom: 20px;
}

.styled-label {
  font-size: 20px; /* Increase the font size */
  font-weight: bold; /* Make the label bold */
  display: block; /* Ensure the label is displayed as a block element */
  margin-bottom: 10px; /* Add some space below the label */
}

.styled-select {
  font-size: 18px;
  padding: 10px;
  border-radius: 5px;
  border: 1px solid #ccc;
  background-color: #f9f9f9;
  width: 100%;
  max-width: 400px;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
}

.error {
  color: red;
}

.student-list ul {
  list-style-type: none;
  padding: 0;
}

.student-list li {
  font-size: 18px;
  margin: 5px 0;
}

.group {
  margin-bottom: 20px;
}

.group-header {
  margin-bottom: 10px;
  color: #2f5f43; /* Set the color to #42b983 green */
}
</style>