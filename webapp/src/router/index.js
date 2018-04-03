import Vue from 'vue'
import Router from 'vue-router'
import Home from '../pages/home/Home'
import Wiki from '../pages/wiki/Wiki'
import More from '../pages/more/More'
import Article from '../pages/article/Article'
import New from '../pages/new/New'
import Access from '../pages/user/Access'
import SignUp from '../pages/user/SignUp'
import Center from '../pages/user/Center'
import About from '../pages/about/About'
import NotFound from '../pages/notfound/NotFound'
Vue.use(Router)
export default new Router({
  mode: 'history',
  linkActiveClass: 'is-active',
  routes: [
    { path: '/', name: 'home', component: Home },
    { path: '/a/wiki', name: 'wiki', component: Wiki },
    { path: '/a/more', name: 'more', component: More },
    { path: '/a/article/:id', name: 'article', component: Article },
    { path: '/a/new', name: 'new', component: New },
    { path: '/a/access', name: 'access', component: Access },
    { path: '/a/signup', name: 'signup', component: SignUp },
    { path: '/a/user/:id', name: 'user', component: Center },
    { path: '/a/about', name: 'about', component: About },
    { path: '*', name: 'notfound', component: NotFound }
  ]
})