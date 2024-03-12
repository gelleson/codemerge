package walker

import (
	"bufio"
	"fmt"
	ignore "github.com/sabhiram/go-gitignore"
	"github.com/spf13/afero"
	"github.com/tiktoken-go/tokenizer"
	"io/fs"
	"strings"
)

type TokenizedFile struct {
	FileName    string
	TokenLength int
}

type Walker struct {
	os             afero.Fs
	currentPath    string
	writer         afero.File
	gitignoreCache []string
	ignores        []string
	tokenizers     map[string]TokenizedFile
	verbose        bool

	tokenizer tokenizer.Codec
}

func (w *Walker) Tokenizers() map[string]TokenizedFile {
	return w.tokenizers
}

func New(os afero.Fs, currentPath string, writer afero.File, verbose bool, ingnores ...string) *Walker {
	return &Walker{os: os, currentPath: currentPath, writer: writer, verbose: verbose, tokenizers: map[string]TokenizedFile{}, ignores: ingnores}
}

func (w *Walker) Ignore(path string) bool {
	arr := []string{
		".git/",
	}
	for _, v := range w.gitignoreCache {
		arr = append(arr, v)
	}
	for _, v := range w.ignores {
		arr = append(arr, v)
	}
	object := ignore.CompileIgnoreLines(arr...)

	return object.MatchesPath(path)
}

func (w *Walker) prepareGitignorePath() string {
	currentPath := w.currentPath
	if !strings.HasSuffix(currentPath, "/") && currentPath != "." {
		currentPath += "/"
	}
	if currentPath == "." {
		return ".gitignore"
	}

	return currentPath + ".gitignore"
}

func (w *Walker) GetGitIgnores() ([]string, error) {
	return afero.Glob(w.os, ".gitignore")
}

func (w *Walker) getGitIgnore() ([]string, error) {
	ignores, err := w.GetGitIgnores()
	if err != nil {
		return nil, err
	}
	lines := make([]string, 0)
	for _, ignore := range ignores {
		f, err := w.os.Open(ignore)
		if err != nil {
			return nil, err
		}
		defer f.Close()
		scanner := bufio.NewScanner(f)
		for scanner.Scan() {
			line := scanner.Text()
			if strings.HasPrefix(line, "#") {
				continue
			}
			if strings.TrimSpace(line) == "" {
				continue
			}
			lines = append(lines, line)
		}
	}

	return lines, nil
}

func (w *Walker) process(path string, info fs.FileInfo) ([]byte, error) {
	f, err := w.os.Open(path)
	if err != nil {
		return nil, err
	}

	defer f.Close()
	content, err := afero.ReadAll(f)
	if err != nil {
		return nil, err
	}
	w.tokenizer, _ = tokenizer.Get(tokenizer.Cl100kBase)
	tokens, _, _ := w.tokenizer.Encode(string(content))

	w.tokenizers[path] = TokenizedFile{
		FileName:    info.Name(),
		TokenLength: len(tokens),
	}

	return content, nil
}

func (w *Walker) Walk() (afero.File, error) {
	gitignoreCache, err := w.getGitIgnore()
	if err != nil {
		return nil, err
	}

	w.gitignoreCache = gitignoreCache

	err = afero.Walk(w.os, w.currentPath, func(path string, info fs.FileInfo, err error) error {
		if w.Ignore(path) || strings.HasSuffix(path, ".gitignore") || path == w.writer.Name() {
			return nil
		}

		if info == nil || info.IsDir() {
			return nil
		}

		content, err := w.process(path, info)
		if err != nil {
			return err
		}
		if w.verbose {
			fmt.Println("File: " + path + " Tokens: " + fmt.Sprint(w.tokenizers[path].TokenLength))
		}
		_, err = w.writer.WriteString("File: " + path + "\n")
		if err != nil {
			return err
		}
		_, err = w.writer.Write(content)
		if err != nil {
			return err
		}
		_, err = w.writer.WriteString("\n")
		if err != nil {
			return err
		}

		return nil
	})

	_ = w.writer.Close()

	return w.writer, err
}

func (w *Walker) GetTokens() int64 {
	tokens := int64(0)

	for _, tokenizedFile := range w.tokenizers {
		tokens += int64(tokenizedFile.TokenLength)
	}

	return tokens
}

func (w *Walker) CalculateTokens() (int64, error) {
	gitignoreCache, err := w.getGitIgnore()
	if err != nil {
		return 0, err
	}

	w.gitignoreCache = gitignoreCache

	tokens := int64(0)

	err = afero.Walk(w.os, w.currentPath, func(path string, info fs.FileInfo, err error) error {
		if w.Ignore(path) || strings.HasSuffix(path, ".gitignore") || (w.writer != nil && path == w.writer.Name()) {
			return nil
		}

		if info == nil || info.IsDir() {
			return nil
		}
		if w.verbose {
			fmt.Println("File: " + path)
		}
		_, err = w.process(path, info)
		if err != nil {
			return err
		}

		return nil
	})

	if w.writer != nil {
		_ = w.writer.Close()
	}

	for _, tokenizedFile := range w.tokenizers {
		tokens += int64(tokenizedFile.TokenLength)
	}

	return tokens, err
}
