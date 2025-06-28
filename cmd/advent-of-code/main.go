package main

import (
	"os"

	"github.com/spf13/cobra"
)

func main() {
	cmd := &cobra.Command{
		Use:   "advent-of-code",
	}

	if err := cmd.Execute(); err != nil {
		os.Exit(1)
	}
}
