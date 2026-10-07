import {Position, TradeForm} from "../types/entity";
import {invoke} from "@tauri-apps/api/core";

/**
 * 时间戳转为日期  4/12 12:21
 * @param timestamp
 */
export function formatDate(timestamp: number): string {
    const date = new Date(timestamp)

    const month = date.getMonth() + 1
    const day = date.getDate()
    const hours = String(date.getHours()).padStart(2, '0')
    const minutes = String(date.getMinutes()).padStart(2, '0')

    return `${month}/${day} ${hours}:${minutes}`
}

export function formatDateForDay(timestamp: number): string {
    const date = new Date(timestamp)

    const year = date.getFullYear()
    const month = String(date.getMonth() + 1).padStart(2, '0')
    const day = String(date.getDate()).padStart(2, '0')

    return `${year}-${month}-${day}`
}


export function formatDateTimeLocal(timestamp: number) {
    const date = new Date(timestamp)

    const pad = (n: number) => String(n).padStart(2, '0')

    return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}T${pad(date.getHours())}:${pad(date.getMinutes())}`
}


export function maxTime() {
    const now = new Date()

    const pad = (n: number) => String(n).padStart(2, '0')

    return `${now.getFullYear()}-${pad(now.getMonth() + 1)}-${pad(now.getDate())}` +
        `T${pad(now.getHours())}:${pad(now.getMinutes())}`
}

/**
 * 时间戳转为持仓时间  2时21分3秒
 * @param ms
 */
export function formatDuration(ms: number): string {
    let seconds = Math.floor(ms / 1000)

    const days = Math.floor(seconds / 86400)
    seconds %= 86400

    const hours = Math.floor(seconds / 3600)
    seconds %= 3600

    const minutes = Math.floor(seconds / 60)
    const secs = seconds % 60

    const result = []

    if (days > 0) result.push(`${days}日`)
    if (hours > 0) result.push(`${hours}时`)
    if (minutes > 0) result.push(`${minutes}分`)
    if (secs > 0) result.push(`${secs}秒`)

    return result.join("")
}

export function createPosition(type: string): Position {
    return {
        id: 0,
        positionType: type,
        date: '',
        price: 0.0,
        quantity: 0.0,
        total: 0.0,
        topPrice: 0.0,
        lowPrice: 0.0,
        score: 0.0,
        reason: '',
        profit: 0.0,
    }
}

export function getScore(direction: string, node: Position, type: string) {
    if (node.lowPrice == 0 || node.topPrice == 0 || node.price == 0) return 0;

    let range = Math.abs(node.topPrice - node.lowPrice);
    let b = 0

    if (direction == '多' && type == 'open') {
        b = Math.abs(node.topPrice - node.price);
    }

    if (direction == '多' && type == 'close') {
        b = Math.abs(node.lowPrice - node.price);
    }


    if (direction == '空' && type == 'open') {
        b = Math.abs(node.lowPrice - node.price);
    }

    if (direction == '空' && type == 'close') {
        b = Math.abs(node.topPrice - node.price);
    }


    node.score = b / range
}

export function getHoldingTime(node: TradeForm): number {
    const first = node.openPosition?.[0]
    const last = node.closePosition?.at(-1)

    if (!first || !last) {
        return 0
    }

    const firstTime = new Date(first.date).getTime()
    const lastTime = new Date(last.date).getTime()


    return lastTime - firstTime
}


export async function getFilePaths(json: string): Promise<string[]> {

    const exeDir = await invoke<string>('get_exe_dir')

    try {
        const files = JSON.parse(json) as string[]

        return files.map(file => {
            return `${exeDir}\\images\\${file}`
        })
    } catch (e) {
        console.error('screenshots JSON 解析失败:', e)
        return []
    }
}


export function removeImage(form: TradeForm, index: number) {
    form.filepath.splice(index, 1)
}

export function getProfitAndLossRate(form: TradeForm) {

    let profit = Math.abs(form.expect - form.profit);
    let loss = Math.abs(form.expect - form.stopLoss);

    if (loss > 0 && profit != loss) {
        form.profitLossRatio = Number((profit / loss).toFixed(2))
    }


    if (form.expect != 0.0 && form.stopLoss != 0.0) {
        form.allowableMaxNum = Math.floor((form.allowableMaxLoss / Math.abs(form.expect - form.stopLoss)))
    }

}