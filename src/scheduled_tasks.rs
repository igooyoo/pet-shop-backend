// // Scheduler, and trait for .seconds(), .minutes(), etc.
// use clokwerk::{Scheduler, TimeUnits};
// // Import week days and WeekDay
// use db_queries::{DbPool};
// // use db_schema::models::audit_logs::AuditLog;
// use diesel::{sql_query, PgConnection, RunQueryDsl};
// use log::info;
// use std::{thread, time::Duration};

// /// Schedules various cleanup tasks for auth in a background thread
// pub fn setup(pool: DbPool) {
//     let mut scheduler = Scheduler::new();

//     let conn = pool.get().unwrap();

//     reindex_aggregates_tables(&conn);
//     scheduler.every(1.hour()).run(move || {
//         reindex_aggregates_tables(&conn);
//     });

//     let conn = pool.get().unwrap();
//     let days: i32 = 7;
//     purge_audit_logs(&conn, days);
//     scheduler.every(1.weeks()).run(move || {
//         purge_audit_logs(&conn, days);
//     });

//     // Manually run the scheduler in an event loop
//     loop {
//         scheduler.run_pending();
//         thread::sleep(Duration::from_millis(1000));
//     }
// }

// /// Reindex the aggregates tables every one hour
// fn reindex_aggregates_tables(conn: &PgConnection) {
//     for table_name in &["instance_aggregates"] {
//         reindex_table(&conn, &table_name);
//     }
// }

// fn reindex_table(conn: &PgConnection, table_name: &str) {
//     info!("Reindexing table {} ...", table_name);
//     let query = format!("reindex table concurrently {}", table_name);
//     sql_query(query).execute(conn).expect("reindex table");
//     info!("Done.");
// }

// /// Clear old activities (this table gets very large)
// fn purge_audit_logs(_conn: &PgConnection, _days: i32) {
//     info!("Clearing old audit logs...");
//     // AuditLog::purge(&conn, days).expect("clear old audit logs");
//     info!("Done.");
// }
