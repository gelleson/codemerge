import { dlopen, FFIType, CString } from "bun:ffi";

import source from '../../rust/target/release/libfile_reader.dylib' with { type: "file" };

const {symbols: {read_files_json, free_json_string, find_gitignore_files_json}} = dlopen(source, {
    read_files_json: {
        args: [FFIType.cstring],
        returns: FFIType.ptr,
    },
    free_json_string: {
        args: [FFIType.ptr],
        returns: FFIType.void,
    },
    find_gitignore_files_json: {
        args: [FFIType.cstring],
        returns: FFIType.ptr,
    },
});

export {
    read_files_json,
    free_json_string,
    find_gitignore_files_json,
}
