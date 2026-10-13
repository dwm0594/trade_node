<script setup lang="ts">
import {onMounted, reactive, ref} from 'vue'
import {invoke} from "@tauri-apps/api/core";
import {DashboardAnalysis, Dictionary} from "../types/entity";
import {formatDateForDay, formatDuration} from "../utils/util";
import {FAVORITE_CURRENCY, find_attribute, insert_attribute, insert_dict} from "../invoke/sql";
import {showToast} from "../toast/useToast";

type TimeRange = 'month' | 'year' | 'lastYear' | 'beforeLastYear' | 'custom'

const activeRange = ref<TimeRange>('month')


async function search(currency: string, start: number, end: number) {
  const result = await invoke<DashboardAnalysis>("trade_analysis", {
    start,
    end,
    currency,
  });

  console.log('-------12313', result);
  stats.value = result;
}

const ranges = [
  {key: 'month', label: '最近一月'},
  {key: 'year', label: '今年'},
  {key: 'lastYear', label: '去年'},
  {key: 'beforeLastYear', label: '前年'},
  {key: 'custom', label: '自定义'},
]


type CurrencyRange = '人民币' | '美元'

const activeCurrency = ref<CurrencyRange>('人民币')


const currency = [
  {key: '人民币', label: '人民币'},
  {key: '美元', label: '美元'}
]

// 模拟数据
function createDefaultStats(): DashboardAnalysis {
  return {
    aveClosePositionScore: 0,
    aveHoldingTime: 0,
    aveLoss: 0,
    aveLossHoldingTime: 0,
    aveOpenPositionScore: 0,
    aveProfit: 0,
    aveProfitHoldingTime: 0,
    loss: 0,
    lossCount: 0,
    maxHoldingTime: 0,
    minHoldingTime: 0,
    profit: 0,
    profitCount: 0,
    profitLoss: 0,
    profitLossRatio: 0,
    profitRatio: 0,
    size: 0,
    sourceAnalysis: [],
    strategyAnalysis: [],
  };
}

let stats = ref<DashboardAnalysis>(createDefaultStats());


let endTimestamp = Date.now();
const date = new Date()
date.setMonth(date.getMonth() - 1)

const start = ref<number>(date.getTime())
const end = ref<number>(endTimestamp)

function selectRange(range: TimeRange) {
  activeRange.value = range

  if (range === 'custom') {
    if (dataStart.value != '' && dataStart.value != '') {
      let start = new Date(dataStart.value).getTime();
      let end = new Date(dataEnd.value).getTime();
      search(activeCurrency.value, start, end)
      return
    } else {
      showToast('请先选择日期', 'error')
      return
    }
  }

  const now = new Date()
  const year = now.getFullYear()

  let targetYear: number

  switch (range) {
    case 'month':
      const date = new Date()
      date.setMonth(date.getMonth() - 1)
      start.value = date.getTime()
      end.value = Date.now()

      search(activeCurrency.value, start.value, end.value)
      return;
    case 'year':
      targetYear = year
      break
    case 'lastYear':
      targetYear = year - 1
      break
    case 'beforeLastYear':
      targetYear = year - 2
      break
  }

  start.value = new Date(targetYear, 0, 1).getTime()
  end.value = new Date(targetYear + 1, 0, 1).getTime() - 1

  search(activeCurrency.value, start.value, end.value)
}


function selectCurrency(range: CurrencyRange) {

  if (range === activeCurrency.value) {
    return
  }

  activeCurrency.value = range
  insert_attribute(FAVORITE_CURRENCY, activeCurrency.value, 0)

  search(activeCurrency.value, start.value, end.value)
}


function formatMoney(value: number) {
  if (value === 0) {
    return '0'
  }

  return `${value > 0 ? '+' : '-'} ${activeCurrency.value === '人民币' ? '¥' : '$'} ${Math.abs(value).toLocaleString()}`
}

function setCurrency(value: Dictionary | null) {
  if (value != null) {
    activeCurrency.value = value.name as CurrencyRange
  }
}

const dataStart = ref('')
const dataEnd = ref('')

onMounted(async () => {

  let dictionary = await find_attribute(FAVORITE_CURRENCY);
  setCurrency(dictionary)


  await search(activeCurrency.value, start.value, end.value)
})

</script>

<template>
  <div class="h-full overflow-y-auto bg-gray-50 p-6">


    <div class="mx-auto max-w-7xl">

      <!-- Header -->
      <div class="mb-6 flex items-center justify-between">
        <div>
          <h1 class="text-2xl font-semibold text-gray-800">
            交易分析
          </h1>

          <p class="mt-1 text-sm text-gray-400">
            {{ formatDateForDay(start) }} — {{ formatDateForDay(end) }}
          </p>
        </div>
      </div>


      <!--    币种 以及  -->
      <div class="mb-6 flex items-center justify-between ">
        <!--        币种-->
        <div class="join ">

          <button
              v-for="item in currency"
              :key="item.key"
              class="btn join-item"
              :class="activeCurrency === item.key ? 'btn-neutral' : 'btn-ghost bg-white'"
              @click="selectCurrency(item.key as CurrencyRange)"
          >
            {{ item.label }}
          </button>
        </div>

        <div class="flex gap-1 items-center">

          <!-- 时间范围 -->
          <div class="join ">

            <button
                v-for="item in ranges"
                :key="item.key"
                class="btn join-item "
                :class="activeRange === item.key ? 'btn-neutral' : 'btn-ghost bg-white'"
                @click="selectRange(item.key as TimeRange)"
            >
              {{ item.label }}
            </button>
          </div>


          <fieldset class="fieldset h-full">
            <div class="flex items-center gap-2">
              <input
                  v-model="dataStart"
                  class="input input-bordered w-full"
                  type="datetime-local"
              />
              <span>至</span>
              <input
                  v-model="dataEnd"
                  class="input input-bordered w-full"
                  type="datetime-local"
              />
            </div>

          </fieldset>

        </div>


      </div>

      <!-- 核心表现 -->
      <section class="mb-6">

        <h2 class="mb-3 text-sm font-medium text-gray-500">
          核心表现
        </h2>

        <div class="grid grid-cols-4 gap-4">

          <!-- 总盈亏 -->
          <div class="rounded-xl border border-gray-200 bg-white p-5">
            <div class="text-sm text-gray-400">
              总盈亏
            </div>

            <div
                class="mt-3 text-3xl font-semibold"
                :class="stats.profitLoss >= 0 ? 'text-green-500' : 'text-red-500'"
            >
              {{ formatMoney(stats.profitLoss) }}
            </div>

            <div class="mt-2 text-xs text-gray-400">
              当前统计周期
            </div>
          </div>


          <!-- 盈亏比 -->
          <div class="rounded-xl border border-gray-200 bg-white p-5">
            <div class="text-sm text-gray-400">
              盈亏比
            </div>

            <div class="mt-3 text-3xl font-semibold text-gray-800">
              {{ (stats.profitLossRatio).toFixed(2) }}
            </div>

            <div class="mt-2 text-xs text-gray-400">
              总盈利 / 总亏损
            </div>
          </div>


          <!-- 胜率 -->
          <div class="rounded-xl border border-gray-200 bg-white p-5">
            <div class="text-sm text-gray-400">
              胜率
            </div>

            <div class="mt-3 text-3xl font-semibold text-gray-800">
              {{ (stats.profitRatio * 100).toFixed(2) }}%
            </div>

            <div class="mt-2 text-xs text-gray-400">
              {{ stats.profitCount }} 胜 / {{ stats.lossCount }} 负
            </div>
          </div>


          <!-- 交易次数 -->
          <div class="rounded-xl border border-gray-200 bg-white p-5">
            <div class="text-sm text-gray-400">
              交易次数
            </div>

            <div class="mt-3 text-3xl font-semibold text-gray-800">
              {{ stats.size }}
            </div>

            <div class="mt-2 text-xs text-gray-400">
              已完成交易
            </div>
          </div>

        </div>
      </section>


      <!-- 盈亏结构 + 持仓特征 -->
      <div class="mb-6 grid grid-cols-2 gap-4">

        <!-- 盈亏结构 -->
        <section class="rounded-xl border border-gray-200 bg-white p-5">

          <h2 class="mb-5 font-medium text-gray-800">
            盈亏结构
          </h2>

          <div class="space-y-4">

            <div class="flex items-center justify-between">
              <span class="text-sm text-gray-400">
                总盈利
              </span>

              <span class="font-medium text-green-500">
                {{ formatMoney(stats.profit) }}
              </span>
            </div>

            <div class="flex items-center justify-between">
              <span class="text-sm text-gray-400">
                总亏损
              </span>

              <span class="font-medium text-red-500">
                {{ formatMoney(stats.loss) }}
              </span>
            </div>

            <div class="h-px bg-gray-100"></div>

            <div class="flex items-center justify-between">
              <span class="text-sm text-gray-400">
                平均盈利
              </span>

              <span class="font-medium text-green-500">
                {{ formatMoney(stats.aveProfit) }}
              </span>
            </div>

            <div class="flex items-center justify-between">
              <span class="text-sm text-gray-400">
                平均亏损
              </span>

              <span class="font-medium text-red-500">
                {{ formatMoney(stats.aveLoss) }}
              </span>
            </div>

          </div>
        </section>


        <!-- 持仓特征 -->
        <section class="rounded-xl border border-gray-200 bg-white p-5">

          <h2 class="mb-5 font-medium text-gray-800">
            持仓特征
          </h2>

          <div class="grid grid-cols-2 gap-x-8 gap-y-4">

            <div>
              <div class="text-sm text-gray-400">
                平均持仓时间
              </div>

              <div class="mt-1 font-medium text-gray-800">
                {{ formatDuration(stats.aveHoldingTime) }}
              </div>
            </div>

            <div>
              <div class="text-sm text-gray-400">
                最长持仓
              </div>

              <div class="mt-1 font-medium text-gray-800">
                {{ formatDuration(stats.maxHoldingTime) }}
              </div>
            </div>

            <div>
              <div class="text-sm text-gray-400">
                最短持仓
              </div>

              <div class="mt-1 font-medium text-gray-800">
                {{ formatDuration(stats.minHoldingTime) }}
              </div>
            </div>

            <div>
              <div class="text-sm text-gray-400">
                盈利单平均
              </div>

              <div class="mt-1 font-medium text-green-500">
                {{ formatDuration(stats.aveProfitHoldingTime) }}
              </div>
            </div>

            <div>
              <div class="text-sm text-gray-400">
                亏损单平均
              </div>

              <div class="mt-1 font-medium text-red-500">
                {{ formatDuration(stats.aveLossHoldingTime) }}
              </div>
            </div>
          </div>
        </section>

      </div>


      <!-- 决策质量 + 交易来源 -->
      <div class="mb-6 grid grid-cols-2 gap-4">

        <!-- 来源分析 -->
        <section class="rounded-xl border border-gray-200 bg-white p-5">

          <h2 class="mb-5 font-medium text-gray-800">
            消息来源
          </h2>

          <div class="overflow-hidden rounded-lg border border-gray-100">

            <table class="w-full text-sm">

              <thead class="bg-gray-50 text-gray-400">
              <tr>
                <th class="px-4 py-3 text-left font-normal">
                  来源
                </th>

                <th class="px-4 py-3 text-right font-normal">
                  次数
                </th>

                <th class="px-4 py-3 text-right font-normal">
                  胜率
                </th>

                <th class="px-4 py-3 text-right font-normal">
                  使用率
                </th>

                <th class="px-4 py-3 text-right font-normal">
                  盈亏比
                </th>
              </tr>
              </thead>

              <tbody>

              <tr
                  v-for="item in [...stats.sourceAnalysis].sort((a, b) => b.useRatio - a.useRatio)"
                  :key="item.name"
                  class="border-t border-gray-100"
              >
                <td class="px-4 py-3 text-gray-700">
                  {{ item.name }}
                </td>

                <td class="px-4 py-3 text-right text-gray-500">
                  {{ item.useCount }}
                </td>

                <td
                    class="px-4 py-3 text-right"
                    :class="
                      item.profitRatio >= 60
                        ? 'text-green-500'
                        : item.profitRatio < 50
                          ? 'text-red-500'
                          : 'text-gray-600'
                    "
                >
                  {{ (item.profitRatio * 100).toFixed(2) }}%
                </td>

                <td class="px-4 py-3 text-right text-gray-700">
                  {{ (item.useRatio * 100).toFixed(2) }}%
                </td>
                <td class="px-4 py-3 text-right text-gray-700">
                  {{ (item.profitLossRatio).toFixed(2) }}
                </td>
              </tr>

              </tbody>

            </table>

          </div>
        </section>

        <!--        策略分析-->
        <section class="rounded-xl border border-gray-200 bg-white p-5">

          <h2 class="mb-5 font-medium text-gray-800">
            交易策略
          </h2>

          <div class="overflow-hidden rounded-lg border border-gray-100">

            <table class="w-full text-sm">

              <thead class="bg-gray-50 text-gray-400">
              <tr>
                <th class="px-4 py-3 text-left font-normal">
                  策略
                </th>

                <th class="px-4 py-3 text-right font-normal">
                  次数
                </th>

                <th class="px-4 py-3 text-right font-normal">
                  胜率
                </th>

                <th class="px-4 py-3 text-right font-normal">
                  使用率
                </th>

                <th class="px-4 py-3 text-right font-normal">
                  盈亏比
                </th>
              </tr>
              </thead>

              <tbody>

              <tr
                  v-for="item in [...stats.strategyAnalysis].sort((a, b) => b.useRatio - a.useRatio)"
                  :key="item.name"
                  class="border-t border-gray-100"
              >
                <td class="px-4 py-3 text-gray-700">
                  {{ item.name }}
                </td>

                <td class="px-4 py-3 text-right text-gray-500">
                  {{ item.useCount }}
                </td>

                <td
                    class="px-4 py-3 text-right"
                    :class="
                      item.profitRatio >= 60
                        ? 'text-green-500'
                        : item.profitRatio < 50
                          ? 'text-red-500'
                          : 'text-gray-600'
                    "
                >
                  {{ (item.profitRatio * 100).toFixed(2) }}%
                </td>

                <td class="px-4 py-3 text-right text-gray-700">
                  {{ (item.useRatio * 100).toFixed(2) }}%
                </td>

                <td class="px-4 py-3 text-right text-gray-700">
                  {{ (item.profitLossRatio).toFixed(2) }}
                </td>
              </tr>

              </tbody>

            </table>

          </div>
        </section>

      </div>


    </div>
  </div>
</template>