use crate::db::repository::{
    batch_insert_position_sql, delete_dictionary, find_dictionaries_by_type,
    find_dictionary_by_type, find_positions_by_id, find_trade_nodes_by_id,
    find_trade_nodes_by_range_and_currency, find_trade_nodes_by_time, insert_dictionary,
    insert_position_sql, insert_trade_node_sql, update_dictionary, update_position_sql,
    update_trade_node_sql, AppState, Dictionary, Position, TradeNode, MESSAGE_SOURCE,
    TRADE_STRATEGY,
};
use crate::enums::Currency;
use crate::util::{
    copy_screenshots, dash_board_analysis, delete_old_picture, Account, DashboardAnalysis,
};
use rusqlite::{Connection, Error};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{MutexGuard, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::State;

#[derive(Debug, Serialize)]
pub struct CommandError(pub String);

impl From<Error> for CommandError {
    fn from(error: Error) -> Self {
        CommandError(error.to_string())
    }
}

impl From<String> for CommandError {
    fn from(error: String) -> Self {
        CommandError(error)
    }
}
impl From<PoisonError<MutexGuard<'_, Connection>>> for CommandError {
    fn from(value: PoisonError<MutexGuard<Connection>>) -> Self {
        CommandError(value.to_string())
    }
}

impl From<serde_json::Error> for CommandError {
    fn from(error: serde_json::Error) -> Self {
        CommandError(error.to_string())
    }
}

#[tauri::command]
pub fn find_trade_node(day: i32, state: State<AppState>) -> Result<Vec<TradeNode>, CommandError> {
    let conn = state.db.lock()?;

    let now: i64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    let start = now - (day as i64) * 24 * 60 * 60 * 1000;

    Ok(find_trade_nodes_by_time(&conn, start, now)?)
}

#[tauri::command]
pub fn find_trade_node_by_range(
    start: i64,
    end: i64,
    state: State<AppState>,
) -> Result<Vec<TradeNode>, CommandError> {
    let conn = state.db.lock()?;

    Ok(find_trade_nodes_by_time(&conn, start, end)?)
}

#[tauri::command]
pub fn find_trade_node_by_id(id: i64, state: State<AppState>) -> Result<TradeNode, CommandError> {
    let conn = state.db.lock()?;

    Ok(find_trade_nodes_by_id(&conn, id)?)
}

#[tauri::command]
pub fn insert_dict(
    strategy: &str,
    name: &str,
    state: State<AppState>,
) -> Result<Dictionary, CommandError> {
    let conn = state.db.lock()?;

    let result = find_dictionaries_by_type(&conn, strategy)?;

    let names: Vec<String> = result.iter().filter_map(|x| x.name.clone()).collect();

    if names.contains(&name.to_string()) {
        return Err(CommandError("名称重复".to_string()));
    }

    let mut value = 0;
    if !result.is_empty() {
        let max_value = result
            .iter()
            .map(|x| x.value)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            .unwrap();

        value = max_value + 1;
    }

    let d = Dictionary::new(strategy, name, value);

    insert_dictionary(&conn, &d)?;

    Ok(d)
}

#[tauri::command]
pub fn insert_or_update_attribute(
    dict_type: &str,
    name: &str,
    value: i32,
    state: State<AppState>,
) -> Result<(), CommandError> {
    let conn = state.db.lock()?;

    let result = find_dictionary_by_type(&conn, dict_type);
    match result {
        Ok(mut dictionary) => {
            dictionary.name = Some(name.to_string());
            dictionary.value = Some(value);
            update_dictionary(&conn, &dictionary)?;
        }

        Err(_) => {
            let dictionary = Dictionary::new(dict_type, name, value);
            insert_dictionary(&conn, &dictionary)?;
        }
    }

    Ok(())
}

#[tauri::command]
pub fn find_attribute(dict_type: &str, state: State<AppState>) -> Result<Dictionary, CommandError> {
    let conn = state.db.lock()?;

    let result = find_dictionary_by_type(&conn, dict_type)?;

    Ok(result)
}

#[tauri::command]
pub fn find_strategy(
    strategy: &str,
    state: State<AppState>,
) -> Result<Vec<Dictionary>, CommandError> {
    let conn = state.db.lock()?;

    let result = find_dictionaries_by_type(&conn, strategy)?;

    Ok(result)
}

#[tauri::command]
pub fn delete_strategy(id: i64, state: State<AppState>) -> Result<usize, CommandError> {
    let conn = state.db.lock()?;

    let result = delete_dictionary(&conn, id)?;

    Ok(result)
}

#[tauri::command]
pub fn insert_trade_node(mut node: TradeNode, state: State<AppState>) -> Result<(), CommandError> {
    let conn = state.db.lock()?;

    if let Some(path) = copy_screenshots(node.screenshots.as_ref())? {
        node.screenshots = Some(path);
    }

    let id = insert_trade_node_sql(&conn, &node)?;

    fn save_positions(
        conn: &Connection,
        positions: Option<Vec<Position>>,
        id: i64,
    ) -> Result<(), CommandError> {
        if let Some(mut positions) = positions {
            for x in positions.iter_mut() {
                x.node_id = Some(id);
            }
            batch_insert_position_sql(&conn, &positions)?;
        }

        Ok(())
    }

    save_positions(&conn, node.open_position, id)?;
    save_positions(&conn, node.close_position, id)?;

    Ok(())
}

#[tauri::command]
pub fn update_trade_node(mut node: TradeNode, state: State<AppState>) -> Result<(), CommandError> {
    let conn = state.db.lock()?;

    if let Some(path) = copy_screenshots(node.screenshots.as_ref())? {
        node.screenshots = Some(path);
    }

    let old_trade_node = find_trade_nodes_by_id(&conn, node.id)?;

    delete_old_picture(&old_trade_node)?;

    update_trade_node_sql(&conn, &node)?;

    fn save_positions(
        conn: &Connection,
        positions: Option<Vec<Position>>,
        node_id: i64,
    ) -> Result<(), CommandError> {
        if let Some(mut positions) = positions {
            for x in positions.iter_mut() {
                if x.id == 0 {
                    x.node_id = Some(node_id);
                    insert_position_sql(conn, x)?;
                } else {
                    update_position_sql(conn, x)?;
                }
            }
        }

        Ok(())
    }

    save_positions(&conn, node.open_position, node.id)?;
    save_positions(&conn, node.close_position, node.id)?;

    Ok(())
}

#[tauri::command]
pub fn get_exe_dir() -> Result<String, String> {
    std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .ok_or_else(|| "无法获取 exe 目录".to_string())
}

#[tauri::command]
pub fn trade_analysis(
    start: i64,
    end: i64,
    currency: Currency,
    state: State<AppState>,
) -> Result<DashboardAnalysis, CommandError> {
    let conn = state.db.lock()?;

    let message_source = find_dictionaries_by_type(&conn, MESSAGE_SOURCE)?;
    let trade_strategy = find_dictionaries_by_type(&conn, TRADE_STRATEGY)?;

    let source_map: HashMap<i32, String> = message_source
        .into_iter()
        .filter_map(|x| Some((x.value?, x.name?)))
        .collect();

    let trade_strategy_map: HashMap<i32, String> = trade_strategy
        .into_iter()
        .filter_map(|x| Some((x.value?, x.name?)))
        .collect();

    println!("{} {} {}", start, end, currency.value());
    let result = find_trade_nodes_by_range_and_currency(&conn, start, end, currency)?;

    if result.is_empty() {
        return Ok(DashboardAnalysis::default());
    }

    let analysis = dash_board_analysis(&result, source_map, trade_strategy_map);

    Ok(analysis)
}

#[tauri::command]
pub fn account_setting(
    dict_type: &str,
    account: Account,
    state: State<AppState>,
) -> Result<(), CommandError> {
    insert_or_update_attribute(
        dict_type,
        serde_json::to_string(&account)?.as_ref(),
        0,
        state,
    )?;

    Ok(())
}

#[tauri::command]
pub fn find_account_setting(
    dict_type: &str,
    state: State<AppState>,
) -> Result<Dictionary, CommandError> {
    let conn = state.db.lock()?;

    let setting = find_dictionary_by_type(&conn, dict_type)?;

    Ok(setting)
}
