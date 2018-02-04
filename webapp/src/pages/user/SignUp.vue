<template>
 <div id="access">
   <mnav id="mnav"></mnav>
   <div id="content">
        <div id="title">    
            <router-link to="/a/access">SignIn &emsp;|&emsp;</router-link>
            <router-link to="/a/signup">SignUp</router-link> 
        </div> 
          <input type="text" name="username" placeholder="Username" v-model="Username"  required />
          <input type="text" name="email" placeholder="E-mail" v-model="Email"  required />
          <input type="password" name="password" placeholder="Password" v-model="Password"  required/>
          <input type="password" name="confirm_password" placeholder="Confirm password" v-model="ConfirmPassword"  required/><br/>
          
          <input type="checkbox" id="tos" name="tos" v-model="tos_checked"/>
          <label for="tos">Accept the <router-link to="">Terms of service</router-link></label><br/>
          <input type="checkbox" id="news" name="news" v-model="news_checked"/>
          <label for="news">Subscribe to our newsltetter !</label><br/>

          <button id="submit" @click="signup">Sign up</button>
        
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
      Email: '',
      Password: '',
      ConfirmPassword: ''
    }
  },
  methods: {
    signup () {
      var username = this.Username
      var email = this.Email
      var password = this.Password
      var confirm_password = this.ConfirmPassword
      axios.post('http://localhost:8000/user/signup', {
          username: username,
          email: email,
          password: password,
          confirm_password: confirm_password
      })
      .then(response => {
        console.log(response.data)
        this.$router.push('/a/access')
      })
      .catch(e => {
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
    font-size: 22px;
    font-weight: bold;
    background-color:bisque;
    text-align: center;
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
#submit  {
  margin: 10px 0 20px 0;
  width: 250px;
  height: 33px;
  background-color:bisque;
  border: none;
  border-radius: 2px;
  font-family: 'Roboto', sans-serif;
  font-weight: bold;
  text-transform: uppercase;
  transition: 0.1s ease;
  cursor: pointer;
}
input[type="checkbox"] {
  margin-top: 11px;
}
dialog {
  top: 50%;
  width: 80%;  
  border: 5px solid rgba(0, 0, 0, 0.3);
}
dialog::backdrop{
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.7);
}
#closeDialog {
  display: inline-block;
  border-radius: 3px;
  border: none;
  font-size: 1rem;
  padding: 0.4rem 0.8em;
  background: #eb9816;
  border-bottom: 1px solid #f1b75c;
  color: white;
  font-weight: bold;
  text-align: center;
}
#closeDialog:hover, #closeDialog:focus {
  opacity: 0.92;
  cursor: pointer;
}
@media only screen and (min-width: 600px) {
    #content  {
      margin: 0 auto;
      padding-top: 100px;
  }
}
</style>