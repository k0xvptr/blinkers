use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

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
    pub container_type: ContainerType,
    pub quality: Option<u8>,
    pub notes: Option<String>,
    pub is_active : bool
}

#[cfg(not(target_arch = "wasm32"))]
#[tauri::command]
pub fn create_container(container_type: String, tasks: VecDeque<String>, container_id : String) -> Result<(), String> {
    let l_container_type = container_type.to_lowercase();
    let ctype: ContainerType = match l_container_type.as_str() {
        "setup" => ContainerType::Setup { setup_tasks: tasks },
        "work" => ContainerType::Work {
            primary_tasks: tasks,
            backup_tasks: VecDeque::new(),
        },
        "break" => ContainerType::Break { break_tasks: tasks },
        _ => return Err(String::from("Invalid Type of Container!!!")),
    };

    // Create The New Container
    let container = Container {
        id : container_id,
        container_type: ctype,
        quality: None,
        notes: None,
        is_active : false
    };
    Ok(())
}
