package main

type Status int

const (
	WORK = iota
	BREAK
)


type TaskContainer struct {
	ID string `json:"id"`	
	AnchorTask string `json:"anchor-task"`
	ActiveQueue []string `json:"active-queue"`
	BackUpQueue []string `json:"backup-queue"`
}

type BreakContainer struct {
	Anchor_TaskContainer *TaskContainer `json:"anchor-task-container"`
	BreakActivity string `json:"break-activity"`
	BreakDuration int `json:"break-duration"`
}






