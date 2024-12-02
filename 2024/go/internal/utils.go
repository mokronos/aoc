package internal

import (
    "os"
    "log"
)

func ReadInput(filename string) string {
    content, err := os.ReadFile(filename)
    if err != nil {
        log.Fatalf("Failed to read file: %v", err)
    }
    return string(content)
}
