<template>
  <div  id="about">
      <mnav id="mnav"></mnav>
      <div id="content">
          <p><strong>About</strong> Thos is the about page.</p> <br>
      </div><br>
  </div>
</template>

<script>
import Mnav from '../../components/nav/Mnav'
export default {
  name: 'about',
  components: {
    "mnav": Mnav
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
@media only screen and (max-width: 600px) {
    #content  {
      margin: 0.5rem auto;
      width: 95%;
  }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
    #content  {
      margin: 0 auto;
      width: 72%;
      padding-top: 77px;
  }
}
@media only screen and (min-width: 1000px) {
  #content  {
      margin: 0 auto;
      width: 66%;
      padding-top: 77px;
  }
}
</style>