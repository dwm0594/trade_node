use std::collections::HashMap;
use crate::enums::{Currency, Direction, PositionType};
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests {
    use crate::db::repository::{
        find_positions_by_node_id, find_trade_nodes_by_time, init_db, insert_dictionary,
        insert_position_sql, insert_trade_node_sql, Dictionary, Position, TradeNode,
    };
    use crate::enums::{Currency, Direction, PositionType};
    use rusqlite::Connection;
    use serde::__private229::ser::constrain;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_sql() -> Result<(), Box<dyn std::error::Error>> {
        init_db().expect("error");

        Ok(())
    }

    #[test]
    fn test_insert() -> Result<(), Box<dyn std::error::Error>> {
        init_db().expect("error");

        Ok(())
    }

    #[test]
    fn test_insert_direction() -> Result<(), Box<dyn std::error::Error>> {
        let connection = init_db().expect("error");

        let d = Dictionary::new("策略", "EFI 底背离", 1);

        let result = insert_dictionary(&connection, &d);

        Ok(())
    }

    #[test]
    fn test_insert_trade_node() -> Result<(), Box<dyn std::error::Error>> {
        let connection = init_db().expect("error");

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let node = TradeNode::new(
            timestamp,
            "茅台",
            Direction::Short,
            23.4,
            1,
            3,
            25.0,
            22.0,
            2.32,
            12031203,
            Currency::RMB,
            "a,b",
            "a,b",
        );

        for i in 0..10 {
            insert_trade_node_sql(&connection, &node).expect("error");
        }

        Ok(())
    }

    #[test]
    fn test_insert_open_position() -> Result<(), Box<dyn std::error::Error>> {
        let connection = init_db().expect("error");

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let node = Position::new(
            1,
            PositionType::OPEN,
            timestamp,
            23.4,
            100.0,
            2340.0,
            24.0,
            23.0,
            2,
            "sada",
            123.3,
        );

        insert_position_sql(&connection, &node).expect("error");
        Ok(())
    }
    #[test]
    fn test_insert_close_position() -> Result<(), Box<dyn std::error::Error>> {
        let connection = init_db().expect("error");

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let node = Position::new(
            1,
            PositionType::CLOSE,
            timestamp,
            25.4,
            200.0,
            2340.0,
            26.0,
            23.0,
            2,
            "sada",
            1323.3,
        );

        insert_position_sql(&connection, &node).expect("error");
        Ok(())
    }

    #[test]
    fn test_find_trade() -> Result<(), Box<dyn std::error::Error>> {
        let connection = init_db().expect("error");

        let result = find_trade_nodes_by_time(&connection, 2, 2)?;
        println!("{:?}", result);
        Ok(())
    }

    #[test]
    fn test_find_position() -> Result<(), Box<dyn std::error::Error>> {
        let connection = init_db().expect("error");
        let vec = find_positions_by_node_id(&connection, 1).expect("error");

        println!("{:?}", vec);
        Ok(())
    }
}

pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
}

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("data.db")?;

    conn.execute_batch(
        "
        create table if not exists main.trade_node
(
    id                integer
        constraint trade_node_pk
            primary key,
    date              integer,
    name              text,
    direction         integer,
    expect             real,
    source            integer,
    strategy            integer,
    profit            real,
    stop_loss         real,
    profit_loss_ratio real,
    holding_time      integer,
    currency_type     integer,
    screenshots       text,
    backtracking       text
);


create table if not exists main.position
(
    id        integer
        constraint position_pk
            primary key,
    node_id   integer,
    type      integer,
    date      integer,
    price     real,
    quantity  real,
    total     real,
    top_price real,
    low_price real,
    score     integer,
    reason    text,
    profit    real
);

create table if not exists main.dictionary
(
    id    integer
        constraint dictionary_pk
            primary key,
    type  text,
    name  text,
    value integer
);


        ",
    )?;

    Ok(conn)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeNode {
    pub id: i64,
    pub date: Option<i64>,
    pub name: Option<String>,
    pub direction: Direction,
    pub expect: Option<f64>,
    pub source: Option<i32>,
    pub strategy: Option<i32>,
    pub profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub profit_loss_ratio: Option<f64>,
    pub holding_time: Option<i64>,
    pub currency_type: Currency,
    pub screenshots: Option<String>,
    pub open_position: Option<Vec<Position>>,
    pub close_position: Option<Vec<Position>>,
    pub backtracking: Option<String>,
}

impl TradeNode {
    fn new(
        date: i64,
        name: &str,
        direction: Direction,
        expect: f64,
        source: i32,
        strategy: i32,
        profit: f64,
        stop_loss: f64,
        profit_loss_ratio: f64,
        holding_time: i64,
        currency_type: Currency,
        screenshots: &str,
        backtracking: &str,
    ) -> TradeNode {
        TradeNode {
            id: 0,
            date: Some(date),
            name: Some(name.to_string()),
            direction,
            expect: Some(expect),
            source: Some(source.into()),
            strategy: Some(strategy.into()),
            profit: Some(profit),
            stop_loss: Some(stop_loss),
            profit_loss_ratio: Some(profit_loss_ratio),
            holding_time: Some(holding_time),
            currency_type,
            screenshots: Some(screenshots.to_string()),
            open_position: Some(vec![]),
            close_position: Some(vec![]),
            backtracking: Some(backtracking.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub id: i64,
    pub node_id: Option<i64>,
    // 开 还是 平
    pub position_type: PositionType,
    // 日期
    pub date: Option<i64>,
    // 价格
    pub price: Option<f64>,
    // 数量
    pub quantity: Option<f64>,
    // 总额
    pub total: Option<f64>,
    // 最高价
    pub top_price: Option<f64>,
    // 最低价
    pub low_price: Option<f64>,
    // 评分
    pub score: Option<f64>,
    // 理由
    pub reason: Option<String>,
    // 盈利 （用于平仓）
    pub profit: Option<f64>,
}

impl Position {
    fn new(
        node_id: i64,
        ope_potion: PositionType,
        date: i64,
        price: f64,
        quantity: f64,
        total: f64,
        top_price: f64,
        low_price: f64,
        score: i32,
        reason: &str,
        profit: f64,
    ) -> Position {
        Position {
            id: 0,
            node_id: Some(node_id),
            date: Some(date),
            position_type: ope_potion,
            price: Some(price),
            quantity: Some(quantity),
            total: Some(total),
            top_price: Some(top_price),
            low_price: Some(low_price),
            score: Some(score.into()),
            reason: Some(reason.to_string()),
            profit: Some(profit),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Dictionary {
    pub id: i64,
    pub strategy: Option<String>,
    pub name: Option<String>,
    pub value: Option<i32>,
}
impl Dictionary {
    pub fn new(strategy: &str, name: &str, value: i32) -> Self {
        Dictionary {
            id: 0,
            strategy: Some(strategy.to_string()),
            name: Some(name.to_string()),
            value: Some(value),
        }
    }
}

pub fn insert_trade_node_sql(conn: &Connection, node: &TradeNode) -> Result<i64> {
    let i = conn.execute(
        "
        INSERT INTO trade_node (
            date,
            name,
            direction,
            expect,
            source,
            strategy,
            profit,
            stop_loss,
            profit_loss_ratio,
            holding_time,
            currency_type,
            screenshots
        )
        VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6,
            ?7, ?8, ?9, ?10, ?11, ?12
        )
        ",
        (
            node.date,
            &node.name,
            node.direction.value(),
            node.expect,
            node.source,
            node.strategy,
            node.profit,
            node.stop_loss,
            node.profit_loss_ratio,
            node.holding_time,
            node.currency_type.value(),
            &node.screenshots,
        ),
    )?;

    let id = conn.last_insert_rowid();

    Ok(id)
}

pub fn update_trade_node_sql(conn: &Connection, node: &TradeNode) -> Result<()> {
    conn.execute(
        "
        UPDATE trade_node
        SET
            date = ?1,
            name = ?2,
            direction = ?3,
            expect = ?4,
            source = ?5,
            strategy = ?6,
            profit = ?7,
            stop_loss = ?8,
            profit_loss_ratio = ?9,
            holding_time = ?10,
            currency_type = ?11,
            screenshots = ?12,
            backtracking = ?13
        WHERE id = ?14
        ",
        (
            node.date,
            &node.name,
            node.direction.value(),
            node.expect,
            node.source,
            node.strategy,
            node.profit,
            node.stop_loss,
            node.profit_loss_ratio,
            node.holding_time,
            node.currency_type.value(),
            &node.screenshots,
            &node.backtracking,
            node.id,
        ),
    )?;

    Ok(())
}

pub fn delete_trade_node(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM trade_node WHERE id = ?1", [id])?;

    Ok(())
}

pub fn insert_position_sql(conn: &Connection, position: &Position) -> Result<()> {
    conn.execute(
        "
        INSERT INTO position (
            node_id,
            type,
            date,
            price,
            quantity,
            total,
            top_price,
            low_price,
            score,
            reason,
            profit
        )
        VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6,
            ?7, ?8, ?9, ?10, ?11
        )
        ",
        (
            position.node_id,
            position.position_type.value(),
            position.date,
            position.price,
            position.quantity,
            position.total,
            position.top_price,
            position.low_price,
            position.score,
            &position.reason,
            position.profit,
        ),
    )?;

    Ok(())
}

pub fn batch_insert_position_sql(conn: &Connection, position: &Vec<Position>) -> Result<()> {
    for p in position {
        insert_position_sql(conn, &p)?;
    }

    Ok(())
}

pub fn update_position_sql(conn: &Connection, position: &Position) -> Result<()> {
    conn.execute(
        "
        UPDATE position
        SET
            node_id = ?1,
            type = ?2,
            date = ?3,
            price = ?4,
            quantity = ?5,
            total = ?6,
            top_price = ?7,
            low_price = ?8,
            score = ?9,
            reason = ?10,
            profit = ?11
        WHERE id = ?12
        ",
        (
            position.node_id,
            position.position_type.value(),
            position.date,
            position.price,
            position.quantity,
            position.total,
            position.top_price,
            position.low_price,
            position.score,
            &position.reason,
            position.profit,
            position.id,
        ),
    )?;

    Ok(())
}

pub fn delete_position(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM position WHERE id = ?1", [id])?;

    Ok(())
}

pub fn insert_dictionary(conn: &Connection, dictionary: &Dictionary) -> Result<()> {
    conn.execute(
        "
        INSERT INTO dictionary (
            type,
            name,
            value
        )
        VALUES (?1, ?2, ?3)
        ",
        (&dictionary.strategy, &dictionary.name, dictionary.value),
    )?;

    Ok(())
}

pub const MESSAGE_SOURCE: &str = "message_source";
pub const TRADE_STRATEGY: &str = "trade_strategy";


pub fn find_dictionaries_by_type(
    conn: &Connection,
    dictionary_type: &str,
) -> Result<Vec<Dictionary>> {
    let mut stmt = conn.prepare(
        "
        SELECT id, type, name, value
        FROM dictionary
        WHERE type = ?1
        ORDER BY id
        ",
    )?;

    let result = stmt
        .query_map([dictionary_type], |row| {
            Ok(Dictionary {
                id: row.get("id")?,
                strategy: row.get("type")?,
                name: row.get("name")?,
                value: row.get("value")?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;

    Ok(result)
}

pub fn update_dictionary(conn: &Connection, dictionary: &Dictionary) -> Result<()> {
    conn.execute(
        "
        UPDATE dictionary
        SET
            type = ?1,
            name = ?2,
            value = ?3
        WHERE id = ?4
        ",
        (
            &dictionary.strategy,
            &dictionary.name,
            dictionary.value,
            dictionary.id,
        ),
    )?;

    Ok(())
}

pub fn delete_dictionary(conn: &Connection, id: i64) -> Result<usize> {
    conn.execute("DELETE FROM dictionary WHERE id = ?1", [id])
}

pub fn find_positions_by_node_id(conn: &Connection, node_id: i64) -> Result<Vec<Position>> {
    let mut stmt = conn.prepare(
        "
        SELECT
            id,
            node_id,
            type,
            date,
            price,
            quantity,
            total,
            top_price,
            low_price,
            score,
            reason,
            profit
        FROM position
        WHERE node_id = ?1
        ORDER BY date
        ",
    )?;

    let positions = stmt
        .query_map([node_id], |row| {
            let ope_position: i32 = row.get("type")?;

            Ok(Position {
                id: row.get("id")?,
                node_id: row.get("node_id")?,
                position_type: ope_position.into(),
                date: row.get("date")?,
                price: row.get("price")?,
                quantity: row.get("quantity")?,
                total: row.get("total")?,
                top_price: row.get("top_price")?,
                low_price: row.get("low_price")?,
                score: row.get("score")?,
                reason: row.get("reason")?,
                profit: row.get("profit")?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;

    Ok(positions)
}

pub fn find_positions_by_id(conn: &Connection, id: i64) -> Result<Position> {
    let mut stmt = conn.prepare(
        "
        SELECT
            id,
            node_id,
            type,
            date,
            price,
            quantity,
            total,
            top_price,
            low_price,
            score,
            reason,
            profit
        FROM position
        WHERE id = ?1
        ORDER BY date
        ",
    )?;

    let positions = stmt.query_one([id], |row| {
        let ope_position: i32 = row.get("type")?;

        Ok(Position {
            id: row.get("id")?,
            node_id: row.get("node_id")?,
            position_type: ope_position.into(),
            date: row.get("date")?,
            price: row.get("price")?,
            quantity: row.get("quantity")?,
            total: row.get("total")?,
            top_price: row.get("top_price")?,
            low_price: row.get("low_price")?,
            score: row.get("score")?,
            reason: row.get("reason")?,
            profit: row.get("profit")?,
        })
    })?;

    Ok(positions)
}

fn find_positions_by_node(conn: &Connection, id: i64) -> Result<(Vec<Position>, Vec<Position>)> {
    let positions: Vec<Position> = find_positions_by_node_id(&conn, id)?;

    let (open_position, close_position) = positions
        .into_iter()
        .partition(|p| p.position_type == PositionType::OPEN);

    Ok((open_position, close_position))
}

pub fn find_trade_nodes_by_time(conn: &Connection, start: i64, end: i64) -> Result<Vec<TradeNode>> {
    let mut stmt = conn.prepare(
        "
        SELECT
            id,
            date,
            name,
            direction,
            expect,
            source,
            strategy,
            profit,
            stop_loss,
            profit_loss_ratio,
            holding_time,
            currency_type,
            screenshots,
            backtracking
        FROM trade_node
        WHERE date >= ?1
          AND date < ?2
        ORDER BY date DESC
        ",
    )?;

    let result = stmt
        .query_map((start, end), |row| {
            let id: i64 = row.get("id")?;
            let direction: i32 = row.get("direction")?;
            let currency_type: i32 = row.get("currency_type")?;

            let (open_position, close_position) = find_positions_by_node(&conn, id)?;

            Ok(TradeNode {
                id: row.get("id")?,
                date: row.get("date")?,
                name: row.get("name")?,
                direction: direction.into(),
                expect: row.get("expect")?,
                source: row.get("source")?,
                strategy: row.get("strategy")?,
                profit: row.get("profit")?,
                stop_loss: row.get("stop_loss")?,
                profit_loss_ratio: row.get("profit_loss_ratio")?,
                holding_time: row.get("holding_time")?,
                currency_type: currency_type.into(),
                screenshots: row.get("screenshots")?,
                open_position: Some(open_position),
                close_position: Some(close_position),
                backtracking: row.get("backtracking")?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(result)
}

pub fn find_trade_nodes_by_id(conn: &Connection, id: i64) -> Result<TradeNode> {
    let mut stmt = conn.prepare(
        "
        SELECT
            id,
            date,
            name,
            direction,
            expect,
            source,
            strategy,
            profit,
            stop_loss,
            profit_loss_ratio,
            holding_time,
            currency_type,
            screenshots,
            backtracking
        FROM trade_node
        WHERE id == ?1
        ",
    )?;

    let result = stmt.query_one((id,), |row| {
        let id: i64 = row.get("id")?;
        let direction: i32 = row.get("direction")?;
        let currency_type: i32 = row.get("currency_type")?;

        let (open_position, close_position) = find_positions_by_node(&conn, id)?;

        Ok(TradeNode {
            id: row.get("id")?,
            date: row.get("date")?,
            name: row.get("name")?,
            direction: direction.into(),
            expect: row.get("expect")?,
            source: row.get("source")?,
            strategy: row.get("strategy")?,
            profit: row.get("profit")?,
            stop_loss: row.get("stop_loss")?,
            profit_loss_ratio: row.get("profit_loss_ratio")?,
            holding_time: row.get("holding_time")?,
            currency_type: currency_type.into(),
            screenshots: row.get("screenshots")?,
            open_position: Some(open_position),
            close_position: Some(close_position),
            backtracking: row.get("backtracking")?,
        })
    });

    result
}

pub fn find_trade_nodes_by_range_and_currency(
    conn: &Connection,
    start: i64,
    end: i64,
    currency: Currency,
) -> Result<Vec<TradeNode>> {
    let mut stmt = conn.prepare(
        "
        SELECT
            id,
            date,
            name,
            direction,
            expect,
            source,
            strategy,
            profit,
            stop_loss,
            profit_loss_ratio,
            holding_time,
            currency_type,
            screenshots,
            backtracking
        FROM trade_node
         WHERE date >= ?1
          AND date < ?2
          AND currency_type = ?3
        ",
    )?;

    let result = stmt
        .query_map((start, end, currency.value()), |row| {
            let id: i64 = row.get("id")?;
            let direction: i32 = row.get("direction")?;
            let currency_type: i32 = row.get("currency_type")?;

            let (open_position, close_position) = find_positions_by_node(&conn, id)?;

            Ok(TradeNode {
                id: row.get("id")?,
                date: row.get("date")?,
                name: row.get("name")?,
                direction: direction.into(),
                expect: row.get("expect")?,
                source: row.get("source")?,
                strategy: row.get("strategy")?,
                profit: row.get("profit")?,
                stop_loss: row.get("stop_loss")?,
                profit_loss_ratio: row.get("profit_loss_ratio")?,
                holding_time: row.get("holding_time")?,
                currency_type: currency_type.into(),
                screenshots: row.get("screenshots")?,
                open_position: Some(open_position),
                close_position: Some(close_position),
                backtracking: row.get("backtracking")?,
            })
        })?
        .collect::<Result<Vec<TradeNode>>>();

    result
}
