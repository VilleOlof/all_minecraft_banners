<script lang="ts">
    import { PUBLIC_API } from "$env/static/public";
    import { RESOLVE_COLOR, RESOLVE_PATTERN } from "$lib/asset_resolver";
    import type { PageProps } from "./$types";
    import ColorDropdown from "$lib/ColorDropdown.svelte";
    import PatternDropdown from "$lib/PatternDropdown.svelte";

    let { data }: PageProps = $props();
    let base_color = $state({
        value: data.patterns.base,
        label: data.patterns.base,
    });
    let pattern_colors = $state(
        data.patterns.patterns.map(([_, col]) => {
            return { value: col, label: col };
        }),
    );
    let pattern_patterns = $state(
        data.patterns.patterns.map(([pat, _]) => {
            return { value: pat, label: pat };
        }),
    );

    let is_modified = $state(false);
    function get_partial_banner_url(layer: number) {
        // 1 for base, 6 for layers & 1 for the bigger one
        // if this function runs anymore than those, its been modified

        let query = "";

        for (let i = 0; i < layer; i++) {
            let color = pattern_colors[i].value;
            let pattern = pattern_patterns[i].value;
            if (pattern === "None") continue;
            query += `&layers=[${obj_index(RESOLVE_PATTERN, pattern)},${obj_index(RESOLVE_COLOR, color)}]`;
        }

        return `${PUBLIC_API}/create?base_color=${obj_index(RESOLVE_COLOR, base_color.value)}${query}`;
    }

    function obj_index(obj: Object, key: string): number {
        return Object.keys(obj).indexOf(key);
    }
</script>

<svelte:head>
    <title>Banner #{data.seed}</title>

    <meta property="og:title" content="All Minecraft Banners" />
    <meta
        property="og:image"
        content="{PUBLIC_API}/banner/{data.seed}?width=128"
    />
    <meta
        property="og:description"
        content="View the beautiful creation of banner #{data.seed}"
    />
</svelte:head>

<div class="flex w-full h-full flex-col items-center justify-center gap-4">
    <div class="flex flex-col lg:flex-row gap-4 h-full lg:h-auto">
        <!-- i dont know why we need the 18 extra margin on top but otherwise half the banner is out of the viewport -->
        <img
            src={get_partial_banner_url(6)}
            alt=""
            class="w-auto h-[20rem] lg:h-[43.3rem] object-contain pixelated"
        />

        <div class="flex flex-col justify-start gap-1">
            <h2 class="text-3xl items-end gap-3 flex">
                Banner <span class="text-base text-neutral-400"
                    >#{data.seed}{is_modified ? "*" : ""}</span
                >
            </h2>

            <div class="w-full h-1 bg-neutral-800 my-1 rounded-sm"></div>

            <div class="flex flex-col gap-2">
                <div class="flex gap-2">
                    {#key base_color}
                        <img
                            src={get_partial_banner_url(0)}
                            alt=""
                            class="pixelated w-10.5 h-21 object-contain"
                        />
                    {/key}

                    <div class="flex flex-col w-11/12">
                        <ColorDropdown
                            bind:value={base_color}
                            bind:is_modified
                        />
                    </div>
                </div>

                {#each data.patterns.patterns as _, i}
                    <div class="flex gap-2">
                        {#key pattern_colors}
                            <img
                                src={get_partial_banner_url(i + 1)}
                                alt="Layer {i + 1}"
                                class="pixelated w-10.5 h-21 object-contain"
                            />
                        {/key}

                        <div class="flex flex-col text-lg w-11/12">
                            <ColorDropdown
                                bind:value={pattern_colors[i]}
                                bind:is_modified
                            />
                            <PatternDropdown
                                bind:value={pattern_patterns[i]}
                                bind:is_modified
                            />
                        </div>
                    </div>
                {/each}
            </div>
        </div>

        <div class="flex pb-4 lg:hidden">
            <p class="text-balance text-center text-neutral-400">
                Any changes you do here won't save.
            </p>
        </div>
    </div>
    <div class="flex pb-4 lg:block">
        <p class="text-balance text-center text-neutral-400">
            Any changes you do here won't save.
        </p>
    </div>
</div>
