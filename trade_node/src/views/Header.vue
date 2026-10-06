<script setup>

import {useRouter} from 'vue-router'

const router = useRouter()

function addTrade() {
  router.push('/add')
}

function addStrategy() {
  router.push('/strategy')
}

function addDashboard() {
  router.push('/')
}

</script>

<template>
  <!-- popovertarget must match the ID of the corresponding popover, and each ID must be unique -->
  <button class="btn sm:hidden" popovertarget="my-megamenu-1">Menu</button>

  <div class="megamenu max-sm:megamenu-vertical h-12 p-2 border border-base-300 w-full" id="my-megamenu-1" popover>
    <span class="megamenu-active"></span>

    <button popovertarget="a1">功能</button>
    <div id="a1" popover>
      <ul class="menu">
        <li><a @click="addTrade()">添加日志</a></li>
        <li><a @click="addStrategy()">添加策略</a></li>
      </ul>
    </div>

    <button popovertarget="a2">统计</button>
    <div id="a2" popover>
      <ul class="menu">
        <li><a @click="addDashboard()">数据总览</a></li>
      </ul>
    </div>


    <button popovertarget="a3">设置</button>
    <div id="a3" popover>
      <ul class="menu">
        <li><a @click="addDashboard()">设置偏好</a></li>
      </ul>
    </div>
  </div>

</template>

<style scoped>

</style>