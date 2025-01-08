import process from "node:process";

export async function readStdin(): Promise<string[]> {
    const data: string[] = [];

    for await (const chunk of process.stdin) {
        data.push(chunk.toString());
    }

    return data.map((d) => d.split('\n')).flat();
}