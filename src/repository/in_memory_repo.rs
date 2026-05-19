use super::TodoRepository;
use crate::{Task, TodoError};
#[derive(Default)]
pub struct InMemoryRepo {
    tasks: Vec<Task>,
    next_id: usize,
}

impl InMemoryRepo {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
}

impl TodoRepository for InMemoryRepo {
    fn add_task(&mut self, title: String, description: String) -> Result<Task, TodoError> {
        if title.trim().is_empty() {
            return Err(TodoError::EmptyTitle);
        }
        if description.trim().is_empty() {
            return Err(TodoError::EmptyDescription);
        }
        let task = Task {
            id: self.next_id,
            title,
            description,
            completed: false,
        };
        self.tasks.push(task.clone());
        self.next_id += 1;
        Ok(task)
    }
    fn list_tasks(&self) -> Result<Vec<Task>, TodoError> {
        Ok(self.tasks.clone())
    }
    fn get_task(&self, id: usize) -> Result<Option<Task>, TodoError> {
        let task = self.tasks.iter().find(|t| t.id == id);
        match task {
            Some(t) => Ok(Some(t.clone())),
            None => Err(TodoError::InvalidIndex(id)),
        }

        // Ok(task)
    }
    fn remove_task(&mut self, id: usize) -> Result<Option<Task>, TodoError> {
        // .position() finds the 0-based vector index where the condition is true
        let position = self.tasks.iter().position(|t| t.id == id);

        match position {
            Some(idx) => Ok(Some(self.tasks.remove(idx))),
            None => Err(TodoError::InvalidIndex(id)),
        }
    }
    fn complete_task(&mut self, id: usize) -> Result<bool, TodoError> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            if task.completed {
                return Err(TodoError::TaskAlreadyComplete(id));
            }
            task.completed = true;
            Ok(task.completed)
        } else {
            Err(TodoError::InvalidIndex(id))
        }
    }

    fn task_count(&self) -> Result<usize, TodoError> {
        Ok(self.tasks.len())
    }
    fn clear_all(&mut self) -> Result<(), TodoError> {
        if self.tasks.len() == 0 {
            return Err(TodoError::AlreadyEmpty);
        }
        self.tasks.clear();
        self.next_id = 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_task_validation() {
        let mut repo = InMemoryRepo::new();
        let err = repo
            .add_task("".to_string(), "description".to_string())
            .unwrap_err();
        assert!(matches!(err, TodoError::EmptyTitle));
    }

    #[test]
    fn test_add_task_validation_description() {
        let mut repo = InMemoryRepo::new();
        let err = repo
            .add_task("title".to_string(), "".to_string())
            .unwrap_err();
        assert!(matches!(err, TodoError::EmptyDescription));
    }
    #[test]
    fn test_add_task() {
        let title = "title".to_string();
        let description = "desc".to_string();
        let mut repo = InMemoryRepo::new();
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

        let mut repo = InMemoryRepo::new();
        repo.add_task(title, description);
        title = "title1".to_string();
        description = "description2".to_string();
        repo.add_task(title, description);
        let tasks = repo.list_tasks().unwrap();
        assert!(matches!(tasks.len(), 2));
    }

    #[test]
    fn test_list_tasks_empty() {
        let mut repo = InMemoryRepo::new();

        let tasks = repo.list_tasks().unwrap();
        assert!(matches!(tasks.len(), 0));
    }

    #[test]
    fn test_get_task() {
        let mut title = "title".to_string();
        let mut description = "desc".to_string();

        let mut repo = InMemoryRepo::new();
        repo.add_task(title, description);
        title = "title1".to_string();
        description = "description2".to_string();
        let task_create = repo.add_task(title, description).unwrap();
        let task = repo.get_task(1).unwrap();
        assert!(matches!(task_create, _task));
    }

    #[test]
    fn test_get_task_not_found() {
        let repo = InMemoryRepo::new();

        let err = repo.get_task(1).unwrap_err();
        assert!(matches!(TodoError::InvalidIndex(1), _err));
    }

    #[test]
    fn test_remove_task() {
        let mut title = "title".to_string();
        let mut description = "desc".to_string();

        let mut repo = InMemoryRepo::new();
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

        let mut repo = InMemoryRepo::new();
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

        let mut repo = InMemoryRepo::new();
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

        let mut repo = InMemoryRepo::new();
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

        let mut repo = InMemoryRepo::new();
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

        let mut repo = InMemoryRepo::new();
        repo.add_task(title, description);
        title = "title1".to_string();
        description = "description2".to_string();
        repo.add_task(title, description);

        repo.clear_all();

        assert_eq!(repo.task_count(), Ok(0));
    }

    #[test]
    fn test_task_clear_already_empty() {
        let mut repo = InMemoryRepo::new();

        let is_cleared =repo.clear_all();

        assert_eq!(is_cleared, Err(TodoError::AlreadyEmpty));
    }
}
