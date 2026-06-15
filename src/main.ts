import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow();
const toggleDrawerBtn = document.getElementById('btn-toggle-drawer');
const drawer = document.getElementById('creation-drawer');
const drawerIcon = document.getElementById('icon-drawer');
const createForm = document.getElementById('form-create-container');

let isDrawerOpen = false;

function handleAdd() {
		
}

interface TaskContainer {
	id: string;
	anchorTask: string;
	activeQueue: string[];
	backupQueue: string[];
}




