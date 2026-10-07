<template>
  <div class="h-full overflow-y-auto bg-gray-50 p-6">

    <div class="mx-auto max-w-7xl">

      <!-- Header -->
      <div class="mb-6">
        <h1 class="text-2xl font-semibold text-gray-800">
          账户信息
        </h1>

        <p class="mt-1 text-sm text-gray-400">
          设置账户的资金及风险参数
        </p>
      </div>


      <!-- 人民币账户 -->
      <section class="mb-6 rounded-xl border border-gray-100 bg-white p-6">

        <div class="mb-6 flex items-center justify-between">
          <div>
            <h2 class="text-base font-medium text-gray-800">
              人民币账户
            </h2>

          </div>

          <div class="rounded-lg bg-gray-50 px-3 py-1.5 text-sm text-gray-500">
            ¥ CNY
          </div>
        </div>


        <!-- 初始余额 -->
        <div class="mb-8">

          <label class="mb-2 block text-sm text-gray-600">
            初始账户余额
          </label>

          <div class="relative max-w-md">
            <span
                class="absolute left-3 top-1/2 -translate-y-1/2 text-sm text-gray-400"
            >
              ¥
            </span>

            <input
                v-model.number="accounts.cny.initialBalance"
                type="number"
                min="0"
                step="0.01"
                class="w-full rounded-lg border border-gray-200 bg-white py-2.5 pl-8 pr-3 text-sm text-gray-700 outline-none focus:border-gray-400"
                placeholder="请输入初始余额"
            />
          </div>

        </div>


        <!-- 单笔风险 -->
        <div class="mb-8">

          <div class="mb-3 flex items-center justify-between">

            <div>
              <div class="text-sm font-medium text-gray-700">
                单笔风险承受
              </div>

              <div class="mt-1 text-xs text-gray-400">
                单笔交易最大允许亏损占账户余额的比例
              </div>
            </div>

            <div class="text-lg font-semibold text-gray-700">
              {{ accounts.cny.singleRisk }}%
            </div>

          </div>

          <input
              v-model.number="accounts.cny.singleRisk"
              type="range"
              min="0"
              max="10"
              step="0.5"
              class="range range-sm w-full"
          />

          <div class="mt-1 flex justify-between text-xs text-gray-400">
            <span>0%</span>
            <span>5%</span>
            <span>10%</span>
          </div>

        </div>


        <!-- 月度风险 -->
        <div>

          <div class="mb-3 flex items-center justify-between">

            <div>
              <div class="text-sm font-medium text-gray-700">
                月度风险承受
              </div>

              <div class="mt-1 text-xs text-gray-400">
                单月累计最大允许亏损占账户余额的比例
              </div>
            </div>

            <div class="text-lg font-semibold text-gray-700">
              {{ accounts.cny.monthlyRisk }}%
            </div>

          </div>

          <input
              v-model.number="accounts.cny.monthlyRisk"
              type="range"
              min="0"
              max="20"
              step="0.5"
              class="range range-sm w-full"
          />

          <div class="mt-1 flex justify-between text-xs text-gray-400">
            <span>0%</span>
            <span>10%</span>
            <span>20%</span>
          </div>

        </div>


        <!-- 风险额度 -->
        <div class="mt-8 grid grid-cols-2 gap-4">

          <div class="rounded-lg bg-gray-50 p-4">
            <div class="text-xs text-gray-400">
              单笔最大亏损
            </div>

            <div class="mt-2 text-xl font-semibold text-gray-700">
              ¥{{ cnySingleRiskAmount.toFixed(2) }}
            </div>
          </div>

          <div class="rounded-lg bg-gray-50 p-4">
            <div class="text-xs text-gray-400">
              月度最大亏损
            </div>

            <div class="mt-2 text-xl font-semibold text-gray-700">
              ¥{{ cnyMonthlyRiskAmount.toFixed(2) }}
            </div>
          </div>

        </div>

      </section>


      <!-- 美元账户 -->
      <section class="mb-6 rounded-xl border border-gray-100 bg-white p-6">

        <div class="mb-6 flex items-center justify-between">
          <div>
            <h2 class="text-base font-medium text-gray-800">
              美元账户
            </h2>

          </div>

          <div class="rounded-lg bg-gray-50 px-3 py-1.5 text-sm text-gray-500">
            $ USD
          </div>
        </div>


        <!-- 初始余额 -->
        <div class="mb-8">

          <label class="mb-2 block text-sm text-gray-600">
            初始账户余额
          </label>

          <div class="relative max-w-md">
            <span
                class="absolute left-3 top-1/2 -translate-y-1/2 text-sm text-gray-400"
            >
              $
            </span>

            <input
                v-model.number="accounts.usd.initialBalance"
                type="number"
                min="0"
                step="0.01"
                class="w-full rounded-lg border border-gray-200 bg-white py-2.5 pl-8 pr-3 text-sm text-gray-700 outline-none focus:border-gray-400"
                placeholder="请输入初始余额"
            />
          </div>

        </div>


        <!-- 单笔风险 -->
        <div class="mb-8">

          <div class="mb-3 flex items-center justify-between">

            <div>
              <div class="text-sm font-medium text-gray-700">
                单笔风险承受
              </div>

              <div class="mt-1 text-xs text-gray-400">
                单笔交易最大允许亏损占账户余额的比例
              </div>
            </div>

            <div class="text-lg font-semibold text-gray-700">
              {{ accounts.usd.singleRisk }}%
            </div>

          </div>

          <input
              v-model.number="accounts.usd.singleRisk"
              type="range"
              min="0"
              max="10"
              step="0.5"
              class="range range-sm w-full"
          />

          <div class="mt-1 flex justify-between text-xs text-gray-400">
            <span>0%</span>
            <span>5%</span>
            <span>10%</span>
          </div>

        </div>


        <!-- 月度风险 -->
        <div>

          <div class="mb-3 flex items-center justify-between">

            <div>
              <div class="text-sm font-medium text-gray-700">
                月度风险承受
              </div>

              <div class="mt-1 text-xs text-gray-400">
                单月累计最大允许亏损占账户余额的比例
              </div>
            </div>

            <div class="text-lg font-semibold text-gray-700">
              {{ accounts.usd.monthlyRisk }}%
            </div>

          </div>

          <input
              v-model.number="accounts.usd.monthlyRisk"
              type="range"
              min="0"
              max="20"
              step="0.5"
              class="range range-sm w-full"
          />

          <div class="mt-1 flex justify-between text-xs text-gray-400">
            <span>0%</span>
            <span>10%</span>
            <span>20%</span>
          </div>

        </div>


        <!-- 风险额度 -->
        <div class="mt-8 grid grid-cols-2 gap-4">

          <div class="rounded-lg bg-gray-50 p-4">
            <div class="text-xs text-gray-400">
              单笔最大亏损
            </div>

            <div class="mt-2 text-xl font-semibold text-gray-700">
              ${{ usdSingleRiskAmount.toFixed(2) }}
            </div>
          </div>

          <div class="rounded-lg bg-gray-50 p-4">
            <div class="text-xs text-gray-400">
              月度最大亏损
            </div>

            <div class="mt-2 text-xl font-semibold text-gray-700">
              ${{ usdMonthlyRiskAmount.toFixed(2) }}
            </div>
          </div>

        </div>

      </section>


      <!-- 保存 -->
      <div class="flex justify-end pb-6">
        <button
            class="btn btn-neutral px-6"
            @click="save"
        >
          保存设置
        </button>
      </div>

    </div>
  </div>
</template>


<script setup lang="ts">

import {computed, onMounted, reactive} from 'vue'
import {CNY_ACCOUNT_SETTING, find_account, insert_account_setting, USD_ACCOUNT_SETTING,} from "../invoke/sql";
import {showToast} from "../toast/useToast";
import {AccountSetting, Dictionary} from "../types/entity";

const accounts = reactive({

  cny: {
    initialBalance: 100000,
    singleRisk: 2,
    monthlyRisk: 6
  },

  usd: {
    initialBalance: 10000,
    singleRisk: 2,
    monthlyRisk: 6
  }

})


// 人民币风险额度
const cnySingleRiskAmount = computed(() => {
  return accounts.cny.initialBalance
      * accounts.cny.singleRisk
      / 100
})

const cnyMonthlyRiskAmount = computed(() => {
  return accounts.cny.initialBalance
      * accounts.cny.monthlyRisk
      / 100
})


// 美元风险额度
const usdSingleRiskAmount = computed(() => {
  return accounts.usd.initialBalance
      * accounts.usd.singleRisk
      / 100
})

const usdMonthlyRiskAmount = computed(() => {
  return accounts.usd.initialBalance
      * accounts.usd.monthlyRisk
      / 100
})


function save() {
  insert_account_setting(USD_ACCOUNT_SETTING, {
    initialBalance: accounts.usd.initialBalance,
    singleRisk: accounts.usd.singleRisk,
    monthlyRisk: accounts.usd.monthlyRisk
  })

  insert_account_setting(CNY_ACCOUNT_SETTING, {
    initialBalance: accounts.cny.initialBalance,
    singleRisk: accounts.cny.singleRisk,
    monthlyRisk: accounts.cny.monthlyRisk
  })

  showToast('保存成功')
}


onMounted(async () => {
  let usdAccount = await find_account(USD_ACCOUNT_SETTING);
  if (usdAccount) {
    accounts.usd = usdAccount
    console.log('--', usdAccount, '00', usdAccount);
  }

  let cnyAccount = await find_account(CNY_ACCOUNT_SETTING);

  if (cnyAccount) {
    accounts.cny = cnyAccount
    console.log('--', usdAccount, '00', cnyAccount);
  }


})
</script>