import { free_json_string, read_files_json } from "@modules/ffi";
import { CString } from "bun:ffi";


export interface LoadResult {
    path: string
    content: string
    tokens: number
    error?: string
}

class FileReaderError extends Error {
    constructor(message: string) {
        super(message);
        this.name = "FileReaderError";
    }
}


export async function loadAll(paths: string[]): Promise<LoadResult[]> {
    if (paths.length === 0) {
        return [];
    }

    const pathsJoined = paths.join("\n");

    // Encode the string as a null-terminated C string
    const encoder = new TextEncoder();
    const pathsBuffer = encoder.encode(pathsJoined + '\0');

    const jsonStringPtr = read_files_json(pathsBuffer);
    //@ts-ignore
    if (jsonStringPtr === 0n) {
        throw new FileReaderError("Failed to read files: Null result string");
    }

    // Read the returned C string
    //@ts-ignore
    const jsonString = new CString(jsonStringPtr).toString();

    // Free the JSON string allocated in Rust
    free_json_string(jsonStringPtr);

    // Parse the JSON string
    return JSON.parse(jsonString);
}

export async function load(path: string): Promise<LoadResult> {
    const results = await loadAll([path]);
    return results[0];
}