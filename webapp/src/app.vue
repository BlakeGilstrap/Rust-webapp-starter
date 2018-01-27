<template>
  <div id="app">
    <router-view></router-view>
  </div>
</template>

<script>
export default {
  name: 'app'
}
</script>

<style>
* {
    padding:0px; margin:0px;
    box-sizing: border-box;
}
a {
  text-decoration: none;
  color: inherit;
}
a:hover { color:#0c9e2b; }

ul, ol, li {
    list-style: none;
}
</style>
