// Package containers provides standardized 
// objects for the app
package containers

import (
	"time"
	"net/http"
)

type Status int

const (
	IDLE Status = iota
	SETUP
	WORK
	BREAK
)

type TaskContainer struct {
	ID string `json:"id"`	
	AnchorTask string `json:"anchorTask"`
	ActiveQueue []string `json:"activeQueue"`
	BackUpQueue []string `json:"backupQueue"`
}

type SetUpContainer struct {
	TaskRefID string `json:"taskRefID"`	
	StartTime time.Time `json:"startTime"`	
}

type BreakContainer struct {
	TaskRefID string `json:"taskRefID"`
	BreakActivity string `json:"breakActivity"`
	BreakDuration int `json:"breakDuration"`
}


func CreateSetUp() SetUpContainer {
	t := time.Now()
	container := SetUpContainer{
		"",
		t,
	}
	return container
}

func CreateBreak() BreakContainer {
	container := BreakContainer{
		"",
		"Activity: ",
		0,
	}
	return container
}

func CreateTask(w http.ResponseWriter, r *http.Request) {
	str := genTaskID()
	task := TaskContainer{
		str,
		"",
		// Empty Slices
		[]string{}, 
		[]string{},
	}
	setup := CreateSetUp()
	
}

