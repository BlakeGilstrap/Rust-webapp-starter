import Vue from 'vue'
import Router from 'vue-router'
import Home from '../pages/home/Home'
import Access from '../pages/user/Access'
import SignUp from '../pages/user/SignUp'
import About from '../pages/about/About'
import Centre from '../pages/user/Centre'
import Article from '../pages/article/Article'
import NotFound from '../pages/notfound/NotFound'
Vue.use(Router)
export default new Router({
  mode: 'history',
  linkActiveClass: 'is-active',
  routes: [
    { path: '/', name: 'home', component: Home },
    { path: '/a/access', name: 'access', component: Access },
    { path: '/a/signup', name: 'signup', component: SignUp },
    { path: '/a/user', name: 'user', component: Centre },
    { path: '/a/about', name: 'about', component: About },
    { path: '/a/article/1', name: 'article', component: Article },
    { path: '*', name: 'notfound', component: NotFound }
  ]
})