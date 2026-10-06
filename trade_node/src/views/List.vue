<script setup lang="ts">
import {onMounted, onUnmounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {Position, TradeForm} from "../types/entity";
import {formatDate, formatDuration} from "../utils/util";


const nodes = ref<TradeForm[]>([]);
import {useRouter} from 'vue-router'
import {emitter} from "../event/eventBus";

const router = useRouter()


async function find_history_data(day: number) {
  try {

    nodes.value = await invoke("find_trade_node", {
      day: day
    });

  } catch (e) {
    console.error("查询失败:", e);
  }
}


async function find_history_data_by_range(start: number, end: number) {
  try {
    nodes.value = await invoke("find_trade_node_by_range", {
      start: start,
      end: end
    });

  } catch (e) {
    console.error("查询失败:", e);
  }
}

function get_profit(node: TradeForm) {
  let profit = 0;


  if (node.closePosition !== undefined) {
    node.closePosition.forEach((item: { profit: number; }) => {
      profit += item.profit;
    })
  }
  return profit;
}

function get_open_position_time(node: TradeForm) {
  return formatDate(Number(node.date))
}


function formatProfit(node: TradeForm): string {
  const profit = get_profit(node)

  if (profit === 0) {
    return ''
  }

  const sign = profit > 0 ? '+' : ''

  return `${sign}${profit}${node.currencyType === '人民币' ? '¥' : '$'}`
}

function get_open_position_number(node: TradeForm): string {

  if (node.openPosition !== undefined && node.openPosition.length > 0) {
    let total = 0;
    node.openPosition.forEach((item: { total: number; }) => {
      total += item.total;
    })
    return `${String(total)}${node.currencyType === '人民币' ? '¥' : '$'}`;
  }

  return '';
}


function get_holding_position_time(node: TradeForm) {
  if (node.holdingTime == null) {
    return '未开仓'
  }
  return `持仓 ${formatDuration(node.holdingTime)}`;
}


function goToBacktracking(id: number) {
  router.push(`/backtrace/${id}`)
}

onMounted(async () => {
  await find_history_data(7);
});

const refreshHistory = async () => {
  await find_history_data(7)
}

const search = async (data: { start: number; end: number }) => {
  await find_history_data_by_range(data.start, data.end)
}


onMounted(async () => {
  await find_history_data(7)

  emitter.on('trade-node-updated', refreshHistory)
  emitter.on('search', search)

})

onUnmounted(() => {
  emitter.off('trade-node-updated', refreshHistory)
  emitter.off('search', search)
})

</script>

<template>

  <div class="list bg-base-100 flex flex-col h-full  px-0.5 ">

    <!--    搜索框-->
    <div class="shrink-0  items-center gap-2 m-1 grid grid-cols-3 ">
      <input
          type="radio"
          name="tradeType"
          class="btn flex-1 min-w-0 px-1"
          aria-label="一周"
          checked
          @click="find_history_data(7)"
      />
      <input
          type="radio"
          name="tradeType"
          class="btn flex-1 min-w-0 px-1"
          aria-label="一月"
          @click="find_history_data(30)"
      />
      <input
          type="radio"
          name="tradeType"
          class="btn flex-1 min-w-0 px-1"
          aria-label="半年"
          @click="find_history_data(180)"
      />
    </div>

    <!-- 展示框 -->
    <div class="flex-1 min-h-0 overflow-y-auto m-1">


      <div
          v-for="node in nodes"
          :key="node.id"
          class="relative mb-2 rounded-lg bg-white p-3
             cursor-pointer hover:bg-gray-50
             border border-gray-200"
          @click="goToBacktracking(node.id)"
      >

        <!-- 多空标签 -->
        <div
            class="absolute left-2 top-3
               px-1.5 py-0.5 rounded text-xs font-medium"
            :class="node.direction === '空'
          ? 'bg-red-100 text-red-600'
          : 'bg-green-100 text-green-600'"
        >
          {{ node.direction }}
        </div>

        <!-- 名称 + 盈利 -->
        <div class="flex items-center justify-between pl-8">

          <div class="font-medium text-gray-800 truncate">
            {{ node.name }}
          </div>

          <!--          开仓量-->
          <div
              class="ml-2 text-sm font-semibold"
          >
            {{ get_open_position_number(node) }}
          </div>


          <div
              class="ml-2 text-sm font-semibold"
              :class="get_profit(node) <= 0
            ? 'text-red-500'
            : 'text-green-500'"
          >
            {{ formatProfit(node) }}
          </div>

        </div>

        <div class="mt-1 pl-8 flex items-center justify-between text-xs text-gray-400">
          <!-- 开仓时间 -->
          <div>
            {{ get_open_position_time(node) }}
          </div>

          <!-- 持仓 -->
          <div>
            {{ get_holding_position_time(node) }}
          </div>
        </div>

      </div>
    </div>

  </div>

</template>

<style scoped>

</style>