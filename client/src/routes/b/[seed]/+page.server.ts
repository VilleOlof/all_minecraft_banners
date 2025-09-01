import { PUBLIC_API } from '$env/static/public';
import type { Patterns } from '$lib';
import { error } from '@sveltejs/kit';

export async function load({ params }) {
    const { seed } = params;
    if (!seed) {
        error(404, "No seed provided");
    }

    const patterns = await fetch_patterns(seed);

    return {
        seed,
        patterns
    }
}

async function fetch_patterns(seed: string): Promise<Patterns> {
    try {
        const res = await fetch(`${PUBLIC_API}/pattern/${seed}`);
        if (res.status !== 200) {
            throw new Error(`Failed to fetch patterns: ${res.status}|${await res.text()}`);
        }

        return await res.json();
    }
    catch (e) {
        console.error(e);
        error(500, "Failed to fetch patterns");
    }
}