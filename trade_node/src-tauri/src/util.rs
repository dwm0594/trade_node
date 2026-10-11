use crate::command::CommandError;
use crate::db::repository::{find_dictionaries_by_type, TradeNode, MESSAGE_SOURCE};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::iter::Map;
use std::path::PathBuf;
use tauri::Pixel;
use uuid::Uuid;

pub fn copy_screenshots(screenshots: Option<&String>) -> Result<Option<String>, String> {
    let Some(screenshots) = screenshots else {
        return Ok(None);
    };

    let source_paths: Vec<String> =
        serde_json::from_str(screenshots).map_err(|e| format!("JSON解析失败: {}", e))?;

    // 当前 exe 所在目录
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("获取程序目录失败: {}", e))?
        .parent()
        .ok_or("获取程序目录失败")?
        .to_path_buf();

    // exe 同级 images 目录
    let image_dir = exe_dir.join("images");

    fs::create_dir_all(&image_dir).map_err(|e| format!("创建图片目录失败: {}", e))?;

    let mut new_paths = Vec::new();

    for source in source_paths {
        let source_path = PathBuf::from(&source);

        let extension = source_path
            .extension()
            .and_then(|x| x.to_str())
            .unwrap_or("png");

        let filename = format!("{}.{}", Uuid::new_v4(), extension);
        let target_path = image_dir.join(&filename);

        fs::copy(&source_path, &target_path).map_err(|e| format!("复制图片失败: {}", e))?;

        // 建议只保存文件名
        new_paths.push(filename);
    }

    let result = serde_json::to_string(&new_paths).map_err(|e| format!("JSON序列化失败: {}", e))?;

    Ok(Some(result))
}

pub fn delete_screenshots(screenshots: Option<&String>) -> Result<(), String> {
    let Some(screenshots) = screenshots else {
        return Ok(());
    };

    let source_paths: Vec<String> =
        serde_json::from_str(screenshots).map_err(|e| format!("JSON解析失败: {}", e))?;

    // 当前 exe 所在目录
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("获取程序目录失败: {}", e))?
        .parent()
        .ok_or("获取程序目录失败")?
        .to_path_buf();

    // exe 同级 images 目录
    let image_dir = exe_dir.join("images");

    for source in source_paths {
        let source_path = image_dir.join(&source);

        if source_path.is_file() {
            std::fs::remove_file(&source_path)
                .map_err(|e| format!("删除图片失败 {:?}: {}", source_path, e))?;
        }
    }

    
    Ok(())
}

pub fn delete_old_picture(node: &TradeNode) -> Result<(), CommandError> {
    let Some(json_path) = &node.screenshots else {
        return Ok(());
    };

    let paths: Vec<String> =
        serde_json::from_str(json_path).map_err(|e| format!("解析图片路径失败: {}", e))?;

    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("获取程序目录失败: {}", e))?
        .parent()
        .ok_or_else(|| "获取程序目录失败".to_string())?
        .to_path_buf();

    let image_dir = exe_dir.join("images");

    for filename in paths {
        let path = image_dir.join(&filename);

        // 文件不存在就跳过
        if !path.exists() {
            continue;
        }

        fs::remove_file(&path)
            .map_err(|e| format!("删除图片失败: {}，错误: {}", path.display(), e))?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_max() {
        println!("i32 max :{} min {}", i64::MAX, i64::MIN);
    }
}

/***
行为分析 包括策略和来源
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BehaviorAnalysis {
    // 名称
    pub name: String,
    // 标记值
    pub value: i32,
    // 使用次数
    pub use_count: i32,
    // 使用率
    pub use_ratio: f64,
    // 盈利次数
    pub profit_count: i32,
    // 盈利
    pub profit: f64,
    // 亏损
    pub loss: f64,
    // 亏损次数
    pub loss_count: i32,
    // 胜率
    pub profit_ratio: f64,
    // 盈亏比
    pub profit_loss_ratio: f64,
}

impl BehaviorAnalysis {
    fn new(value: i32) -> BehaviorAnalysis {
        BehaviorAnalysis {
            name: "".to_string(),
            value,
            use_count: 0,
            use_ratio: 0.0,
            profit_count: 0,
            profit: 0.0,
            loss: 0.0,
            loss_count: 0,
            profit_ratio: 0.0,
            profit_loss_ratio: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DashboardAnalysis {
    // 单数
    pub size: usize,
    // 总盈亏
    pub profit_loss: f64,
    // 盈利
    pub profit: f64,
    // 平均盈利
    pub ave_profit: f64,
    // 亏损
    pub loss: f64,
    // 平均亏损
    pub ave_loss: f64,
    // 盈利数
    pub profit_count: i32,
    // 亏损数
    pub loss_count: i32,
    // 盈亏比
    pub profit_loss_ratio: f64,
    // 胜率
    pub profit_ratio: f64,

    // 平均持仓时间
    pub ave_holding_time: i64,
    // 亏损平均持仓时间
    pub ave_loss_holding_time: i64,

    // 盈利平均持仓时间
    pub ave_profit_holding_time: i64,

    // 最大 最小持仓时间
    pub max_holding_time: i64,
    pub min_holding_time: i64,

    // 平均开仓 平仓评分
    pub ave_open_position_score: f64,
    pub ave_close_position_score: f64,

    // 策略统计
    pub strategy_analysis: Vec<BehaviorAnalysis>,

    // 消息来源统计
    pub source_analysis: Vec<BehaviorAnalysis>,
}

impl DashboardAnalysis {
    pub fn new(
        size: usize,
        profit_loss: f64,
        profit: f64,
        ave_profit: f64,
        loss: f64,
        ave_loss: f64,
        profit_count: i32,
        loss_count: i32,
        profit_loss_ratio: f64,
        profit_ratio: f64,
        ave_holding_time: i64,
        ave_loss_holding_time: i64,
        ave_profit_holding_time: i64,
        max_holding_time: i64,
        min_holding_time: i64,
        ave_open_position_score: f64,
        ave_close_position_score: f64,
        strategy_analysis: Vec<BehaviorAnalysis>,
        source_analysis: Vec<BehaviorAnalysis>,
    ) -> Self {
        Self {
            size,
            profit_loss,
            profit,
            ave_profit,
            loss,
            ave_loss,
            profit_count,
            loss_count,
            profit_loss_ratio,
            profit_ratio,
            ave_holding_time,
            ave_loss_holding_time,
            ave_profit_holding_time,
            max_holding_time,
            min_holding_time,
            ave_open_position_score,
            ave_close_position_score,
            strategy_analysis,
            source_analysis,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub initial_balance: f64,

    pub monthly_risk: f64,

    pub single_risk: f64,
}

impl Account {
    pub fn new(initial_balance: f64, monthly_risk: f64, single_risk: f64) -> Self {
        Account {
            initial_balance,
            monthly_risk,
            single_risk,
        }
    }
}

pub fn dash_board_analysis(
    nodes: &Vec<TradeNode>,
    source_map: HashMap<i32, String>,
    trade_strategy_map: HashMap<i32, String>,
) -> DashboardAnalysis {
    // 单数
    let mut size = 0;
    // 总盈亏
    let mut profit_loss = 0.0;
    // 总盈利
    let mut profit = 0.0;
    // 盈利次数
    let mut profit_count = 0;

    // 盈利单持仓时间
    let mut profit_holding_time = 0;
    //  盈利单持仓时间数
    let mut profit_holding_time_count = 0;

    // 总亏损
    let mut loss = 0.0;
    // 亏损次数
    let mut loss_count = 0;

    // 亏损单持仓时间
    let mut loss_holding_time = 0;
    // 亏损单持仓时间数
    let mut loss_holding_time_count = 0;

    // 总持仓时间
    let mut all_holding_time = 0;
    let mut all_holding_count: i64 = 0;

    // 最大持仓时间
    let mut max_holding_time = i64::MIN;
    // 最小持仓时间
    let mut min_holding_time = i64::MAX;

    // 开仓评分
    let mut open_score = 0.0;
    // 开仓次数
    let mut open_score_count = 0;

    // 平仓评分
    let mut close_score = 0.0;
    // 平仓次数
    let mut close_score_count = 0;

    // 标准评分线
    let score_line = 0.3;

    // 策略统计
    let mut strategy_analysis_map: HashMap<i32, BehaviorAnalysis> = HashMap::new();

    // 消息来源统计
    let mut source_analysis_map: HashMap<i32, BehaviorAnalysis> = HashMap::new();

    nodes.iter().for_each(|node| {
        if let Some(holding_time) = node.holding_time {
            all_holding_time += holding_time;
            all_holding_count += 1;

            if holding_time > max_holding_time {
                max_holding_time = holding_time;
            }
            if holding_time < min_holding_time {
                min_holding_time = holding_time;
            }
        }

        // 这单的开仓分
        let mut this_open_score = 0.0;
        let mut this_open_score_count = 0;

        if let Some(op) = &node.open_position {
            op.iter().for_each(|op| {
                // 统计开仓评分
                if let Some(score) = op.score {
                    if score != 0.0 {
                        this_open_score += score;
                        this_open_score_count += 1;
                    }
                }
            })
        }

        // 平均开仓分
        let ave_open_score = this_open_score / this_open_score_count as f64;

        // 加入总开仓评分
        open_score += ave_open_score;
        open_score_count += 1;

        let mut this_profit = 0.0;

        // 这单的平均平仓分
        let mut this_close_score = 0.0;
        let mut this_close_score_count = 0;

        if let Some(cp) = &node.close_position {
            cp.iter().for_each(|cp| {
                // 统计盈利
                if let Some(profit) = cp.profit {
                    this_profit += profit;
                }

                // 统计平仓评分
                if let Some(score) = cp.score {
                    if score != 0.0 {
                        this_close_score += score;
                        this_close_score_count += 1;
                    }
                }
                size += 1;
            })
        }

        // 平均平仓分
        let ave_close_score = this_close_score / this_close_score_count as f64;

        // 加入总平仓评分
        close_score += ave_close_score;
        close_score_count += 1;

        // 统计这单的盈亏
        if this_profit > 0.0 {
            profit += this_profit;
            profit_count += 1;

            // 统计盈利单持仓时间
            if let Some(holding_time) = node.holding_time {
                profit_holding_time += holding_time;
                profit_holding_time_count += 1;
            }
        } else {
            loss += this_profit;
            loss_count += 1;

            // 统计亏损单持仓时间
            if let Some(holding_time) = node.holding_time {
                loss_holding_time += holding_time;
                loss_holding_time_count += 1;
            }
        }

        behavior_analysis(
            this_profit,
            node,
            &mut source_analysis_map,
            &mut strategy_analysis_map,
        )
    });

    // 设置行为使用率 胜率
    calculate_ratio(&mut strategy_analysis_map, size);
    calculate_ratio(&mut source_analysis_map, size);

    // 设置行为名称
    set_behavior_name(&mut strategy_analysis_map, &trade_strategy_map);
    set_behavior_name(&mut source_analysis_map, &source_map);

    let profit_ratio = profit_count as f64 / size as f64;
    // 平均平仓评分
    let mut ave_close_position_score = close_score / close_score_count as f64;
    // 平均开仓评分
    let mut ave_open_position_score = open_score / open_score_count as f64;
    // 亏损单平均持仓
    let mut ave_loss_holding_time = loss_holding_time as f64 / loss_holding_time_count as f64;
    // 盈利单平均持仓
    let mut ave_profit_holding_time = profit_holding_time as f64 / profit_holding_time_count as f64;
    // 平均持仓时间
    let ave_holding_time = all_holding_time as f64 / all_holding_count as f64;

    let strategy_analysis = strategy_analysis_map.into_values().collect();
    let source_analysis = source_analysis_map.into_values().collect();

    let profit_loss_ratio = profit / loss.abs();

    // 平均盈利
    let ave_profit = profit / profit_count as f64;
    // 平均亏损
    let ave_loss = loss / loss_count as f64;

    let analysis = DashboardAnalysis::new(
        size,
        profit + loss,
        profit,
        ave_profit,
        loss,
        ave_loss,
        profit_count,
        loss_count,
        profit_loss_ratio,
        profit_ratio,
        ave_holding_time as i64,
        ave_loss_holding_time as i64,
        ave_profit_holding_time as i64,
        max_holding_time,
        min_holding_time,
        ave_open_position_score,
        ave_close_position_score,
        strategy_analysis,
        source_analysis,
    );

    println!("{:#?}", analysis);
    analysis
}

///
/// 计算行为使用率和胜率
fn calculate_ratio(map: &mut HashMap<i32, BehaviorAnalysis>, size: usize) {
    map.values_mut().for_each(|v| {
        v.use_ratio = v.use_count as f64 / size as f64;
        v.profit_ratio = v.profit_count as f64 / v.use_count as f64;
        if v.loss != 0.0 {
            v.profit_loss_ratio = v.profit / v.loss.abs();
        } else {
            v.profit_loss_ratio = v.profit.abs()
        }
    });
}

fn set_behavior_name(map: &mut HashMap<i32, BehaviorAnalysis>, dict_map: &HashMap<i32, String>) {
    map.values_mut().for_each(|v| {
        if let Some(name) = dict_map.get(&v.value) {
            v.name = name.clone();
        }
    });
}

///
/// 行为分析
fn behavior_analysis(
    this_profit: f64,
    node: &TradeNode,
    source_analysis_map: &mut HashMap<i32, BehaviorAnalysis>,
    strategy_analysis_map: &mut HashMap<i32, BehaviorAnalysis>,
) {
    // 消息来源分析
    if let Some(source) = &node.source {
        let behavior = source_analysis_map
            .entry(*source)
            .or_insert_with(|| BehaviorAnalysis::new(*source));

        behavior.use_count += 1;

        if this_profit > 0.0 {
            behavior.profit_count += 1;
            behavior.profit += this_profit;
        } else {
            behavior.loss_count += 1;
            behavior.loss += this_profit;
        }
    }

    // 策略分析
    if let Some(strategy) = &node.strategy {
        let behavior = strategy_analysis_map
            .entry(*strategy)
            .or_insert_with(|| BehaviorAnalysis::new(*strategy));
        // 使用率+1
        behavior.use_count += 1;

        if this_profit > 0.0 {
            behavior.profit_count += 1;
            behavior.profit += this_profit;
        } else {
            behavior.loss_count += 1;
            behavior.loss += this_profit;
        }
    }
}
