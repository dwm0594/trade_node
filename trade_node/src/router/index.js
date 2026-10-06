import {createRouter, createWebHashHistory} from 'vue-router'
import Home from '../views/Home.vue'
import Strategy from '../views/Strategy.vue'
import AddTrade from "../views/AddTrade.vue";
import DashBoard from "../views/DashBoard.vue";
import List from '../views/List.vue';
import Backtracking from "../views/Backtracking.vue";


const router = createRouter({
    history: createWebHashHistory(),


    routes: [
        {
            path: '/',
            component: Home,

            children: [
                {
                    path: '',
                    components: {
                        default: DashBoard,
                        left: List
                    }
                },

                {
                    path: 'add',
                    components: {
                        default: AddTrade,
                        left: List
                    }
                }, {
                    path: 'strategy',
                    components: {
                        default: Strategy,
                        left: List
                    }
                }, {
                    path: 'backtrace/:id',
                    name: 'Backtracking',
                    components: {
                        default: Backtracking,
                        left: List
                    }
                }
            ]
        }
    ]
})

export default router