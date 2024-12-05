import { z } from 'zod';

export type Preset = {
    ignores: string[];
    filters: string[];
};

export type Context = {
    context: string | 'default';
} & Preset

export type ConfigWithContextV1 = {
    version: 1
    contexts: Context[];
}

export type Config = ConfigWithContextV1;

export const configSchema = z.object({
    version: z.literal(1),
    contexts: z.array(
        z.object({
            context: z.string(),
            ignores: z.array(z.string()).optional(),
            filters: z.array(z.string()).default(["**"]),
        })
    ),
});
