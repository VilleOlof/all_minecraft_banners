import { PUBLIC_API } from "$env/static/public";
import { random_bigint, type Metadata } from "$lib";
import { error } from "@sveltejs/kit";

const MAX_CACHE_TIME: number = 60 * 60 * 1000;
let metadata_cache: { fetched_at: number, data: Metadata | null } = {
    fetched_at: 0,
    data: null
};

export async function load() {
    const metadata = await get_metadata();

    return {
        metadata,
        header_seed: random_bigint(BigInt(metadata.combinations))
    }
}

async function get_metadata(): Promise<Metadata> {
    if (metadata_cache.data) {
        if (Date.now() > (metadata_cache.fetched_at + MAX_CACHE_TIME)) {
            metadata_cache.data = null;
        }
        else {
            return metadata_cache.data;
        }
    }

    try {
        const res = await fetch(`${PUBLIC_API}/metadata`);
        if (res.status !== 200) {
            throw new Error(`Failed to fetch metadata: ${res.status}|${await res.text()}`);
        }

        const data = await res.json();
        metadata_cache.data = data;
        metadata_cache.fetched_at = Date.now();
        return data;
    }
    catch (e) {
        console.error(e);
        error(500, "Failed to fetch metadata");
    }
}