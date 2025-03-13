<script lang="ts">
    import { Sun, Moon } from "lucide-svelte";

    import { resetMode, setMode } from "mode-watcher";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
    import { buttonVariants } from "$lib/components/ui/button/index.js";
</script>

<DropdownMenu.Root>
    <DropdownMenu.Trigger
        class={buttonVariants({
            variant: "outline",
            size: "icon",
            class: "bg-white dark:bg-brand",
        })}
    >
        <Sun
            class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0 text-brand-dark"
        />
        <Moon
            class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100 text-brand-dark"
        />
        <span class="sr-only">Toggle theme</span>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content align="end">
        <DropdownMenu.Item onclick={() => setMode("light")}>Light</DropdownMenu.Item>
        <DropdownMenu.Item onclick={() => setMode("dark")}>Dark</DropdownMenu.Item>
        <DropdownMenu.Item onclick={() => resetMode()}>System</DropdownMenu.Item>
    </DropdownMenu.Content>
</DropdownMenu.Root>
