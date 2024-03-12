package walker

import (
	"github.com/spf13/afero"
	"github.com/tiktoken-go/tokenizer"
	"testing"
)

func TestWalker_GetGitIgnores(t *testing.T) {
	mem := afero.NewMemMapFs()
	walker := Walker{
		os:          mem,
		currentPath: ".",
	}

	ig1, err := mem.Create(".gitignore")
	if err != nil {
		t.Error(err)
	}
	ig1.WriteString("test")
	ig1.Close()

	ig2, err := mem.Create("test/.gitignore")
	if err != nil {
		t.Error(err)
	}
	ig2.WriteString("test")
	ig2.Close()

	ig3, err := mem.Create("test2/.gitignore")
	if err != nil {
		t.Error(err)
	}
	ig3.WriteString("test")
	ig3.Close()

	ignores, err := walker.GetGitIgnores()
	if err != nil {
		t.Error(err)
	}
	if len(ignores) != 1 {
		t.Error("Expected 1 ignores, got", len(ignores))
	}
}

func TestWalker_Gitignore(t *testing.T) {
	mem := afero.NewMemMapFs()
	walker := Walker{
		os:          mem,
		currentPath: ".",
	}

	ig1, err := mem.Create(".gitignore")
	if err != nil {
		t.Error(err)
	}
	ig1.WriteString("test\n")
	ig1.WriteString("test\n")
	ig1.Close()
	lines, err := walker.getGitIgnore()
	if err != nil {
		t.Error(err)
	}
	if len(lines) != 2 {
		t.Error("Expected 2 ignore, got", len(lines))
	}
}

func TestWalker_Walk(t *testing.T) {
	mem := afero.NewMemMapFs()
	writer, _ := mem.Create("output.txt")
	walker := Walker{
		os:          mem,
		currentPath: ".",
		writer:      writer,
	}

	ig1, err := mem.Create(".gitignore")
	if err != nil {
		t.Error(err)
	}
	ig1.WriteString(".idea/\n")
	ig1.Close()

	ig2, err := mem.Create(".idea/file.txt")
	if err != nil {
		t.Error(err)
	}
	ig2.WriteString("test")
	ig2.Close()

	ig3, err := mem.Create("file.txt")
	if err != nil {
		t.Error(err)
	}
	ig3.WriteString("a")
	ig3.Close()

	_, err = walker.Walk()
	if err != nil {
		t.Error(err)
	}
	writer2, _ := mem.Open("output.txt")
	content, _ := afero.ReadAll(writer2)
	if string(content) != "File: file.txt\na\n" {
		t.Error("Expected 'File: file.txt\na\n', got", string(content))
	}
}

func TestWalker_CalculateTokens(t *testing.T) {
	mem := afero.NewMemMapFs()
	writer, _ := mem.Create("output.txt")
	walker := Walker{
		os:          mem,
		currentPath: ".",
		writer:      writer,
		verbose:     true,
	}

	ig1, err := mem.Create(".gitignore")
	if err != nil {
		t.Error(err)
	}
	ig1.WriteString(".idea/\n")
	ig1.Close()

	ig2, err := mem.Create(".idea/file.txt")
	if err != nil {
		t.Error(err)
	}
	ig2.WriteString("test")
	ig2.Close()

	ig3, err := mem.Create("file.txt")
	if err != nil {
		t.Error(err)
	}
	ig3.WriteString("a")
	ig3.Close()

	tokens, err := walker.CalculateTokens()
	if err != nil {
		t.Error(err)
	}
	if tokens != 1 {
		t.Error("Expected 1 token, got", tokens)
	}
}

func TestWalker_Subset(t *testing.T) {
	mem := afero.NewMemMapFs()
	writer, _ := mem.Create("output.txt")
	walker := Walker{
		os:          mem,
		currentPath: ".",
		writer:      writer,
		verbose:     true,
	}

	files := map[string][]string{
		".gitignore": {
			".idea/",
		},
		".idea/file.txt": {
			"test",
		},
		"file.txt": {
			"a",
		},
	}
	generateFiles(mem, files)

	tokens, err := walker.CalculateTokens()
	if err != nil {
		t.Error(err)
	}
	if tokens != 1 {
		t.Error("Expected 1 token, got", tokens)
	}
}

func generateFiles(fs afero.Fs, files map[string][]string) map[string]afero.File {
	created := make(map[string]afero.File)
	for name, contents := range files {
		file, err := fs.Create(name)
		for _, content := range contents {
			if err != nil {
				panic(err)
			}
			_, err = file.WriteString(content)
			if err != nil {
				panic(err)
			}
		}
		created[name] = file
	}
	return created

}
func BenchmarkTokenizer(b *testing.B) {
	for i := 0; i < b.N; i++ {
		coder, _ := tokenizer.Get(tokenizer.Cl100kBase)
		_, _, _ = coder.Encode("supercalifragilistic")
	}
}
