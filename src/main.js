// import Vue from 'vue'
// import App from './App.vue'
// import vuetify from './plugins/vuetify';
//
// import("wasm").then(mod => {
//   window.console.log(mod);
// });
//
// Vue.config.productionTip = false;
//
// new Vue({
//   vuetify,
//   render: h => h(App)
// }).$mount('#app')
import("./wasm.js").catch(e => window.console.log(e));