import { type Config, configSchema, type Context } from "@modules/config/config.ts";
import path from "node:path";
import YAML from 'yaml';
import { join } from "path";

export const loadConfig = async (path: string, context?: string): Promise<Context | null> => {
    const data: string | null = await Bun.file(path).text();
    const config: Config = configSchema.parse(YAML.parse(data));

    // Default the context to 'default' if it's not provided
    context = context || 'default';

    let contextConfig = config.contexts.find(ctx => ctx.context === context);

    // If the context wasn't found and context is 'default', try to get the first context
    if (!contextConfig && context === 'default' && config.contexts.length > 0) {
        contextConfig = config.contexts[0];
    }

    if (!contextConfig) {
        throw new Error(`Context '${context}' not found in config.`);
    }
    return contextConfig;
};

export type LoadArgs = {
    cwd?: string;
    configPath?: string;
    context?: string;
    silent?: boolean;
}

export async function load(args: LoadArgs): Promise<Context | undefined> {
    try {
        //@ts-ignore
        const cwd: string = args.cwd ?? Bun.cwd;

        const configPath = args.configPath ?? join(cwd, '.codemerge.yaml');
        const config = await loadConfig(configPath, args.context);
        if (!config) {
            return;
        }
        return config;
    } catch (error) {
        if (!args.silent) {
            console.error(error);
            throw error;
        }
    }
}