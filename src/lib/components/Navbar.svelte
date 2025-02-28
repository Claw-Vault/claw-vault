<script lang="ts">
    import clawVaultLogo from "$lib/images/claw-logo.png";
    import { GITHUB_LINK, Routes } from "$lib/routes";
    import { Menu, X } from "lucide-svelte";
    import ThemeSwitcher from "./ThemeSwitcher.svelte";

    let showMobileMenu = $state(false);
</script>

<header
    class="sticky inset-x-0 top-0 z-30 text-brand-dark bg-white dark:text-white dark:bg-brand-dark"
>
    <nav class="flex items-center justify-between px-6 py-4 lg:px-8">
        <div class="flex lg:flex-1 flex-row items-center">
            <a href="/" class="flex justify-center items-center text-lg font-medium">
                <img class="w-12 mr-4 rounded-full" src={clawVaultLogo} alt="Claw Vault" />
                Claw Vault
            </a>
        </div>
        <div class="flex lg:hidden">
            <button
                type="button"
                class="hover:cursor-pointer"
                onclick={() => (showMobileMenu = true)}
            >
                <Menu />
            </button>
        </div>
        <div class="hidden items-center lg:flex lg:gap-x-12 lg:justify-end">
            <a href={GITHUB_LINK} target="_blank" class="text-md font-medium leading-6">Github</a>
            <a href={Routes.Privacy} class="text-md font-medium leading-6">Privacy</a>
            <ThemeSwitcher />
        </div>
    </nav>

    <!-- Mobile menu, show/hide based on menu open state. -->
    <div class={`${showMobileMenu ? "" : "hidden"}`}>
        <!-- Background backdrop, show/hide based on slide-over state. -->
        <div
            class="fixed inset-y-0 right-0 z-30 w-full overflow-y-auto bg-gray-800 text-gray-300 px-6 py-6 sm:max-w-sm sm:ring-1 sm:ring-gray-900/10"
        >
            <div class="flex items-center justify-between">
                <div class="flex lg:flex-1 flex-col items-center">
                    <a href="/" class="flex justify-center items-center text-lg font-semibold">
                        <img class="w-12 mr-4" src={clawVaultLogo} alt="Claw Vault" />
                        Claw Vault
                    </a>
                </div>
                <button type="button" class="-m-2.5 rounded-md p-2.5 text-gray-700">
                    <span class="sr-only">Close menu</span>
                    <X />
                </button>
            </div>
            <div class="mt-6">
                <div class="-my-6">
                    <div class="space-y-2 py-6">
                        <a
                            href="https://github.com/Claw-Vault/claw-vault"
                            target="_blank"
                            class="-mx-3 block rounded-lg px-3 py-2 text-base font-semibold leading-7 hover:bg-gray-700"
                            >Github</a
                        >
                        <a
                            href="/privacy"
                            class="-mx-3 block rounded-lg px-3 py-2 text-base font-semibold leading-7 hover:bg-gray-700"
                        >
                            Privacy
                        </a>
                    </div>
                </div>
            </div>
        </div>
    </div>
</header>
