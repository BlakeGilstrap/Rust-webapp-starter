export default {
    user: {
		authenticated: false,
		role: null
    }, 
    mounted() {
        checkAuth()
    },
    checkAuth() {
		var jwt = localStorage.getItem('id_token');
		var role = localStorage.getItem('user_role');
		if (jwt) {
			this.user.authenticated = true;
			this.user.role = role;
		} else {
			this.user.authenticated = false;
		}
    },

    getAuthHeader () {
        return {
            headers: {
                'Authorization': 'Bearer ' + sessionStorage.getItem('token')
            }
        }
    }
}
