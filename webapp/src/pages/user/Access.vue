<template>
  <div id="access">
      <mnav id="mnav"></mnav>
      <div id="content">
          <div id="title">    
            <router-link to="/a/access">SignIn &emsp;|&emsp;</router-link>
            <router-link to="/a/signup">SignUp</router-link> 
          </div>
            <input type="text" name="username" placeholder="Username" v-model="Username" />
            <input type="password" name="password" placeholder="Password" v-model="Password" /><br/>
          <div id="add">
            <label class="checkbox">
            <input type="checkbox" name="remember" value="1" id="checks" checked="true"/>
            <span class="check" for="checks"></span> 
          </label>Remmber
          <span> &emsp;&emsp;<router-link to="/settings/missing_pwd" style="text-decoration-line: none;">Forget Password?</router-link></span><br/>
          </div>
          <button id="submit" @click="signin">Sign in</button><br/>
          <div id="text"> Login with social </div>
          <button class="social-signin facebook">facebook</button>
          <button class="social-signin twitter">Twitter</button>
          <button class="social-signin google">Google+</button>
      </div>
  </div>
</template>

<script>
import axios from 'axios'
import Mnav from '../../components/nav/Mnav'
export default {
  name: 'access',
  components: {
    "mnav": Mnav
  },
  data () {
    return {
      Username: '',
      Password: ''
    }
  },
  methods: {
    signin () {
      var username = this.Username
      var password = this.Password

      axios.post('http://localhost:8000/user/signin', {
          username: username,
          password: password
      })
      .then((response) => {
        // console.log(response.data.token);
        sessionStorage.setItem('token',response.data.token);
        // sessionStorage.setItem('user',response.data.signin_user);
        sessionStorage.setItem('username',response.data.signin_user.username);
        // console.log(response.data.message)
        // console.log(response.data.signin_user)
        // console.log(sessionStorage.getItem('username'))
        window.location.reload ( true ); 
        this.$router.push('/')
      })
      .catch((e) => {
        console.log(e)
      })
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
#content {
    width: 250px;
    margin: 0 auto;
    padding-top: 33px;
}
#title {
    padding: 0.5rem 0;
    text-align: center;
    font-size: 22px;
    font-weight: bold;
    background-color:bisque;
}
input[type="text"],
input[type="password"] {
  margin: 6px auto auto;
  width: 250px;
  height: 36px;
  border: none;
  border-bottom: 1px solid #AAA;
  font-size: 16px;
}
#add {
  margin-top: 10px;
}
#submit  {
  margin: 10px 0;
  width: 250px;
  height: 33px;
  background-color:bisque;
  border: none;
  border-radius: 2px;
  font-weight: bold;
}
#text {
  width: 250px;
  border-radius: 3px;
  padding: 0.3rem 0.8em;
  background-color:bisque;
  border-bottom: 1px solid #f1b75c;
  font-weight: bold;
  text-align: center;
}
button.social-signin {
  margin: 10px 0;
  width: 80.5px;
  height: 33px;
  border: none;
  border-radius: 2px;
  color: #FFF;
}
button.social-signin:hover,
button.social-signin:focus {
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.4);
  transition: 0.2s ease;
}
button.social-signin:active {
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.4);
  transition: 0.2s ease;
}
button.social-signin.facebook {
  background: #32508E;
}
button.social-signin.twitter {
  background: #55ACEE;
}
button.social-signin.google {
  background: #DD4B39;
}
@media only screen and (min-width: 600px) {
    #content {
      margin: 0 auto;
      padding-top: 100px;
    }
}
</style>