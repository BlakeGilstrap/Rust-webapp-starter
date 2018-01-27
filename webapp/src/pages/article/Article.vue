<template>
    <div id="article">
        <mnav id="mnav"></mnav>
                    <div id="content">
                        <div id="title">
                            <h3> {{ article_id.title }} </h3> 
                            <span id="info">{{ article_id.category }}</span> • 
                            <span id="info"><router-link to="/user/article.uid">{{ article_id.username }}</router-link></span> • 
                        </div>
                        <div id="content">{{ article_id.cooked }}</div>
                    </div>
    </div>
</template>

<script>
import axios from 'axios'
import Mnav from '../../components/nav/Mnav'
export default {
    name: 'article_id',
    components: {
        "mnav": Mnav
    },
    data: function() {
        return {
            article_id: ''
        }
    },
    mounted: function() {
    axios.get('http://localhost:8000/api/article_id')
      .then((response) => {
        this.article_id = response.data.article_id
        console.log(response.data.article_id)
      })
      .catch((e) => {
        console.log(e)
      })
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