<script setup lang="ts">
defineProps<{
  currencyType: string
}>()
</script>

<template>
  <span v-if="currencyType === '人民币'">¥</span>
  <span v-else-if="currencyType === '美元'">$</span>
</template>