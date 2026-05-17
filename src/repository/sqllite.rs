use super::TodoRepository;
use crate::{Task, TodoError};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

#[derive(Clone)]
pub struct SqliteRepo {
    pool: Pool<SqliteConnectionManager>,
}
impl SqliteRepo {
    pub fn new(db_path: &str) -> Result<Self, TodoError> {
        // 1. Create the connection manager. It automatically creates the DB file if missing.
        let manager = SqliteConnectionManager::file(db_path);

        // 2. Initialize the pool with custom configuration (e.g., max 10 concurrent connections)
        let pool = r2d2::Pool::builder()
            .max_size(10)
            .build(manager)
            .map_err(|e| TodoError::DatabaseError(format!("Failed to create pool: {e}")))?;

        // 3. Grab a temporary connection from the pool to run setup migrations
        let conn = pool.get().map_err(|e| {
            TodoError::DatabaseError(format!("Failed to get connection from pool: {e}"))
        })?;

        // 4. Optimize SQLite performance settings for the connection
        conn.execute_batch(
            "
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA foreign_keys = ON;
        ",
        )
        .map_err(|e| TodoError::DatabaseError(e.to_string()))?;

        // 5. Create the table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT 0
            )",
            [],
        )
        .map_err(|e| TodoError::DatabaseError(e.to_string()))?;

        // 6. Return the repo containing the thread-safe pool
        // (The temporary connection `conn` is safely dropped and returned to the pool here)
        Ok(Self { pool })
    }
}

impl TodoRepository for SqliteRepo {
    fn add_task(&mut self, title: String, description: String) -> Result<Task, TodoError> {
        if title.trim().is_empty() {
            return Err(TodoError::EmptyTitle);
        }
        if description.trim().is_empty() {
            return Err(TodoError::EmptyDescription);
        }
        let conn = self
            .pool
            .get()
            .map_err(|e| TodoError::DatabaseError(format!("Pool error:{e}")))?;
        conn.execute(
            "INSERT INTO tasks (title,description,completed)VALUES(?1,?2,?3)",
            rusqlite::params![title, description, false],
        )
        .map_err(|e| TodoError::DatabaseError(e.to_string()))?;
        let id = conn.last_insert_rowid() as usize;
        Ok(Task {
            id,
            title,
            description,
            completed: false,
        })
    }

    fn list_tasks(&self) -> Result<Vec<Task>, TodoError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| TodoError::DatabaseError(format!("Pool err: {e}")))?;
        let mut stmt = conn
            .prepare("SELECT id, title, description, completed FROM tasks ORDER BY id")
            .map_err(|e| TodoError::DatabaseError(e.to_string()))?;

        // This returns rusqlite::Rows
        let mut rows = stmt
            .query([])
            .map_err(|e| TodoError::DatabaseError(e.to_string()))?;

        let mut result = Vec::new();

        // Fix: Use while let Some(row) = rows.next()? instead of a for loop
        while let Some(row) = rows
            .next()
            .map_err(|e| TodoError::DatabaseError(e.to_string()))?
        {
            let task = Task {
                id: row
                    .get(0)
                    .map_err(|e| TodoError::DatabaseError(e.to_string()))?,
                title: row
                    .get(1)
                    .map_err(|e| TodoError::DatabaseError(e.to_string()))?,
                description: row
                    .get(2)
                    .map_err(|e| TodoError::DatabaseError(e.to_string()))?,
                completed: row
                    .get(3)
                    .map_err(|e| TodoError::DatabaseError(e.to_string()))?,
            };
            result.push(task);
        }

        Ok(result)
    }

    fn get_task(&self, id: usize) -> Result<Option<Task>, TodoError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| TodoError::DatabaseError(format!("Pool err: {e}")))?;
        let mut stmt = conn
            .prepare("SELECT id, title, description,complete FROM tasks WHERE id=?1")
            .map_err(|e| TodoError::DatabaseError(e.to_string()))?;
        let mut rows = stmt
            .query([id])
            .map_err(|e| TodoError::DatabaseError(e.to_string()))?;
        if let Some(row) = rows
            .next()
            .map_err(|e| TodoError::DatabaseError(e.to_string()))?
        {
            Ok(Some(Task {
                id: row.get(0).map_err(|e| TodoError::DatabaseError(e.to_string()))?,
                title: row.get(1).map_err(|e| TodoError::DatabaseError(e.to_string()))?,
                description: row.get(2).map_err(|e| TodoError::DatabaseError(e.to_string()))?,
                completed: row.get(3).map_err(|e| TodoError::DatabaseError(e.to_string()))?,
            }))
        } else {
            Err(TodoError::InvalidIndex(id))
        }
    }
    fn remove_task(&mut self, id: usize) -> Result<Option<Task>, TodoError> {
        let task = self.get_task(id)?;

        let conn = self
            .pool
            .get()
            .map_err(|e| TodoError::DatabaseError(format!("Pool err: {e}")))?;

        if task.is_some() {
            conn.execute("DELETE FROM tasks where id=?1", [id])
                .map_err(|e| TodoError::DatabaseError(e.to_string()))?;
        }
        Ok(task)
    }

    fn complete_task(&mut self, id: usize) -> Result<bool, TodoError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| TodoError::DatabaseError(format!("Pool err: {e}")))?;
        let affected = conn
            .execute(
                "UPDATE tasks SET completed=1 WHERE id =?1 AND completed=0",
                [id],
            )
            .map_err(|e| TodoError::DatabaseError(e.to_string()))?;

        // task = self.get_task(id);
        Ok(affected > 0)
        //     Ok(task)
        // }
    }

    fn task_count(&self) -> Result<usize, TodoError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| TodoError::DatabaseError(format!("Pool err: {e}")))?;
        let count: usize = conn
            .query_row("SELECT COUNT (*) FROM tasks", [], |row| row.get(0))
            .map_err(|e| TodoError::DatabaseError(e.to_string()))?;
        Ok(count)
    }
    fn clear_all(&mut self) -> Result<(), TodoError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| TodoError::DatabaseError(format!("Pool err: {e}")))?;
        conn.execute("DELETE FROM tasks", [])
            .map_err(|e| TodoError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
