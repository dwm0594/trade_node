<script setup lang="ts">
import {convertFileSrc, invoke} from "@tauri-apps/api/core";
import {open} from "@tauri-apps/plugin-dialog";
import {onMounted, reactive, ref, watch} from "vue";
import {Dictionary, Position, TradeForm} from "../types/entity";
import {showToast} from "../toast/useToast";
import {find_dict} from "../invoke/sql";
import {getScoreClass} from "../assets/style";
import {RouteParamValue, useRoute} from "vue-router";
import {
  createPosition,
  getScore,
  getHoldingTime,
  getFilePaths,
  removeImage, maxTime, formatDateTimeLocal, getProfitAndLossRate
} from "../utils/util";
import {emitter} from "../event/eventBus";
import CurrencySymbol from "../components/CurrencySymbol.vue";


const form = reactive<TradeForm>({
  allowableMaxLoss: 0,
  allowableMaxNum: 0,
  id: 0,
  date: '',
  name: '',
  direction: '多',
  source: 0,
  strategy: 0,
  // 目标位
  expect: 0.0,
  // 盈利位
  profit: 0.0,
  // 止损位
  stopLoss: 0.0,
  // 盈亏比
  profitLossRatio: 0.0,
  // 持仓时间
  holdingTime: 0,
  currencyType: '人民币',
  openPosition: [] as Position[],
  closePosition: [] as Position[],
  filepath: [] as string[],
  screenshots: '',
  backtracking: ''
})


async function submitForm() {
  try {
    console.log('before form ', form)

    const data = {
      ...form,

      date: new Date(form.date).getTime(),

      openPosition: form.openPosition.map(item => ({
        ...item,
        date: new Date(item.date).getTime()
      })),

      closePosition: form.closePosition.map(item => ({
        ...item,
        date: new Date(item.date).getTime()
      })),

      screenshots: JSON.stringify(form.filepath),
      holdingTime: getHoldingTime(form)
    }

    console.log('submit data', data)

    await invoke("update_trade_node", {
      node: data
    });

    showToast("保存成功")

    emitter.emit('trade-node-updated')

    await load(String(data.id))

  } catch (e: any) {
    showToast(e.toString(), 'error');
  }

}

function addOpenPosition() {
  form.openPosition.push(createPosition('open'))
}

function removeOpenPosition(index: number) {
  form.openPosition.splice(index, 1)
}


function addClosePosition() {
  form.closePosition.push(createPosition("close"))
}

function removeClosePosition(index: number) {
  form.closePosition.splice(index, 1)
}


async function handleImage() {
  const selected = await open({
    multiple: true,
    directory: false,
    filters: [
      {
        name: '图片',
        extensions: ['png', 'jpg', 'jpeg', 'webp', 'gif']
      }
    ]
  })

  if (!selected) return

  let list: string[] = Array.isArray(selected)
      ? selected
      : [selected];

  list.forEach((item) => {
    form.filepath.push(item)
  })
}

const previewImage = ref('')

function openImage(path: string) {
  previewImage.value = convertFileSrc(path)
}


function closeAllPosition(index: number) {

  let quantity = 0;
  let money = 0;
  form.openPosition.forEach((position) => {
    quantity += position.quantity;
    money += position.total;
  })


  form.closePosition[index].quantity = quantity;
  form.closePosition[index].total = money;
}

const tradeDictionary = ref<Dictionary[]>([])

const messageDictionary = ref<Dictionary[]>([])

onMounted(async () => {
  tradeDictionary.value = await find_dict('trade_strategy');
  messageDictionary.value = await find_dict('message_source');
})


async function load(id: string | RouteParamValue[]) {
  const result = await invoke<TradeForm>(
      'find_trade_node_by_id',
      {
        id: Number(id)
      }
  )

  const data = {
    ...result,
    date: formatDateTimeLocal(Number(result.date)),

    openPosition: result.openPosition.map(item => ({
      ...item,
      date: formatDateTimeLocal(Number(item.date)),
    })),

    closePosition: result.closePosition.map(item => ({
      ...item,
      date: formatDateTimeLocal(Number(item.date)),
    })),

    filepath: await getFilePaths(result.screenshots)
  }

  Object.assign(form, data)
}

const route = useRoute()

watch(
    () => route.params.id,
    async (id) => {
      await load(id)
    },
    {
      immediate: true
    }
)

const dataStart = ref('')
const dataEnd = ref('')


function search() {

  if (dataStart.value == '' || dataEnd.value == '') {
    return
  }

  const startTimestamp = new Date(dataStart.value).getTime()
  const endTimestamp = new Date(dataEnd.value).getTime()

  emitter.emit('search', {
    start: startTimestamp,
    end: endTimestamp
  })
}

function searchByYear(year: number) {
  const currentYear = new Date().getFullYear()
  const targetYear = currentYear + year

  const start = new Date(targetYear, 0, 1, 0, 0, 0, 0).getTime()
  const end = new Date(targetYear + 1, 0, 1, 0, 0, 0, 0).getTime() - 1


  emitter.emit('search', {
    start: start,
    end: end
  })
}


</script>

<template>

  <form class="min-h-full bg-base-200 p-3 h-full overflow-y-auto" @submit.prevent="submitForm">

    <!-- 页面标题 -->
    <div class="max-w-9xl mx-auto mb-6">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-2xl font-bold">
            复盘交易日志
          </h1>
          <p class="text-sm text-base-content/60 mt-1">
            我们不应该犯重复的错。
          </p>
        </div>

        <div class="badge badge-primary badge-outline">
          复盘
        </div>
      </div>
    </div>


    <!-- 主体 -->
    <div class="max-w-9xl mx-auto gap-6">

      <!-- 交易信息 -->
      <div class="space-y-4">

        <!-- 时间选择 -->
        <div class="card bg-base-100 shadow-sm border border-base-200">
          <div class="card-body">
            <h2 class="card-title text-base">
              时间选择<span class=" text-xs text-gray-400">可在此灵活选择任意的时间范围,结果会展示到左侧</span>
            </h2>
            <div class="divider my-1"></div>

            <div class="flex gap-5">

              <!-- 日期 -->
              <fieldset class="fieldset">
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

              <fieldset class="fieldset">
                <div class="w-25">
                  <button type="button" class="btn btn-primary w-full" @click="search()">搜索</button>
                </div>
              </fieldset>

              <fieldset class="fieldset">
                <div class="w-25">
                  <button type="button" class="btn btn-primary w-full" @click="searchByYear(0)">今年</button>
                </div>
              </fieldset>

              <fieldset class="fieldset">
                <div class="w-25">
                  <button type="button" class="btn btn-primary w-full" @click="searchByYear(-1)">去年</button>
                </div>
              </fieldset>

              <fieldset class="fieldset">
                <div class="w-25">
                  <button type="button" class="btn btn-primary w-full" @click="searchByYear(-2)">前年</button>
                </div>
              </fieldset>

            </div>
          </div>
        </div>

        <div class="divider my-1"></div>

        <!-- 基本信息 -->
        <div class="card bg-base-100 shadow-sm border border-base-200">
          <div class="card-body">
            <h2 class="card-title text-base">
              交易信息
            </h2>
            <div class="divider my-1"></div>
            <div class="grid grid-cols-6 gap-5">

              <!-- 信息来源 -->
              <fieldset class="fieldset">
                <legend class="fieldset-legend">
                  信息来源
                </legend>

                <select v-model="form.source" class="select select-bordered w-full">
                  <option v-for="node in messageDictionary" :key="node.name" :value="node.value">{{
                      node.name
                    }}
                  </option>
                </select>

              </fieldset>

              <!-- 日期 -->
              <fieldset class="fieldset">
                <legend class="fieldset-legend">
                  交易日期
                </legend>

                <input
                    v-model="form.date"
                    class="input input-bordered w-full"
                    type="datetime-local"
                    :max="maxTime()"
                />
              </fieldset>


              <!-- 股票名称 -->
              <fieldset class="fieldset">
                <legend class="fieldset-legend">
                  名称
                </legend>

                <input
                    v-model="form.name"
                    type="text"
                    placeholder="例如：贵州茅台"
                    class="input input-bordered w-full"

                />
              </fieldset>


              <!-- 交易方向 -->
              <fieldset class="fieldset">
                <legend class="fieldset-legend">
                  交易方向
                </legend>

                <select v-model="form.direction" class="select select-bordered w-full" disabled>
                  <option>多</option>
                  <option>空</option>
                </select>

              </fieldset>

              <!-- 币种 -->
              <fieldset class="fieldset">
                <legend class="fieldset-legend">
                  币种
                </legend>

                <select v-model="form.currencyType" class="select select-bordered w-full" disabled>
                  <option>人民币</option>
                  <option>美元</option>
                </select>

              </fieldset>


              <!-- 交易策略 -->
              <fieldset class="fieldset">
                <legend class="fieldset-legend">
                  交易策略
                </legend>

                <select v-model="form.strategy" class="select select-bordered w-full">
                  <option v-for="node in tradeDictionary" :key="node.name" :value="node.value">{{ node.name }}</option>
                </select>

              </fieldset>


            </div>
          </div>
        </div>

        <!--        资金管理-->
        <div class="card bg-base-100 shadow-sm border border-base-200">
          <div class="card-body">
            <h2 class="card-title text-base">
              资金管理
            </h2>
            <div class="divider my-1"></div>
            <div class="grid grid-cols-4 gap-5">

              <!-- 目标价 -->
              <fieldset class="fieldset flex-1">
                <legend class="fieldset-legend">
                  预计入场价
                </legend>

                <label class="input input-bordered w-full">
                  <CurrencySymbol :currency-type="form.currencyType"/>

                  <input
                      v-model.number="form.expect"
                      type="number"
                      step="0.01"
                      placeholder="0.00"
                      @change="getProfitAndLossRate(form)"
                  />
                </label>
              </fieldset>

              <!-- 止损 -->
              <fieldset class="fieldset flex-1">
                <legend class="fieldset-legend">
                  止损价
                </legend>

                <label class="input input-bordered w-full">
                  <CurrencySymbol :currency-type="form.currencyType"/>


                  <input
                      v-model.number="form.stopLoss"
                      type="number"
                      step="0.01"
                      placeholder="0.00"
                      @change="getProfitAndLossRate(form)"
                  />
                </label>
              </fieldset>

              <!-- 盈利价 -->
              <fieldset class="fieldset flex-1">
                <legend class="fieldset-legend">
                  盈利价
                </legend>

                <label class="input input-bordered w-full">
                  <CurrencySymbol :currency-type="form.currencyType"/>

                  <input
                      v-model.number="form.profit"
                      type="number"
                      step="0.01"
                      placeholder="0.00"
                      @change="getProfitAndLossRate(form)"
                  />
                </label>
              </fieldset>


              <!-- 盈亏比 -->
              <fieldset class="fieldset flex-1">
                <legend class="fieldset-legend">
                  盈亏比
                </legend>

                <label
                    class="input w-full"
                    :class="form.profitLossRatio < 2 &&  form.profitLossRatio != 0
      ? 'border-error bg-error/10 text-error'
      : 'border-base-300'"
                >
                  <input
                      v-model.number="form.profitLossRatio"
                      type="number"
                      step="0.01"
                      disabled
                  />
                </label>
              </fieldset>

            </div>
          </div>
        </div>


        <!-- 开仓-->
        <div class="card bg-base-100 shadow-sm border border-base-200">
          <div class="card-body">

            <div class="flex items-center justify-between">

              <h2 class="card-title text-base">
                开仓
              </h2>

              <button
                  type="button"
                  class="btn btn-sm btn-outline btn-primary"
                  @click="addOpenPosition()"
              >
                + 开仓
              </button>

            </div>

            <div class="divider my-1"></div>

            <!--            for -->
            <div
                v-for="(trade, index) in form.openPosition"
                :key="index"
                class="mb-4"
            >

              <div class="flex items-end gap-3">
                <!-- 日期 -->
                <fieldset class="fieldset  w-60">
                  <legend class="fieldset-legend">
                    时间
                  </legend>

                  <input v-model="trade.date" class="input input-bordered w-full" type="datetime-local"
                         :max="maxTime()"/>
                </fieldset>

                <!-- 单价 -->
                <fieldset class="fieldset flex-1">
                  <legend class="fieldset-legend">
                    单价
                  </legend>

                  <label class="input input-bordered w-full">
                    <CurrencySymbol :currency-type="form.currencyType"/>

                    <input
                        v-model.number="trade.price"
                        type="number"
                        step="0.01"
                        placeholder="0.00"
                        @change="getScore(form.direction,trade,'open')"
                    />
                  </label>
                </fieldset>


                <!-- 数量 -->
                <fieldset class="fieldset flex-1">
                  <legend class="fieldset-legend">
                    数量
                  </legend>

                  <input
                      v-model.number="trade.quantity"
                      type="number"
                      placeholder="0.0"
                      step="0.01"
                      class="input input-bordered w-full"
                  />
                </fieldset>

                <!-- 总数 -->
                <fieldset class="fieldset flex-1">
                  <legend class="fieldset-legend">
                    总额(含佣)
                  </legend>

                  <label class="input input-bordered w-full">
                    <CurrencySymbol :currency-type="form.currencyType"/>


                    <input
                        v-model.number="trade.total"
                        type="number"
                        step="0.01"
                        placeholder="0.00"
                    />
                  </label>

                </fieldset>


                <fieldset class="fieldset flex-1">
                  <legend class="fieldset-legend">
                    当天最高价
                  </legend>

                  <label class="input input-bordered w-full">
                    <CurrencySymbol :currency-type="form.currencyType"/>


                    <input
                        v-model.number="trade.topPrice"
                        type="number"
                        step="0.01"
                        placeholder="0.00"
                        @change="getScore(form.direction,trade,'open')"
                    />
                  </label>

                </fieldset>

                <fieldset class="fieldset flex-1">
                  <legend class="fieldset-legend">
                    当天最低价
                  </legend>

                  <label class="input input-bordered w-full">
                    <CurrencySymbol :currency-type="form.currencyType"/>


                    <input
                        v-model.number="trade.lowPrice"
                        type="number"
                        step="0.01"
                        placeholder="0.00"
                        @change="getScore(form.direction,trade,'open')"
                    />
                  </label>

                </fieldset>

                <!-- 评级 -->
                <fieldset class="fieldset w-40">
                  <legend class="fieldset-legend">
                    评级
                  </legend>

                  <div
                      class="input input-bordered w-full transition-all duration-300"
                      :class="getScoreClass(trade.score)"
                  >
                    {{ (trade.score * 100).toFixed(2) }}%
                  </div>
                </fieldset>


                <!-- 删除 -->
                <button
                    v-if="form.openPosition.length > 1"
                    type="button"
                    class="btn btn-square btn-ghost text-error mb-1"
                    @click="removeOpenPosition(index)"
                >
                  ×
                </button>


              </div>

              <div class="mt-2">

                <fieldset class="fieldset">
              <textarea
                  v-model="trade.reason"
                  class="textarea textarea-bordered w-full h-12 resize-none"
                  placeholder="请输入你的开仓理由..."
              ></textarea>
                </fieldset>

              </div>

            </div>

          </div>

        </div>

        <!-- 平仓-->
        <div class="card bg-base-100 shadow-sm border border-base-200">
          <div class="card-body">

            <div class="flex items-center justify-between">

              <h2 class="card-title text-base">
                平仓
              </h2>

              <button
                  type="button"
                  class="btn btn-sm btn-outline btn-primary"
                  @click="addClosePosition()"
              >
                + 平仓
              </button>

            </div>

            <div class="divider my-1"></div>

            <div
                v-for="(trade, index) in form.closePosition"
                :key="index"
                class="mb-4"
            >

              <div class="flex items-end gap-3">
                <!-- 日期 -->
                <fieldset class="fieldset  w-60">
                  <legend class="fieldset-legend">
                    时间
                  </legend>

                  <input v-model="trade.date" class="input input-bordered w-full" type="datetime-local"
                         :max="maxTime()"/>
                </fieldset>

                <!-- 价格 -->
                <fieldset class="fieldset flex-1">
                  <legend class="fieldset-legend">
                    价格
                  </legend>

                  <label class="input input-bordered w-full">
                    <CurrencySymbol :currency-type="form.currencyType"/>


                    <input
                        v-model.number="trade.price"
                        type="number"
                        step="0.01"
                        placeholder="0.00"
                        @change="getScore(form.direction,trade,'close')"
                    />
                  </label>
                </fieldset>


                <!-- 数量 -->
                <fieldset class="fieldset flex-1">
                  <legend class="fieldset-legend">
                    数量
                  </legend>

                  <div class="flex gap-2 ">
                    <input
                        v-model.number="trade.quantity"
                        type="number"
                        placeholder="0"
                        step="0.01"
                        class="input input-bordered flex-1"
                    >
                  </div>

                </fieldset>

                <fieldset class="fieldset w-70">
                  <legend class="fieldset-legend">
                    总额(含佣)
                  </legend>

                  <div class="flex gap-2 ">

                    <label class="input input-bordered w-full">
                      <CurrencySymbol :currency-type="form.currencyType"/>

                      <input
                          v-model.number="trade.total"
                          type="number"
                          placeholder="0"
                          step="0.01"
                          class="input input-bordered flex-1"
                      >
                    </label>

                    <button
                        type="button"
                        class="btn btn-info"
                        @click="closeAllPosition(index)"
                    >
                      全平
                    </button>
                  </div>

                </fieldset>


                <fieldset class="fieldset flex-1">
                  <legend class="fieldset-legend">
                    当天最高价
                  </legend>

                  <label class="input input-bordered w-full">
                    <CurrencySymbol :currency-type="form.currencyType"/>


                    <input
                        v-model.number="trade.topPrice"
                        type="number"
                        step="0.01"
                        placeholder="0.00"
                        @change="getScore(form.direction,trade,'close')"
                    />
                  </label>

                </fieldset>

                <fieldset class="fieldset flex-1">
                  <legend class="fieldset-legend">
                    当天最低价
                  </legend>

                  <label class="input input-bordered w-full">
                    <CurrencySymbol :currency-type="form.currencyType"/>


                    <input
                        v-model.number="trade.lowPrice"
                        type="number"
                        step="0.01"
                        placeholder="0.00"
                        @change="getScore(form.direction,trade,'close')"
                    />
                  </label>

                </fieldset>


                <fieldset class="fieldset flex-1">

                  <legend class="fieldset-legend">
                    利润
                  </legend>

                  <label class="input input-bordered w-full">
                    <CurrencySymbol :currency-type="form.currencyType"/>

                    <input
                        v-model.number="trade.profit"
                        type="number"
                        step="0.01"
                        placeholder="0.00"
                    />
                  </label>

                </fieldset>


                <!-- 评级 -->
                <fieldset class="fieldset w-40">
                  <legend class="fieldset-legend">
                    评级
                  </legend>

                  <div
                      class="input input-bordered w-full transition-all duration-300"
                      :class="getScoreClass(trade.score)"
                  >
                    {{ (trade.score * 100).toFixed(2) }}%
                  </div>
                </fieldset>


                <!-- 删除 -->
                <button
                    v-if="form.closePosition.length > 1"
                    type="button"
                    class="btn btn-square btn-ghost text-error mb-1"
                    @click="removeClosePosition(index)"
                >
                  ×
                </button>


              </div>


              <div class="mt-2">

                <fieldset class="fieldset">
              <textarea
                  v-model="trade.reason"
                  class="textarea textarea-bordered w-full h-12 resize-none"
                  placeholder="请输入你的平仓理由..."
              ></textarea>
                </fieldset>

              </div>

            </div>

          </div>

        </div>

        <!-- 交易截图-->
        <div class="card bg-base-100 shadow-sm border border-base-200">
          <div class="card-body">
            <h2 class="card-title text-base">
              交易截图
            </h2>
            <div class="divider my-1"></div>
            <div>

              <button
                  type="button"
                  class="btn btn-primary w-full"
                  @click="handleImage"
              >
                选择图片
              </button>

              <!-- 图片列表 -->
              <div class="flex flex-wrap gap-4 mt-4">
                <div
                    v-for="(path, index) in form.filepath"
                    :key="path"
                    class="relative group"
                >
                  <img
                      :src="convertFileSrc(path)"
                      class="max-w-full max-h-64 rounded-lg cursor-pointer"
                      @dblclick="openImage(path)"
                  />

                  <!-- 删除按钮 -->
                  <button
                      class="
          absolute top-2 right-2
          w-8 h-8
          rounded-full
          bg-black/50
          backdrop-blur-sm
          text-white
          flex items-center justify-center
          opacity-0 group-hover:opacity-100
          transition-all duration-200
          hover:bg-red-500
          hover:scale-110
        "
                      title="删除图片"
                      @click.stop="removeImage(form,index)"
                  >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        class="w-4 h-4"
                    >
                      <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          d="M6 7h12M9 7V5h6v2m-7 0 1 12h6l1-12M10 11v5M14 11v5"
                      />
                    </svg>
                  </button>
                </div>
              </div>

              <!-- 原图预览弹窗 -->
              <div
                  v-if="previewImage"
                  class="fixed inset-0 z-50 flex items-center justify-center bg-black/80"
                  @click="previewImage = ''"
              >
                <img
                    :src="previewImage"
                    class="max-w-[90vw] max-h-[90vh] object-contain"
                    @click.stop
                />
              </div>

            </div>
          </div>
        </div>


        <!-- 总结-->
        <div class="card bg-base-100 shadow-sm border border-base-200">
          <div class="card-body">

            <div class="flex items-center">

              <h2 class="card-title text-base">
                总结
              </h2>

              <span class="ml-2 text-xs text-gray-400">
    不必贬低自己
  </span>

            </div>

            <div class="divider my-1"></div>

            <fieldset class="fieldset">
              <textarea
                  v-model="form.backtracking"
                  class="textarea textarea-bordered w-full h-12 resize-none"
                  placeholder="这一单做的如何？"
              ></textarea>
            </fieldset>

          </div>

        </div>

      </div>
      <!-- 底部操作 -->
      <div class="max-w-9xl mx-auto mt-6 flex justify-end gap-3">

        <button type="submit" class="btn btn-primary px-8">
          保存交易日志
        </button>

      </div>

    </div>

  </form>

</template>

<style scoped>

</style>