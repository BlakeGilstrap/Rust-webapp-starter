<template>
  <div id="home">
      <mnav id="mnav"></mnav>
      <main>
        <div id="center">
            <div id="header">
                  <li  id="first"><router-link to="/">All</router-link></li>
                  <li  ><router-link to="/latest">Latest</router-link></li>
                  <li  ><router-link to="/top">Top</router-link></li>
                  <li  ><router-link to="/Share">Share</router-link></li>
                  <li  ><router-link to="/top">Article</router-link></li>
                  <li  ><router-link to="/">FAQ</router-link></li>
                  <li  ><router-link to="/Share">Job</router-link></li>
            </div>
            <div id="title">
                  <span id="title-topic">Topic</span>
                  <span id="right">
                      <span id="info">Category</span>
                      <span id="info">User</span>
                      <span id="info">Reply</span>
                      <span id="info">View</span>
                      <span>Activity</span>
                  </span>      
            </div>
            <div id="content">
                  
                  <div id="items" v-for="article in article_list">
                      <div id="announcement" v-if="article.category === 'Announcement'">
                          <span id="announcement-title"> <router-link to="/kxco/article/article.id" title="article.title">  {{ article.title }} </router-link></span>
                          <span id="right">
                              <span id="info"> {{ article.category }} </span>
                              <span id="info"><router-link to="/user/article.uid"> {{ article.username }} </router-link></span>
                              <span id="info"> {{ article.comments_count }} </span>
                              <span > {{ article.created_at }} </span>
                          </span>                        
                      </div>
                      <div id="item" v-else>
                        <span id="item-title"> <router-link to="/kxco/article/article.id" title="article.title">  {{ article.title }} </router-link></span>
                        <span id="right">
                            <span id="info"> {{ article.category }} </span>
                            <span id="info"><router-link to="/user/article.uid"> {{ article.username }} </router-link></span>
                            <span id="info"> {{ article.comments_count }} </span>
                            <span > {{ article.created_at }} </span>
                        </span>
                      </div>
                  </div>
            </div>
        </div>
      </main>
  </div>
</template>

<script>
import axios from 'axios'
import Mnav from '../../components/nav/Mnav'
export default {
  name: 'home',
  components: {
    "mnav": Mnav
  },
  data: function() {
    return {
      article_list: ''
    }
  },
  mounted: function() {
    axios.get('http://localhost:8000/api/article_list')
      .then((response) => {
        this.article_list = response.data.article_result
        // console.log(response.data.article_result[1])
        // console.log(sessionStorage.getItem('token'))
        // console.log(sessionStorage.getItem('username'))
      })
      .catch((e) => {
        console.log(e)
      })
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
main {
    padding-bottom: 44px;
}
#center {
  box-shadow: 0 1px 1px #cccccc;
}
#center #header {
  height: 50px;
  background-color: rgb(215, 248, 237); 
  /* box-shadow: 0 -3px 3px rgba(0,0,0,0.12), 0 -1px 1px rgba(0,0,0,0.24); */
}
#center #header #first {
  margin-left: 1vw;
}
#center #header li {
  display: inline-block;
  line-height: 50px;
  margin-left: 5vw;
  font-weight: bold;
}
#center #title {
  line-height: 44px;
  border-bottom: 1px solid rgb(231, 238, 233);
}
#center #title #right #info {
  padding-right: 4vw;
}
#center #items {
  line-height: 55px;
  border-bottom: 1px solid rgb(231, 238, 233);
}
#center #announcement {
  color: #eb15ce;
}
#center #title-topic, #announcement-title, #item-title {
  padding: 0 2vw 0 1vw;
}
#center #right {
  float: right;
  padding-right: 1vw;
}
#center #right #info {
  padding-right: 3vw;
}
@media only screen and (max-width: 600px) {
    main{
        margin: 0 auto;
        width: 95%;
    }
    #center {
        margin-top: 28px;
    }
    #center #header li {
      margin-left: 2.2vw;
    }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
    main{
        margin: 0 auto;
        width: 80%;
        padding-top: 77px;
    }
}
@media only screen and (min-width: 1000px) {
    main {
        margin: 0 auto;
        width: 70%;
        padding-top: 77px;
    }
}
</style>