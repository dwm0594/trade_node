<script setup lang="ts">
import {onMounted, onUnmounted, ref, reactive} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {useRouter} from "vue-router";
import {TradeForm} from "../types/entity";
import {formatDate, formatDuration} from "../utils/util";
import {emitter} from "../event/eventBus";
import {showToast} from "../toast/useToast";

const router = useRouter();

const nodes = ref<TradeForm[]>([]);

// 当前查询条件，删除后按原条件刷新
type HistoryQuery =
    | { type: "days"; day: number }
    | { type: "range"; start: number; end: number };

let currentQuery: HistoryQuery = {type: "days", day: 7};

// 右键菜单状态
const contextMenu = reactive({
  visible: false,
  id: 0,
  x: 0,
  y: 0
});

// 查询历史数据
async function find_history_data(day: number) {
  currentQuery = {type: "days", day};
  await loadHistory();
}

async function find_history_data_by_range(start: number, end: number) {
  currentQuery = {type: "range", start, end};
  await loadHistory();
}

async function loadHistory() {
  try {
    if (currentQuery.type === "days") {
      nodes.value = await invoke("find_trade_node", {
        day: currentQuery.day
      });
    } else {
      nodes.value = await invoke("find_trade_node_by_range", {
        start: currentQuery.start,
        end: currentQuery.end
      });
    }
  } catch (e) {
    console.error("查询失败:", e);
  }
}

function get_profit(node: TradeForm) {
  let profit = 0;

  if (node.closePosition !== undefined) {
    node.closePosition.forEach((item: { profit: number }) => {
      profit += item.profit;
    });
  }

  return profit;
}

function get_open_position_time(node: TradeForm) {
  return formatDate(Number(node.date));
}

function formatProfit(node: TradeForm): string {
  const profit = get_profit(node);

  if (profit === 0) {
    return "";
  }

  const sign = profit > 0 ? "+" : "";

  return `${sign}${profit}${node.currencyType === "人民币" ? "¥" : "$"}`;
}

function get_open_position_number(node: TradeForm): string {
  if (node.openPosition !== undefined && node.openPosition.length > 0) {
    let total = 0;

    node.openPosition.forEach((item: { total: number }) => {
      total += item.total;
    });

    return `${String(total)}${node.currencyType === "人民币" ? "¥" : "$"}`;
  }

  return "";
}

function get_holding_position_time(node: TradeForm) {
  if (node.holdingTime == null) {
    return "未开仓";
  }

  return `持仓 ${formatDuration(node.holdingTime)}`;
}

function goToBacktracking(id: number) {
  router.push(`/backtrace/${id}`);
}

// 右键打开菜单
function showContextMenu(event: MouseEvent, id: number) {
  const menuWidth = 120;
  const menuHeight = 45;

  contextMenu.id = id;
  contextMenu.x = Math.max(
      0,
      Math.min(event.clientX, window.innerWidth - menuWidth)
  );
  contextMenu.y = Math.max(
      0,
      Math.min(event.clientY, window.innerHeight - menuHeight)
  );
  contextMenu.visible = true;
}

// 关闭菜单
function closeContextMenu() {
  contextMenu.visible = false;
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") {
    closeContextMenu();
  }
}

// 删除记录
async function deleteNode() {
  const id = contextMenu.id;

  closeContextMenu();

  if (!confirm("确定删除这条交易记录吗？")) {
    return;
  }

  try {
    // 请确保 Rust 端注册了这个命令。
    // 如果你的删除命令名称不同，请替换 delete_trade_node。
    await invoke("delete_trade_node", {id});

    // 按当前筛选条件刷新列表
    await loadHistory();

  } catch (e:any) {
    showToast(e.toString(), 'error');
  }
}

// 刷新当前列表，保留原查询条件
const refreshHistory = async () => {
  await loadHistory();
};

const search = async (data: { start: number; end: number }) => {
  await find_history_data_by_range(data.start, data.end);
};

onMounted(async () => {
  document.addEventListener("click", closeContextMenu);
  document.addEventListener("keydown", handleKeydown);

  emitter.on("trade-node-updated", refreshHistory);
  emitter.on("search", search);

  await find_history_data(7);
});

onUnmounted(() => {
  document.removeEventListener("click", closeContextMenu);
  document.removeEventListener("keydown", handleKeydown);

  emitter.off("trade-node-updated", refreshHistory);
  emitter.off("search", search);
});
</script>

<template>
  <div class="list bg-base-100 flex flex-col h-full px-0.5">

    <!-- 搜索框 -->
    <div class="shrink-0 items-center gap-2 m-1 grid grid-cols-3">
      <input
          type="radio"
          name="tradeType"
          class="btn flex-1 min-w-0 px-1"
          aria-label="一周"
          :checked="currentQuery.type === 'days' && currentQuery.day === 7"
          @click="find_history_data(7)"
      />

      <input
          type="radio"
          name="tradeType"
          class="btn flex-1 min-w-0 px-1"
          aria-label="一月"
          :checked="currentQuery.type === 'days' && currentQuery.day === 30"
          @click="find_history_data(30)"
      />

      <input
          type="radio"
          name="tradeType"
          class="btn flex-1 min-w-0 px-1"
          aria-label="半年"
          :checked="currentQuery.type === 'days' && currentQuery.day === 180"
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
          @contextmenu.prevent.stop="showContextMenu($event, node.id)"
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

          <!-- 开仓量 -->
          <div class="ml-2 text-sm font-semibold">
            {{ get_open_position_number(node) }}
          </div>

          <!-- 盈利 -->
          <div
              class="ml-2 text-sm font-semibold"
              :class="get_profit(node) <= 0
                ? 'text-red-500'
                : 'text-green-500'"
          >
            {{ formatProfit(node) }}
          </div>

        </div>

        <!-- 开仓时间 + 持仓 -->
        <div
            class="mt-1 pl-8 flex items-center justify-between
                   text-xs text-gray-400"
        >
          <div>
            {{ get_open_position_time(node) }}
          </div>

          <div>
            {{ get_holding_position_time(node) }}
          </div>
        </div>

      </div>
    </div>

    <!-- 右键菜单 -->
    <div
        v-if="contextMenu.visible"
        class="fixed z-[100] w-28 rounded-lg
               border border-gray-200 bg-white
               py-1 shadow-lg"
        :style="{
          left: `${contextMenu.x}px`,
          top: `${contextMenu.y}px`
        }"
        @click.stop
        @contextmenu.prevent.stop
    >
      <button
          class="flex w-full items-center gap-2
                 px-3 py-2 text-left text-sm
                 text-red-600 hover:bg-red-50"
          @click="deleteNode"
      >
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
          <path d="M3 6h18"/>
          <path d="M8 6V4h8v2"/>
          <path d="m19 6-1 14H6L5 6"/>
          <path d="M10 11v5"/>
          <path d="M14 11v5"/>
        </svg>

        删除记录
      </button>
    </div>

  </div>
</template>

<style scoped>
</style>
