<template>
  <div id="mnav">
      <header>
        <div id="line"></div>
        <h1><router-link to="/">Rust</router-link></h1>
        <label ><router-link to="#" id="menu">Menu</router-link></label>
      </header>
      <div id="lnav">
          <li> <router-link to="/a/wiki" title="wiki">wiki</router-link></li>
          <li> <router-link to="/a/more" title="more">more</router-link></li>
      </div>
      <div id="rnav">
          <li v-if="username"> 
            <a href="/a/new" title="new">new</a>
            <router-link to="/a/user" title="username">{{username}}</router-link>
            <a href="/a/access" title="Logout" @click="logout">logout</a>
          </li>
          <li v-else > 
            <router-link to="/a/access" title="access">access</router-link>
          </li>
          <li> <router-link to="/a/about" title="about">about</router-link></li>
      </div> 
  </div>
</template>

<script>
export default {
  name: 'Mnav',
  data: function () {
    return { 
      username: sessionStorage.getItem('username')
    }
  },
  methods: {
    logout() {
     sessionStorage.removeItem('token')
     sessionStorage.removeItem('username')
     this.$router.push('/a/access')
    }
  }
}
</script>

<style lang="css" scoped>
#line {
  position: fixed;
  zoom:1;
  display: flex;
  width: 100%;
  height: 4px;
  background-color: green; 
}
header {
  position: fixed;
  width: 100%;
  line-height: 60px;
  zoom:1;
  display: flex;
  background-color: white;
  justify-content: space-between;
  border-bottom: 1px solid #dfdcdc;  
  box-shadow: 0 3px 3px rgba(200, 216, 206, 0.12), 0 3px 3px rgba(97, 107, 100, 0.24);
}

header h1 a {
  font-size: 33px;
  margin: auto 2vw;
  font-weight: bold;
}
header label {
  float: right;
  font-size: 26px;
  font-weight: bold;
  margin: auto 2vw;
}
@media only screen and (min-width:  600px) and (max-width:  1000px) {
  #menu {
    display: none;
  }
  header h1 a {
    font-size: 33px;
    margin: auto 6vw;
    font-weight: bold;
  }
  #lnav {
    position: fixed;
    margin-left: 18vw;
    line-height: 60px;
  }
  #lnav li {
    display: inline-block;
  }  
  #lnav li a {
    font-size: 20px;
    font-weight: bold;
    padding-right: 2vw;
  }
  #rnav {
    position: fixed;
    right: 0;
    line-height: 60px;
    padding-right: 6vw;
  }

  #rnav li {
    display: inline-block;
  }
  #rnav li a {
    font-size: 20px;
    font-weight: bold;
    padding-left: 2vw;
  }
}
@media only screen and (min-width:  1000px) {
  #menu {
    display: none;
  }
  header h1 a {
    font-size: 33px;
    margin: auto 11vw;
    font-weight: bold;
  }
  #lnav {
    position: fixed;
    margin-left: 20vw;
    line-height: 60px;
  }
  #lnav li {
    display: inline-block;
  }  
  #lnav li a {
    font-size: 20px;
    font-weight: bold;
    padding-right: 2vw;
  }
  #rnav {
    position: fixed;
    right: 0;
    line-height: 60px;
    padding-right: 10vw;
  }

  #rnav li {
    display: inline-block;
  }
  #rnav li a {
    font-size: 20px;
    font-weight: bold;
    padding-left: 2vw;
  }
}
</style>