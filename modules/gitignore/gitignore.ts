import ignore, { type Ignore } from 'ignore'
//@ts-ignore
import { glob } from 'glob-gitignore'
import { find_gitignore_files_json, free_json_string } from "@modules/ffi";
import { CString } from "bun:ffi";


export async function findGitignoreFiles(path: string): Promise<string[]> {
    // Ensure the input string is null-terminated
    const encoder = new TextEncoder();
    const pathBuffer = encoder.encode(path + '\0');

    const resultPtr = find_gitignore_files_json(pathBuffer);

    // @ts-ignore
    if (resultPtr === 0n) {
        throw new Error("Failed to find .gitignore files: Null result string");
    }

    // Read the returned C string
    // @ts-ignore
    const resultJson = new CString(resultPtr).toString();

    // Free the JSON string allocated in Rust
    free_json_string(resultPtr);

    // Parse the JSON string
    const result = JSON.parse(resultJson);

    if (result.error) {
        throw new Error(`Error finding .gitignore files: ${result.error}`);
    }

    return result.gitignore_files;
}

const preIgnoredFiles = [
    '.git/',
    '**/.git/',
    '**/node_modules/',
    '**/target/',
    '**/.idea/',
    '**/.DS_Store',
    '**/.DS_Store?',
    '**/Thumbs.db',
    '**/*.iml',
    '**/.idea/',
    '.git',
    '.env',
    'bun.lockb',
    '**/Cargo.lock',
];

export const gitignore = ignore({
    ignoreCase: true,
    allowRelativePaths: true,
})

gitignore.add(preIgnoredFiles)


export async function buildGitIgnores(
    cwd: string,
    ignore: Ignore
): Promise<Ignore> {
    const files = await findGitignoreFiles(cwd);
    for (const file of files) {
        try {
            const f = await Bun.file(file).text()

            if (file === '.gitignore') {
                ignore.add(f)
            } else {
                const path = file.replaceAll('/.gitignore', '')
                f.split('\n').forEach((line) => {
                    ignore.add(`${path}/${line}`)
                })
            }
        } catch {
            console.warn('Error reading .gitignore file:', file)
        }
    }
    return ignore
}
