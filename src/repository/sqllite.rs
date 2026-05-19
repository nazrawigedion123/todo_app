use super::TodoRepository;
use crate::{Task, TodoError};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

#[derive(Clone)]
pub struct SqliteRepo {
    pool: Pool<SqliteConnectionManager>,
}
impl SqliteRepo {
    pub fn new_in_memory(random_string: String) -> Result<Self, TodoError> {
        // "file::memory:?cache=shared" keeps the memory DB alive across connections
        Self::new(&format!("file:{random_string}:memory:?cache=shared"))
    }
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
            .prepare("SELECT id, title, description,completed FROM tasks WHERE id=?1")
            .map_err(|e| TodoError::DatabaseError(e.to_string()))?;
        let mut rows = stmt
            .query([id])
            .map_err(|e| TodoError::DatabaseError(e.to_string()))?;
        if let Some(row) = rows
            .next()
            .map_err(|e| TodoError::DatabaseError(e.to_string()))?
        {
            Ok(Some(Task {
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
        let task = self.get_task(id);

        let affected = conn
            .execute(
                "UPDATE tasks SET completed=1 WHERE id =?1 AND completed=0",
                [id],
            )
            .map_err(|e| TodoError::DatabaseError(e.to_string()))?;

        if affected == 0 {
            // Check if the task even exists in the database
            let task = self.get_task(id);
            match task {
                Ok(_) => return Ok(false),
                Err(_) => return Err(TodoError::InvalidIndex(id)),
            }

            // If it exists but affected was 0, it means it was already completed.
            // Return false indicating no state change occurred.
            // return Ok(false);
        }

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
        let tasks = self.list_tasks();
        match tasks {
            Ok(ts) => ({
                if ts.is_empty(){
                  return   Err(TodoError::AlreadyEmpty);
                }
            }),
            Err(e) => println!("error :{e}"),
        }
      
        conn.execute("DELETE FROM tasks", [])
            .map_err(|e| TodoError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup_test_repo() -> SqliteRepo {
        let random_id = uuid::Uuid::new_v4().to_string();
        // let url =format!("file:{random_id}?mode=memory&cache=shared");
        SqliteRepo::new_in_memory(random_id).unwrap()
    }

    fn test_add_task_validation() {
        let mut repo = setup_test_repo();
        let err = repo
            .add_task("".to_string(), "description".to_string())
            .unwrap_err();
        assert!(matches!(err, TodoError::EmptyTitle));
    }

    #[test]
    fn test_add_task_validation_description() {
        let mut repo = setup_test_repo();
        let err = repo
            .add_task("title".to_string(), "".to_string())
            .unwrap_err();
        assert!(matches!(err, TodoError::EmptyDescription));
    }
    #[test]
    fn test_add_task() {
        let title = "title".to_string();
        let description = "desc".to_string();
        let mut repo = setup_test_repo();
        let task = repo.add_task(title, description).unwrap();
        assert!(matches!(
            task,
            Task {
                id: 1,
                title,
                description,
                completed: false
            }
        ));
    }

    #[test]
    fn test_list_tasks() {
        let mut title = "title".to_string();
        let mut description = "desc".to_string();

        let mut repo = setup_test_repo();
        repo.add_task(title, description);
        title = "title1".to_string();
        description = "description2".to_string();
        repo.add_task(title, description);
        let tasks = repo.list_tasks().unwrap();
        assert!(matches!(tasks.len(), 2));
    }

    #[test]
    fn test_list_tasks_empty() {
        let mut repo = setup_test_repo();

        let tasks = repo.list_tasks().unwrap();
        assert!(matches!(tasks.len(), 0));
    }

    #[test]
    fn test_get_task() {
        let mut title = "title".to_string();
        let mut description = "desc".to_string();

        let mut repo = setup_test_repo();
        repo.add_task(title, description);
        title = "title1".to_string();
        description = "description2".to_string();
        let task_create = repo.add_task(title, description).unwrap();
        let task = repo.get_task(1).unwrap();
        assert!(matches!(task_create, _task));
    }

    #[test]
    fn test_get_task_not_found() {
        let repo = setup_test_repo();

        let err = repo.get_task(1).unwrap_err();
        assert!(matches!(TodoError::InvalidIndex(1), _err));
    }

    #[test]
    fn test_remove_task() {
        let mut title = "title".to_string();
        let mut description = "desc".to_string();

        let mut repo = setup_test_repo();
        repo.add_task(title, description);
        title = "title1".to_string();
        description = "description2".to_string();
        repo.add_task(title, description);
        repo.remove_task(1);
        let tasks = repo.list_tasks();
        match tasks {
            Ok(tasks) => assert!(matches!(tasks.len(), 1)),
            Err(e) => assert!(matches!(e, TodoError::InvalidIndex(0))),
        }
    }
    #[test]
    fn test_remove_task_not_found() {
        let mut title = "title".to_string();
        let mut description = "desc".to_string();

        let mut repo = setup_test_repo();
        repo.add_task(title, description);
        title = "title1".to_string();
        description = "description2".to_string();
        repo.add_task(title, description);
        let task = repo.remove_task(0);

        assert_eq!(task, Err(TodoError::InvalidIndex(0)));
    }
    #[test]
    fn test_complete_task() {
        let mut title = "title".to_string();
        let mut description = "desc".to_string();

        let mut repo = setup_test_repo();
        repo.add_task(title, description);
        title = "title1".to_string();
        description = "description2".to_string();
        let task = repo.add_task(title, description).unwrap();

        let is_complete = repo.complete_task(task.id);

        assert_eq!(is_complete, Ok(true));
    }

    #[test]
    fn test_complete_task_not_found() {
        let mut title = "title".to_string();
        let mut description = "desc".to_string();

        let mut repo = setup_test_repo();
        repo.add_task(title, description);
        title = "title1".to_string();
        description = "description2".to_string();
        repo.add_task(title, description).unwrap();

        let is_complete = repo.complete_task(0);

        assert_eq!(is_complete, Err(TodoError::InvalidIndex(0)));
    }

    #[test]
    fn test_task_count() {
        let mut title = "title".to_string();
        let mut description = "desc".to_string();

        let mut repo = setup_test_repo();
        repo.add_task(title, description);
        title = "title1".to_string();
        description = "description2".to_string();
        repo.add_task(title, description);

        let count = repo.task_count();

        assert_eq!(count, Ok(2))
    }

    #[test]
    fn test_task_clear() {
        let mut title = "title".to_string();
        let mut description = "desc".to_string();

        let mut repo = setup_test_repo();
        repo.add_task(title, description);
        title = "title1".to_string();
        description = "description2".to_string();
        repo.add_task(title, description);

        repo.clear_all();

        assert_eq!(repo.task_count(), Ok(0));
    }

    #[test]
    fn test_task_clear_already_empty() {
        let mut repo = setup_test_repo();

        let is_cleared = repo.clear_all();

        assert_eq!(is_cleared, Err(TodoError::AlreadyEmpty));
    }
}
