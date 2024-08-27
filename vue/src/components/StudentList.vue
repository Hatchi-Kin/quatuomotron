<template>
  <div class="student-list">
    <div class="controls">
      <label for="route-select" class="styled-label" :style="{ marginTop: '20px' }">Choisissez une action :</label>
      <select v-model="selectedRoute" @change="fetchData" class="styled-select">
        <option value="count">Afficher le nombre d'étudiants</option>
        <option value="people">Afficher tous les étudiants</option>
        <option value="groups/2">Générer des groupes de 2</option>
        <option value="groups/3">Générer des groupes de 3</option>
        <option v-for="(datetime, index) in datetimes" :key="index" :value="`groups_by_datetime/${datetime}`">
          Groupes enregistrés le {{ datetime }}
        </option>
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
      <button v-if="showSaveButton" @click="saveGroups" class="save-button">Enregistrer les groupes</button>
    </div>
    <ul v-else-if="students.length > 0">
      <li v-for="(student, index) in students" :key="index">
        <strong>{{ student.nom }} {{ student.prenom }}</strong> - {{ student.email }}
      </li>
    </ul>
    <p v-else-if="count !== null">
      Nombre d'étudiants : {{ count }}
    </p>
    <div v-else>
      <p>Aucune donnée disponible.</p>
    </div>
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
      datetimes: [], // New data property for unique datetimes
    };
  },
  computed: {
    showSaveButton() {
      return this.selectedRoute === "groups/2" || this.selectedRoute === "groups/3";
    }
  },
  methods: {
    async fetchData() {
      this.error = null;
      this.students = [];
      this.groups = [];
      this.count = null;
      let url = `http://localhost:8000/${this.selectedRoute}`;
      try {
        if (this.selectedRoute.startsWith("groups_by_datetime/")) {
          const datetime = this.selectedRoute.split("/")[1];
          await this.fetchGroupsByDatetime(datetime);
        } else {
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
        }
      } catch (error) {
        this.error = error.message;
      }
    },
    async saveGroups() {
      try {
        const response = await fetch('http://localhost:8000/save_groups', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(this.groups),
        });
        if (!response.ok) {
          throw new Error("Erreur lors de l'enregistrement des groupes");
        }
        alert("Groupes enregistrés avec succès !");
      } catch (error) {
        this.error = error.message;
      }
    },
    async fetchUniqueDatetimes() {
      try {
        const response = await fetch('http://localhost:8000/unique_datetimes');
        if (!response.ok) {
          throw new Error("Erreur lors de la récupération des datetimes uniques");
        }
        this.datetimes = await response.json();
      } catch (error) {
        this.error = error.message;
      }
    },
    async fetchGroupsByDatetime(datetime) {
      console.log(`Fetching groups for datetime: ${datetime}`);
      try {
        const response = await fetch(`http://localhost:8000/groups_by_datetime/${encodeURIComponent(datetime)}`);
        console.log(`Request URL: http://localhost:8000/groups_by_datetime/${encodeURIComponent(datetime)}`);
        if (!response.ok) {
          throw new Error("Erreur lors de la récupération des groupes par datetime");
        }
        const data = await response.json();
        console.log('Groups data:', data);
        this.groups = data;
      } catch (error) {
        console.error('Error fetching groups by datetime:', error);
        this.error = error.message;
      }
    },
  },
  mounted() {
    this.fetchData();
    this.fetchUniqueDatetimes(); // Fetch unique datetimes when the component is mounted
  },
};
</script>

<style scoped>
.about {
  padding: 20px;
}

h1, h2 {
  color: #42b983;
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
  color: #42b983;
  text-decoration: none;
}

a:hover {
  text-decoration: underline;
}

.save-button {
  margin-top: 20px;
  padding: 10px 20px;
  background-color: #42b983;
  color: white;
  border: none;
  cursor: pointer;
}

.save-button:hover {
  background-color: #369f6b;
}

.controls {
  margin-bottom: 20px;
}

.styled-label {
  font-size: 20px; 
  font-weight: bold; 
  display: block; 
  margin-bottom: 10px; 
}

.styled-select {
  font-size: 18px;
  padding: 10px;
  border-radius: 5px;
  border: 1px solid #ccc;
  background-color: #f9f9f9;
  width: 100%;
  max-width: 500px;
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
  color: #2f5f43; 
}
</style>