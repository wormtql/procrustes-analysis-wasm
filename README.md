# procrustes-analysis-wasm

## build web assembly
make sure you have rust wasm toolchain installed
```bash
cd wasm/rust-wasm
wasm-pack build
```
and you'll notice a pkg directory generated

## install dependencies
```bash
npm install
```
the wasm dependency is also included

### Compiles and hot-reloads for development
```
npm run serve
```

### Compiles and minifies for production
```
npm run build
```

### Lints and fixes files
```
npm run lint
```

### Customize configuration
See [Configuration Reference](https://cli.vuejs.org/config/).
