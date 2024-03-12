package cmd

import (
	"fmt"
	"github.com/gelleson/codemerge/codemerge/pkg/walker"
	"github.com/spf13/afero"
	"github.com/urfave/cli/v2"
	"os"
	"sort"
)

var tokenCmd = &cli.Command{
	Name:    "tokens",
	Aliases: []string{"t"},
	Flags: []cli.Flag{
		&cli.IntFlag{
			Name:    "count",
			Aliases: []string{"c"},
			Usage:   "count",
			Value:   10,
		},
		&cli.StringSliceFlag{
			Name:    "ignores",
			Aliases: []string{"i"},
		},
		&cli.BoolFlag{
			Name:    "verbose",
			Aliases: []string{"v"},
			Usage:   "verbose",
			Value:   true,
		},
	},
	Usage:  "tokens",
	Action: tokens,
}

func tokens(c *cli.Context) error {
	currentDir, err := os.Getwd()
	if err != nil {
		return err
	}
	currentDir = currentDir + "/"

	wk := walker.New(afero.NewBasePathFs(afero.NewOsFs(), currentDir), ".", nil, c.Bool("verbose"), c.StringSlice("ignores")...)
	tokens_, err := wk.CalculateTokens()
	if err != nil {
		return err
	}
	count := c.Int("count") + 1
	arr := make([]walker.TokenizedFile, 0)

	for _, file := range wk.Tokenizers() {
		arr = append(arr, file)
	}

	sort.Slice(arr, func(i, j int) bool {
		return arr[i].TokenLength > arr[j].TokenLength
	})

	if len(arr) < count {
		count = len(arr)
	}

	fmt.Printf("Top %d files with most tokens\n", count)
	for _, file := range arr[:count] {
		fmt.Println(file.FileName, ":", file.TokenLength)
	}
	fmt.Println("Tokens: ", tokens_)

	return err
}
