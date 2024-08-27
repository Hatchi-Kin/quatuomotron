import { createApp } from 'vue';
import App from './App.vue';
import router from './../src/router/router.js';

const app = createApp(App);

// Si vous souhaitez désactiver le message de tip en production
app.config.productionTip = false;

app.use(router);
app.mount('#app');


