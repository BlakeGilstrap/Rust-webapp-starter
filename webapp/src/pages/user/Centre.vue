<template>
    <div id="centre">
      <mnav id="mnav"></mnav>
      <div id="content">
        <p>Welcom user centre.</p>
        <button id="submit" @click="home">GET</button><br/>
      </div>
    </div>
</template>

<script>
import axios from 'axios'
import auth from '../../utils/auth'
import Mnav from '../../components/nav/Mnav'
export default {
  name: 'centre',
  components: {
    "mnav": Mnav
  },
  data: function() {
    return {
      
    }
  },
  methods: {
    home() {
    // if(auth.user.authenticated == ture) {
        axios.get('http://localhost:8000/api/user_info', auth.getAuthHeader() )
        .then((response) => {
          if(response.data.status === "200") {
            // console.log(response.data.user_info)
          }
          window.location.reload ( true ); 
          this.$router.push('/')
        })
        .catch((e) => {
          console.log(e)
        })
    // }else{
    //     this.$router.push('/kxco/user_id')
    // }
    }
  }
}
</script>

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