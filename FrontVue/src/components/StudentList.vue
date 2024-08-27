` <template>
  <div class="student-list">
    <h2>Liste des étudiants</h2>

    <div class="controls">
      <label for="route-select">Choisissez une action :</label>
      <select v-model="selectedRoute" @change="fetchData">
        <option value="people">Afficher tous les étudiants</option>
        <option value="count">Afficher le nombre d'étudiants</option>
        <option value="groups/2">Générer des groupes de 2</option>
        <option value="groups/3">Générer des groupes de 3</option>
      </select>
    </div>

    <div v-if="error" class="error">
      <p>Erreur : {{ error }}</p>
    </div>

    <ul v-if="students.length > 0">
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
      count: null,
      selectedRoute: "people",
      error: null,
    };
  },
  methods: {
    async fetchData() {
      this.error = null;
      this.students = [];
      this.count = null;

      let url = `http://localhost:8000/${this.selectedRoute}`;

      try {
        const response = await fetch(url);
        if (!response.ok) {
          throw new Error("Erreur lors de la récupération des données");
        }

        if (this.selectedRoute === "count") {
          const data = await response.json();
          this.count = data;
        } else {
          const data = await response.json();
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
</style>
`