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

impl TodoRepository for InMemoryRepo{
    fn add_task(&mut self,title:String,description: String)->Result<Task,TodoError>{
        if title.trim().is_empty(){
            return Err(TodoError::EmptyTitle);
        }
        if description.trim().is_empty(){
            return Err(TodoError::EmptyDescription);
        }
        let task=Task{
            id:self.next_id,
            title,
            description,
            completed:false,

        };
        self.tasks.push(task.clone());
        self.next_id+=1;
        Ok(task)
    }
    fn list_tasks(&self)->Result<Vec<Task>,TodoError>{
        Ok(self.tasks.clone())
    }
    fn get_task(&self, id:usize)->Result<Option<Task>,TodoError>{
        Ok(self.tasks.iter().find(|t| t.id==id).cloned())
    }
    fn remove_task(&mut self, id:usize)->Result<Option<Task>,TodoError>{
        let position = self.tasks.iter().position(|t| t.id==id);
        match position{
            Some(idx)=>Ok(Some(self.tasks.remove(idx))),
            None=> Err(TodoError::InvalidIndex(id)),
        }
    }
    fn complete_task(&mut self, id :usize)->Result<bool,TodoError>{
        if let Some(task)=self.tasks.iter_mut().find(|t| t.id==id){
            if task.completed{
                return Err(TodoError::TaskAlreadyComplete(id))
            }
            task.completed=true;
            Ok(task.completed)
        }else{
            Err(TodoError::InvalidIndex(id))
          
        }
    }

    fn task_count(&self)->Result<usize,TodoError>{
        Ok(self.tasks.len())
    }
    fn clear_all(&mut self)->Result<(),TodoError>{
        if self.tasks.len()==0{
            return Err(TodoError::AlreadyEmpty);
        }
        self.tasks.clear();
        self.next_id=1;
        Ok(())


    }
}