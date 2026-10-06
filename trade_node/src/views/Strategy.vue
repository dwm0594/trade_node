<script setup lang="ts">

import {onMounted, ref} from 'vue'
import {find_dict, insert_dict} from "../invoke/sql";
import {invoke} from "@tauri-apps/api/core";
import {showToast} from "../toast/useToast";
import {Dictionary} from "../types/entity";

const tradeStrategy = ref({
  name: ''
})

const messageStrategy = ref({
  name: ''
})

const tradeDictionary = ref<Dictionary[]>([])

const messageDictionary = ref<Dictionary[]>([])

async function addMessageSourceStrategy(name: string) {

  if (name === "") {
    showToast('无法添加空来源', 'error')
    return
  }

  const value: Dictionary | null = await insert_dict('message_source', name)
  if (value !== null) {
    messageDictionary.value.push(value)
  }

  await find_dict('message_source')
}


async function addTradeNodeStrategy(name: string) {

  if (name === "") {
    showToast('无法添加空策略', 'error')
    return
  }

  const value: Dictionary | null = await insert_dict('trade_strategy', name)
  if (value !== null) {
    tradeDictionary.value.push(value)
  }
}


async function deleteStrategy(id: number) {
  const size = await invoke("delete_strategy", {id})

  if (size !== 0) {
    showToast("删除成功")

    tradeDictionary.value = tradeDictionary.value.filter(node => node.id !== id)
  } else {
    showToast("删除失败", 'error')
  }
}

onMounted(async () => {
  tradeDictionary.value = await find_dict('trade_strategy')
  messageDictionary.value = await find_dict('message_source')
})

</script>

<template>

  <div class="h-full p-3 bg-base-200">


    <!-- 页面标题 -->
    <div class="mb-6">
      <h1 class="text-2xl font-semibold">策略管理</h1>
      <p class="text-sm text-base-content/50 mt-1">
        管理交易日志中可以选择的策略
      </p>
    </div>

    <div class="grid grid-cols-1 gap-3">
      <!-- 添加交易策略 -->
      <div class="h-100 card grid grid-cols-2 bg-base-100 shadow-sm gap-5">

        <div class="h-full min-h-0 card-body">

          <h2 class="card-title">
            添加交易策略
          </h2>

          <div class="divider my-1"></div>

          <!-- 策略名称 -->
          <fieldset class="fieldset">
            <legend class="fieldset-legend">
              策略名称
            </legend>

            <input
                v-model="tradeStrategy.name"
                type="text"
                placeholder="例如：趋势突破"
                class="input input-bordered w-full"
            />
          </fieldset>

          <!-- 按钮 -->
          <div class="card-actions justify-end mt-5">
            <button
                class="btn btn-primary"
                @click="addTradeNodeStrategy(tradeStrategy.name)"
            >
              添加策略
            </button>
          </div>

        </div>

        <div class="h-full min-h-0 card-body">
          <h2 class="card-title">
            现有交易策略
          </h2>
          <div class="divider my-1"></div>

          <ul
              class="flex-1 min-h-0 overflow-y-auto
         rounded-xl border border-base-200 p-2
         bg-base-100"
          >
            <li
                v-for="node in tradeDictionary"
                :key="node.id"
                class="flex items-center justify-between
           px-4 py-3
           border-b border-base-200 last:border-b-0
           hover:bg-base-200/50"
            >
              <div class="flex min-w-0 items-center gap-3">

                <!-- 名称 -->
                <span class="truncate text-sm font-medium">
        {{ node.name }}
      </span>
              </div>

              <button
                  class="btn btn-ghost btn-sm btn-square
             text-base-content/30
             hover:bg-error/10 hover:text-error"
                  title="删除策略"
                  @click.stop="deleteStrategy(node.id)"
              >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-4 w-4"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                >
                  <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6M9 7V4a1 1 0 011-1h4a1 1 0 011 1v3m-9 0h12"
                  />
                </svg>
              </button>
            </li>
          </ul>
        </div>

      </div>

      <!--      添加消息来源-->
      <div class="h-100 card grid grid-cols-2 bg-base-100 shadow-sm gap-5">

        <div class="h-full min-h-0 card-body">

          <h2 class="card-title">
            添加消息来源
          </h2>

          <div class="divider my-1"></div>

          <!-- 策略名称 -->
          <fieldset class="fieldset">
            <legend class="fieldset-legend">
              消息来源名称
            </legend>

            <input
                v-model="messageStrategy.name"
                type="text"
                placeholder="例如：财经新闻"
                class="input input-bordered w-full"
            />
          </fieldset>

          <!-- 按钮 -->
          <div class="card-actions justify-end mt-5">
            <button
                class="btn btn-primary"
                @click="addMessageSourceStrategy(messageStrategy.name)"
            >
              添加消息
            </button>
          </div>

        </div>

        <div class="h-full min-h-0 card-body">
          <h2 class="card-title">
            现有消息来源
          </h2>
          <div class="divider my-1"></div>

          <ul
              class="flex-1 min-h-0 overflow-y-auto
         rounded-xl border border-base-200 p-2
         bg-base-100"
          >
            <li
                v-for="node in messageDictionary"
                :key="node.id"
                class="flex items-center justify-between
           px-4 py-3
           border-b border-base-200 last:border-b-0
           hover:bg-base-200/50"
            >
              <div class="flex min-w-0 items-center gap-3">

                <!-- 名称 -->
                <span class="truncate text-sm font-medium">
        {{ node.name }}
      </span>
              </div>

              <button
                  class="btn btn-ghost btn-sm btn-square
             text-base-content/30
             hover:bg-error/10 hover:text-error"
                  title="删除策略"
                  @click.stop="deleteStrategy(node.id)"
              >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-4 w-4"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                >
                  <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6M9 7V4a1 1 0 011-1h4a1 1 0 011 1v3m-9 0h12"
                  />
                </svg>
              </button>
            </li>
          </ul>
        </div>

      </div>
    </div>
  </div>

</template>

<style scoped>

</style>