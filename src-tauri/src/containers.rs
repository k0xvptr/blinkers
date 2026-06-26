use crate::db;
use crate::db::DbState;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContainerType {
    Setup {
        setup_tasks: VecDeque<String>,
    },
    Work {
        primary_tasks: VecDeque<String>,
        backup_tasks: VecDeque<String>,
    },
    Break {
        break_tasks: VecDeque<String>,
    },
}

#[derive(Clone, Serialize)]
pub struct Container {
    pub id: String,
    pub focus_block_id: String,
    pub container_type: ContainerType,
    pub quality: Option<u8>,
    pub notes: Option<String>
}


pub struct FocusBlock {
    pub id : String,
    pub setup_container : Container,
    pub work_container : Container,
    pub break_container : Container,
    pub is_completed : bool
}

#[cfg(not(target_arch = "wasm32"))]
#[tauri::command]
pub async fn create_container(
    setup: VecDeque<String>,
    tasks: VecDeque<String>,
    breaks: VecDeque<String>,
    db_state: State<'_, DbState>,
) -> Result<(), String> { 
    
    // Create the IDs of all containers
    let main_id = Uuid::new_v4().to_string();
    let setup_id = Uuid::new_v4().to_string();
    let work_id = Uuid::new_v4().to_string();
    let break_id = Uuid::new_v4().to_string();

    // Create Setup Container
    let container_setup = Container {
        id: setup_id,
        focus_block_id: main_id.clone(),
        container_type: ContainerType::Setup {
            setup_tasks: setup
        },
        quality: None,
        notes: None
    };

    // Create Work Container
    let container_work = Container {
        id: work_id,
        focus_block_id: main_id.clone(),
        container_type: ContainerType::Work {
            primary_tasks: tasks,
            backup_tasks: VecDeque::new()
        },
        quality: None,
        notes: None
    };

    // Create Break Container
    let container_break = Container {
        id: break_id,
        focus_block_id: main_id.clone(),
        container_type: ContainerType::Break {
            break_tasks: breaks
        },
        quality: None,
        notes: None
    };
    
    // Create The New FocusBlock
    let newblock = FocusBlock {
        id: main_id,
        setup_container: container_setup,   
        work_container: container_work,
        break_container: container_break,
        is_completed: false
    };
    db::insert_container(db_state, &newblock).await?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
#[tauri::command]
pub async fn fetch_container() -> Result<(), String> {

    Ok(())
}
