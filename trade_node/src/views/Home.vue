<script setup>

import Header from "./Header.vue";
</script>

<template>

  <div class="flex flex-col h-screen p-2 bg-gray-50 gap-2 ">

    <!-- 顶部 -->
    <header class="h-12  shrink-0">
      <Header></Header>
    </header>

    <!-- 下面 -->
    <div class="flex flex-1 min-h-0">

      <!-- 左边 -->
      <aside class="w-64 min-h-0 ">
        <router-view name="left"></router-view>
      </aside>

      <!-- 右边 -->
      <main class="flex-1 min-w-0 min-h-0 h-full ">
        <router-view></router-view>
      </main>

    </div>

  </div>


</template>

<style scoped>

</style>