<template>
    <div id="center">
      <mnav id="mnav"></mnav>
      <div id="content">
        <div id="user-center"><p>Uaer Center</p></div>
        <p>Welcom user center.</p>
        <p>email : {{ email }}</p>
        <p>username ：{{ username }}</p>
        <p>created_time : {{ created_time }}</p>
        <button id="submit" @click="home">Home</button><br/>
      </div>
    </div>
</template>

<script>
import axios from 'axios'
import auth from '../../utils/auth'
import Mnav from '../../components/nav/Mnav'
export default {
  name: 'center',
  components: {
    "mnav": Mnav
  },
  data: function() {
    return {
      email: '',
      username: '',
      created_time: ''
    }
  },
  mounted: function() {
    // if(auth.user.authenticated == ture) {
        axios.get('http://localhost:8000/api/user_info', auth.getAuthHeader() )
        .then((response) => {
          // if(response.data.status === "200") {
            console.log(response.data.user_info)
            this.email =  response.data.user_info.email
            this.username =  response.data.user_info.username
            this.created_time =  response.data.user_info.created_time
          // }
          // this.$router.push('/')
        })
        .catch((e) => {
          console.log(e)
        })
    // }else{
    //     this.$router.push('/kxco/user_id')
    // }
  },
  methods: {
    home() {
        window.location.reload ( true ); 
        this.$router.push('/')
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