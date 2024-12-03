import { buildGitIgnores, gitignore } from "@modules/gitignore";
//@ts-ignore
import { glob } from "glob-gitignore";
import { join } from "path";

type MatchOptions = {
    path: string
    ignores: string[]
    filters: string[]
    absolute?: boolean
}

function absolute(path: string): (val: string) => string {
    return (val: string) => join(path, val)
}

function identity(val: string): string {
    return val
}

export async function match(options: MatchOptions) {
    const ignore = await buildGitIgnores(options.path, gitignore);
    for (const i of options.ignores) {
        ignore.add(i)
    }
    ignore.add(".git/")
    ignore.add("**/.git/")
    const opts = {
        ignore: gitignore,
        cwd: options.path,
        nodir: true,
        dot: true,
    }
    return (await glob(options.filters, opts))
        .map(options.absolute ? absolute(options.path) : identity)
}

export function isCWD(path: string): boolean {
    //@ts-ignore
    return Bun.cwd !== path;
}