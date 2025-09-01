import { PUBLIC_API } from "$env/static/public";
import { random_bigint, type Metadata } from "$lib";
import { error } from "@sveltejs/kit";

export async function load() {
    const metadata = await get_metadata();

    return {
        metadata,
        header_seed: random_bigint(BigInt(metadata.combinations))
    }
}

async function get_metadata(): Promise<Metadata> {
    try {
        const res = await fetch(`${PUBLIC_API}/metadata`);
        if (res.status !== 200) {
            throw new Error(`Failed to fetch metadata: ${res.status}|${await res.text()}`);
        }

        return await res.json();
    }
    catch (e) {
        console.error(e);
        error(500, "Failed to fetch metadata");
    }
}