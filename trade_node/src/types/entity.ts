export interface TradeForm {
    id: number
    date: string
    name: string
    direction: string
    source: number
    strategy: number
    expect: number
    profit: number
    stopLoss: number
    profitLossRatio: number
    // 允许单笔最大开仓
    allowableMaxNum: number,
    // 允许单笔最大损失
    allowableMaxLoss: number
    holdingTime: number
    currencyType: string
    openPosition: Position[]
    closePosition: Position[]
    filepath: string[],
    screenshots: string,
    backtracking: string
}

export interface Dictionary {
    id: number
    strategy: string
    name: string
    value: number
}

export interface Position {
    // open or close
    id: number
    positionType: string
    // 日期
    date: string
    // 开仓 平仓价
    price: number
    // 数量
    quantity: number
    // 总额
    total: number
    // 最高价
    topPrice: number
    // 最低价
    lowPrice: number
    // 评分
    score: number
    // 原因
    reason: string
    // 盈利 (平仓时会有盈利,正或者负)
    profit: number
}


export interface DashboardAnalysis {
    /** 单数 */
    size: number


    // 总盈亏
    profitLoss: number
    /** 盈利 */
    profit: number

    /** 平均盈利 */
    aveProfit: number

    /** 亏损 */
    loss: number

    /** 平均亏损 */
    aveLoss: number

    /** 盈利数 */
    profitCount: number

    /** 亏损数 */
    lossCount: number
    // 盈亏比
    profitLossRatio: number
    /** 胜率 */
    profitRatio: number

    /** 平均持仓时间 */
    aveHoldingTime: number

    /** 亏损平均持仓时间 */
    aveLossHoldingTime: number

    /** 盈利平均持仓时间 */
    aveProfitHoldingTime: number

    /** 最大最小持仓时间 */
    maxHoldingTime: number
    minHoldingTime: number

    /** 平均开仓平仓评分 */
    aveOpenPositionScore: number
    aveClosePositionScore: number

    /** 策略统计 */
    strategyAnalysis: BehaviorAnalysis[]

    /** 消息来源统计 */
    sourceAnalysis: BehaviorAnalysis[]
}

export interface BehaviorAnalysis {
    /** 名称 */
    name: string

    /** 标记值 */
    value: number

    /** 使用次数 */
    useCount: number

    /** 使用率 */
    useRatio: number

    /** 盈利次数 */
    profitCount: number

    /** 盈利 */
    profit: number

    /** 亏损 */
    loss: number

    /** 亏损次数 */
    lossCount: number

    /** 胜率 */
    profitRatio: number


    /** 盈亏比 */
    profitLossRatio: number
}


export interface AccountSetting {
    initialBalance: number
    monthlyRisk: number
    singleRisk: number
}