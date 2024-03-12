package cmd

import (
	"fmt"
	"github.com/gelleson/codemerge/codemerge/pkg/walker"
	"github.com/spf13/afero"
	"github.com/urfave/cli/v2"
	"os"
)

var mergeCmd = &cli.Command{
	Name:    "merge",
	Aliases: []string{"m"},
	Usage:   "merge two or more files",
	Action:  merge,
	Flags: []cli.Flag{
		&cli.StringFlag{
			Name:     "output",
			Aliases:  []string{"o"},
			Usage:    "output file",
			Required: true,
			EnvVars:  []string{"OUTPUT_FILE"},
		},
		&cli.StringSliceFlag{
			Name:    "ignores",
			Aliases: []string{"i"},
		},
		&cli.StringFlag{
			Name:    "match",
			Aliases: []string{"m"},
			Usage:   "match files",
		},
		&cli.BoolFlag{
			Name:    "verbose",
			Aliases: []string{"v"},
			Usage:   "verbose",
			Value:   true,
		},
	},
	Args: true,
}

func merge(c *cli.Context) error {
	currentDir, err := os.Getwd()
	if err != nil {
		return err
	}
	currentDir = currentDir + "/"

	fs := afero.NewOsFs()
	writer, err := fs.Create(c.String("output"))
	if err != nil {
		return err
	}

	wk := walker.New(afero.NewBasePathFs(afero.NewOsFs(), currentDir), ".", writer, c.Bool("verbose"), c.StringSlice("ignores")...)
	_, err = wk.Walk()

	if err != nil {
		return err
	}

	if c.Bool("verbose") {
		fmt.Println("Files merged into", c.String("output"))
		fmt.Println("Tokens:", wk.GetTokens())
	}

	return err
}
