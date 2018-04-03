<template>
    <div id="article_id">
        <mnav id="mnav"></mnav>
                    <div id="content">
                        <div id="title">
                            <h3> title : {{ article.title }} </h3> 
                            <span id="info">category : {{ article.category }}</span>  
                            <span id="info"><a :href="'/a/user/' + article.user_id">user_id : {{ article.user_id }}</a></span>  
                            <span id="info">created_at : {{ article.created_at }}</span>  
                        </div>
                        <div id="body">content : {{ article.body }}</div>
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
            article: ''
        }
    },
    mounted: function() {
    // let article_id = this.$route.params.id
    // console.log(article_id)
    axios.get("http://localhost:8000/api/" + this.$route.params.id)
      .then((response) => {
        this.article = response.data.article
        console.log(response.data.article)
      })
      .catch((e) => {
        console.log(e)
      })
  }
}
</script>

<style scoped>
#body {
    margin: 2rem auto;
}

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