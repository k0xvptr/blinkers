import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/core';

// 1. Initialize the Native Tauri Window instance
const appWindow = getCurrentWindow();

// 2. Fetch DOM Layout Elements
const pauseBtn = document.getElementById('btn-pause');
const toggleDrawerBtn = document.getElementById('btn-toggle-drawer');
const drawer = document.getElementById('creation-drawer');
const drawerIcon = document.getElementById('icon-drawer');
const createForm = document.getElementById('form-create-container');
const stateBadge = document.getElementById('current-state-badge');
const taskState = document.getElementById('task-state');
const taskList = document.getElementById('tasks');
const addTaskButton = document.getElementById('add-task');
const setupList = document.getElementById('setup');
const addSetupButton = document.getElementById('add-setup');
const breakList = document.getElementById('breaks');
const addBreakButton = document.getElementById('add-break');

// 3. Application State Flags
let isCurrentlyPaused = false;
let isDrawerOpen = false;

// 4. Exact Pixel Dimensions for the Operating System Window Frame
// (Width, Height) matching our Tailwind sizing layouts
const COMPACT_SIZE = new LogicalSize(460, 68);
const EXPANDED_SIZE = new LogicalSize(460, 260);

// 5. Clean, Multi-OS SVG Path Definitions
const playSVG = `<svg class="w-3.5 h-3.5 text-amber-400" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>`;
const pauseSVG = `<svg class="w-3.5 h-3.5 text-slate-400 hover:text-slate-100" fill="currentColor" viewBox="0 0 24 24"><path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/></svg>`;

// 6. Variables
let task_number = 0;
let setup_number = 0;
let break_number = 0;

// Force the OS window to match our tight, compact HUD bar styling when the page loads
window.addEventListener('DOMContentLoaded', () => {
  appWindow.setSize(COMPACT_SIZE);
});

async function handleSubmit(type : string, list : string[]) {
 try {
	 const ID : string = crypto.randomUUID();
	 await invoke<void>("create_container", { containerType: type, tasks : list, id : ID });
 } catch (errorMessage) {
		console.error("Command Failed: ", errorMessage);
	}
} 

/**
 * Handles the "Music Player" Play/Pause Style Toggle Behavior
 */
function handlePlaybackToggle() {
  isCurrentlyPaused = !isCurrentlyPaused;

  if (isCurrentlyPaused) {
    // STATE: App is Frozen
    if (pauseBtn) {
      pauseBtn.innerHTML = playSVG;
      pauseBtn.title = "Resume Task";
    }
    if (stateBadge) {
      stateBadge.innerText = 'PAUSED';
      stateBadge.className = 'text-[9px] font-semibold bg-amber-500/10 text-amber-400 border border-amber-500/20 px-1.5 py-0.5 rounded-full uppercase';
    }

    console.log("IPC: Sending Pause Command down to Go Engine...");
    // TODO: Connect Tauri IPC Bridge
    // invoke("pause_timer");

  } else {
    // STATE: App is Running
    if (pauseBtn) {
      pauseBtn.innerHTML = pauseSVG;
      pauseBtn.title = "Pause Task";
    }
    if (stateBadge) {
      stateBadge.innerText = 'SETUP'; // Dynamic depending on the active container state
      stateBadge.className = 'text-[9px] font-semibold bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 px-1.5 py-0.5 rounded-full uppercase';
    }

    console.log("IPC: Sending Resume Command down to Go Engine...");
    // TODO: Connect Tauri IPC Bridge
    // invoke("resume_timer");
  }
}

/**
 * Handles the Slidable Drawer Container Animation & OS Window Resizing
 */
function toggleDrawer() {
  isDrawerOpen = !isDrawerOpen;

  if (isDrawerOpen) {
    // Expand Sequence: Grow OS window FIRST, then animate UI elements down
    appWindow.setSize(EXPANDED_SIZE).then(() => {
      if (drawer) {
        drawer.style.maxHeight = '300px'; 
        drawer.classList.remove('border-t-border-slate-800/0');
        drawer.classList.add('border-t', 'border-slate-800/60');
      }
      // Rotate the plus icon (+) 45 degrees to turn it into a close cross (x)
      if (drawerIcon) {
        drawerIcon.classList.add('rotate-45');
      }
    });
  } else {
    // Collapse Sequence: Animate UI elements up, then shrink the OS window box
    if (drawer) {
      drawer.style.maxHeight = '0px';
      drawer.classList.remove('border-t', 'border-slate-800/60');
    }
    if (drawerIcon) {
      drawerIcon.classList.remove('rotate-45');
    }

    // Delay shrinking the native frame until the 300ms CSS height slide animation clears
    setTimeout(() => {
      if (!isDrawerOpen) {
        appWindow.setSize(COMPACT_SIZE);
      }
    }, 300);
  }
}

function addTask() {
	const newInput = document.createElement('input');
	newInput.type = 'text';
	newInput.name = 'objective-tasks';
	task_number++;
	let n_str = String(task_number);
	newInput.placeholder = n_str + '.';
	newInput.className = "w-full bg-slate-900 border border-slate-700 rounded px-2.5 py-1.5 text-xs text-slate-100 focus:outline-none focus:border-indigo-500 placeholder:text-slate-500"
	if (taskList) taskList.insertBefore(newInput, addTaskButton);
}


function addSetup() {
	const newInput = document.createElement('input');
	newInput.type = 'text';
	newInput.name = 'setup-tasks';
	setup_number++;
	let n_str = String(setup_number);
	if (setup_number == 1) {
		newInput.placeholder = n_str + '.' + '(Anchor)';
		newInput.required = true;
	} else {
		newInput.placeholder = n_str + '.';
	}
	newInput.className = "w-full bg-slate-900 border border-slate-700 rounded px-2.5 py-1.5 text-xs text-slate-100 focus:outline-none focus:border-indigo-500 placeholder:text-slate-500"
	if (setupList) setupList.insertBefore(newInput, addSetupButton);
}


function addBreak() {
	const newInput = document.createElement('input');
	newInput.type = 'text';
	newInput.name = 'break-tasks';
	break_number++;
	let n_str = String(break_number);
	if (break_number == 1) {
		newInput.placeholder = n_str + '.' + '(Transition)';
	} else {
		newInput.placeholder = n_str + '.';
		newInput.className = "w-full bg-slate-900 border border-slate-700 rounded px-2.5 py-1.5 text-xs text-slate-100 focus:outline-none focus:border-indigo-500 placeholder:text-slate-500";
	}
	if (breakList) breakList.insertBefore(newInput, addBreakButton);
}

// Attach Event Listeners to Buttons
if (pauseBtn) pauseBtn.addEventListener('click', handlePlaybackToggle);
if (toggleDrawerBtn) toggleDrawerBtn.addEventListener('click', toggleDrawer);
if (addTaskButton) addTaskButton.addEventListener('click', addTask);
if (addSetupButton) addSetupButton.addEventListener('click', addSetup);
if (addBreakButton) addBreakButton.addEventListener('click', addBreak);

// Intercept form submissions to extract data structures and deploy them to Go
if (createForm) {
	createForm.addEventListener('submit', (e) => {
    e.preventDefault(); // Stop standard browser page-reloads
   	
		const formElement = e.currentTarget as HTMLFormElement;
  	const formData = new FormData(formElement); 	 
		const task_list = formData.getAll('objective-tasks') as string[]; 
		const setup_list = formData.getAll('setup-tasks') as string[];
		const break_list = formData.getAll('break-tasks') as string[];

		// Call Rust Functions to store lists
		const types : string[] = ["setup", "work", "break"];
		const lists = [setup_list, task_list, break_list];
		for (let i = 0; i < 3; ++i) {
			handleSubmit(types[i], lists[i]);	
		}
		
		console.log("Form Submitted!!");

    // Reset and collapse the form drawer back up cleanly after submission
    formElement.reset();
		
		if (taskList) {
      const extraTasks = taskList.querySelectorAll('input[name="objective-tasks"]');
      extraTasks.forEach((input, index) => { if (index > 0) input.remove(); });
    }

    if (setupList) {
      const extraSetups = setupList.querySelectorAll('input[name="setup-tasks"]');
      extraSetups.forEach((input, index) => { if (index > 0) input.remove(); });
    }

    if (breakList) {
      const extraBreaks = breakList.querySelectorAll('input[name="break-tasks"]');
      extraBreaks.forEach((input, index) => { if (index > 0) input.remove(); });
    }

		task_number = 0;
		setup_number = 0;
		break_number = 0;
    toggleDrawer();
  });
}




