package cmd

import "github.com/urfave/cli/v2"

var Version string

var appCmd = &cli.App{
	Name:    "codemerge",
	Usage:   "merge two or more files",
	Version: Version,
	Commands: []*cli.Command{
		mergeCmd,
		tokenCmd,
	},
}

func Run(args []string) error {
	return appCmd.Run(args)
}
