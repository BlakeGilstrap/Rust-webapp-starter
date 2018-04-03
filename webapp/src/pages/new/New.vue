<template>
    <div id="new">
        <mnav id="mnav"></mnav>
        <div id="content">
            <div id="new-title"><p>New Publish</p></div>
            <form id="form" >
                    <div id="topic-group">
                        <span  id="category">
                                <select v-if="username" name="category" v-model="category" id="category-control" >
                                    <option value="Topic">Topic <span class="icon-arrow"></span></option>
                                    <option value="Share">Share</option>
                                    <option value="Article">Article</option>
                                    <option value="FAQ">FAQ</option>
                                    <option value="Job">Job</option>
                                    <option value="Blog">Blog</option>
                                    <option value="Announcement">Announcement</option>

                                    <option style="color: #1EB332;" disabled="disabled" >--wiki--</option>
                                    <option value="Docs">Docs</option>
                                    <option value="Resources">Resources</option>
                                    <option value="Web">Web</option>
                                    <option value="Embed">Embed</option>
                                    <option value="Server">Server</option>
                                    <option value="Client">Wasm</option>
                                </select>
                                <select v-else name="category" v-model="category" id="category-control">
                                    <option value="Topic">Topic</option>
                                    <option value="Share">Share</option>
                                    <option value="Article">Article</option>
                                    <option value="FAQ">FAQ</option>
                                    <option value="Job">Job</option>
                                    <option value="Blog">Blog</option>
                                </select>
                        </span>
                        <span id="title">
                                <input type="text" name="title" v-model="Title" placeholder="Please input title">
                        </span>
                    </div>    
                    <div id="new">
                                <textarea name="content" v-model="Content" placeholder="Write new Publish in markdown!"></textarea>
                    </div>
                    <div id="new">
                                <button type="submit" id="submit" @click="publish" ><span class="tip"> Publish </span></button>
                    </div>
            </form>
        </div>
    </div>
</template>

<script>
import axios from 'axios'
import Mnav from '../../components/nav/Mnav'
export default {
    name: 'new',
    components: {
        "mnav": Mnav
    },
    data () {
        return {
        Category: '',
        Title: '',
        Content: ''
        }
    },
    methods: {
        publish () {
            var category = this.Category
            var title = this.Title
            var content = this.Content
            var user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
            axios.post('http://localhost:8000/api/article_new', {
                user_id: user_id,
                category: category,
                title: title,
                content: content
            })
            .then(response => {
                // console.log(response.data.msg)
                window.location.reload ( true )
                this.$router.push('/')
            })
            .catch(e => {
                console.log(e)
            })
        }
    }
}
</script>

<style scoped>
#new-title {
    margin: 4px auto;
    width: 100%;
    line-height: 33px;
    background-color:rgb(231, 236, 235);
}
form #topic-group {
   margin: 11px 0 11px 0;
}
form #topic-group #category #category-control {
    background-color: #FFFFFF;
    border :1px solid #CAC1C1; /*Chrome和Firefox里面的边框是不一样的，所以复写了一下*/
    border: solid 1px #CAC1C1;
    padding-left: 9px;
}
form #topic-group #category #category-control, form #topic-group input {
    height: 30px;
}
form #new textarea {
    width:100%; 
    height: 444px;
}
form #new button {
    width:63px; 
    line-height:30px;
    background-color:rgb(255, 255, 255);
    border :1px solid #a39c9c;
}
@media only screen and (max-width: 600px) {
    #content  {
      margin: 0.5rem auto;
      width: 95%;
    }
    form #topic-group #category #category-control, form #topic-group input {
        width: 100%;
    }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
    #content  {
        margin: 0 auto;
        width: 72%;
        padding-top: 77px;
    }
    form #topic-group input {
        width: 72%;
        float: right;
    }
}
@media only screen and (min-width: 1000px) {
  #content  {
      margin: 0 auto;
      width: 66%;
      padding-top: 77px;
  }
  form #topic-group input {
        width: 80%;
        float: right;
  }
}
</style>