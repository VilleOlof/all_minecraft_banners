<script lang="ts">
    import { format_num_string, PER_PAGE, random_bigint } from "$lib";
    import Banner from "$lib/Banner.svelte";
    import type { LayoutProps } from "./$types";
    import { queryParameters } from "sveltekit-search-params";
    import dice from "$lib/assets/dice.webp";
    import { onMount } from "svelte";

    const { data }: LayoutProps = $props();
    const max = BigInt(data.metadata.combinations);
    const MAX_PAGE = max / PER_PAGE;

    const params = queryParameters(
        {
            page: {
                defaultValue: random_bigint(MAX_PAGE),
                encode: (v: BigInt) => {
                    return v.toString();
                },
                decode: (v) => {
                    if (v) return BigInt(v);
                    else return random_bigint(MAX_PAGE);
                },
            },
        },
        { showDefaults: true },
    );

    let navigation_disabled = $state(false);
    function nav() {
        navigation_disabled = true;
        setTimeout(() => {
            navigation_disabled = false;
        }, 200);
    }

    onMount(() => (params.page = params.page));
</script>

<div class="flex gap-6 flex-wrap content-start justify-center">
    {#each { length: Number(PER_PAGE) }, i}
        {@const seed = params.page * PER_PAGE + BigInt(i)}
        {#if seed <= max}
            <Banner {seed} />
        {/if}
    {/each}
</div>

<div class="flex flex-col gap-4 justify-center items-center text-2xl mt-4">
    <div class="flex gap-2 mt-4">
        <button
            disabled={navigation_disabled}
            class="cursor-pointer disabled:opacity-50 bg-neutral-950 px-3.5 py-1 rounded-sm hover:bg-neutral-800 transition-colors active:bg-neutral-900"
            onclick={() => {
                params.page -= BigInt(1);
                if (params.page < 0) {
                    params.page = BigInt(0);
                }
                nav();
            }}
        >
            {"<"}
        </button>
        <p class="text-3xl items-center flex gap-2">
            <input
                onchange={(e) => {
                    if (!e.target) return;
                    let value = (e.target as HTMLInputElement).value;
                    value = value.replaceAll(",", "");
                    value = value.trim();
                    let new_page = BigInt(value);
                    if (new_page < 0) {
                        new_page = BigInt(0);
                    }
                    if (new_page > MAX_PAGE) {
                        new_page = MAX_PAGE;
                    }
                    params.page = new_page;
                }}
                type="text"
                class="w-min bg-neutral-950 px-2 outline-0 hover:bg-neutral-800 focus:bg-neutral-800 rounded-sm transition-colors py-1"
                value={format_num_string(params.page.toString())}
            />
            <button
                class="disabled:opacity-50"
                disabled={navigation_disabled}
                onclick={() => {
                    params.page = random_bigint(MAX_PAGE);
                    nav();
                }}
            >
                <img
                    src={dice}
                    alt=""
                    class="w-11 h-11 rounded-sm z-10 hover:bg-neutral-800 cursor-pointer transition-colors bg-neutral-950 p-2"
                />
            </button>
            <span class="bg-neutral-950 px-2 py-1 rounded-sm"
                >{format_num_string(MAX_PAGE.toString())}</span
            >
        </p>
        <button
            disabled={navigation_disabled}
            class="cursor-pointer disabled:opacity-50 bg-neutral-950 px-3.5 py-1 rounded-sm hover:bg-neutral-800 transition-colors active:bg-neutral-900"
            onclick={() => {
                if (params.page + BigInt(1) > MAX_PAGE) {
                    params.page = MAX_PAGE;
                } else {
                    params.page += BigInt(1);
                }
                nav();
            }}
        >
            {">"}
        </button>
    </div>

    <div class="flex flex-col gap-2">
        <p class="text-neutral-400 text-sm text-center text-balance">
            Check out the project on <a
                href="https://github.com/VilleOlof/minecraft_banners"
                class="underline">Github</a
            >. Written with Rust + Sveltekit & love
        </p>
        <p class="text-neutral-500 text-xs text-center text-balance">
            NOT AN OFFICIAL MINECRAFT SERVICE. NOT APPROVED BY OR ASSOCIATED
            WITH MOJANG OR MICROSOFT.
        </p>
    </div>
</div>
