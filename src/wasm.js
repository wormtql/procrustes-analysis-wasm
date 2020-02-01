import * as wasm from "wasm"
import Vue from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify';

// import("wasm").then(mod => {
//     window.console.log(mod);
// });

Vue.config.productionTip = false;
Vue.prototype.wasm = wasm;
// wasm.greet();

new Vue({
    vuetify,
    render: h => h(App)
}).$mount('#app')
