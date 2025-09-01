<script lang="ts" generics="T">
    import Select from "svelte-select";
    import { RESOLVE_COLOR } from "./asset_resolver";

    let {
        value = $bindable(),
        is_modified = $bindable(),
    }: { value: { value: T; label: T }; is_modified: boolean } = $props();
</script>

<Select
    --border-radius="0"
    --list-border-radius="0"
    --multi-item-border-radius="0"
    --item-first-border-radius="0"
    --background="#0a0a0a"
    --border="0"
    --list-border="0"
    --border-focused="0"
    --border-hover="0"
    --list-background="#0a0a0a"
    --placeholder-color="#e5e5e5"
    --selected-item-color="#e5e5e5"
    --item-color="#e5e5e5"
    --item-active-background="#e62222"
    --item-hover-bg="#404040"
    --item-is-active-bg="#262626"
    containerStyles="width: 100%"
    items={Object.keys(RESOLVE_COLOR)}
    clearable={false}
    showChevron={true}
    on:change={() => {
        is_modified = true;
    }}
    bind:value
>
    <div slot="selection" let:selection class="flex items-center gap-1.5">
        <img
            src="/dye/{RESOLVE_COLOR[selection.value].item}.webp"
            alt=""
            class="pixelated w-6 h-6"
        />
        <p>{RESOLVE_COLOR[selection.value].name}</p>
    </div>

    <div slot="item" let:item class="flex items-center gap-1.5">
        <img
            src="/dye/{RESOLVE_COLOR[item.value].item}.webp"
            alt=""
            class="pixelated w-6 h-6"
        />
        <p>{RESOLVE_COLOR[item.value].name}</p>
    </div>
</Select>
