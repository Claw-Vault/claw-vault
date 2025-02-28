<script lang="ts">
    import { enhance } from "$app/forms";
    import type { DecryptResponse } from "$lib/api/models/decrypt";
    import type { ApiEmpty } from "$lib/api/models/empty";
    import type { ActionResult } from "@sveltejs/kit";
    import Button from "./ui/button/button.svelte";
    import VaultDialog from "./VaultDialog.svelte";
    import Input from "./ui/input/input.svelte";

    let idKey = $state("");

    let decryptForm: any = null;
    let decryptedData = $state<DecryptResponse | ApiEmpty | undefined>(undefined);
    let dialogOpen = $state(false);

    function handleResult(result: ActionResult) {
        if (result.type === "failure") {
            let err = result.data as ApiEmpty;
            decryptedData = err;
        } else if (result.type === "success") {
            decryptedData = result.data as DecryptResponse;
        }
    }
</script>

<div class="mt-20 sm:mt-24 md:mx-auto md:max-w-2xl lg:mx-0 lg:mt-0 lg:w-screen">
    <div class="shadow-lg md:rounded-3xl">
        <div
            class="bg-brand [clip-path:inset(0)] md:[clip-path:inset(0_round_theme(borderRadius.3xl))]"
        >
            <div class="mx-auto max-w-2xl lg:mx-0 sm:p-16 p-8 lg:flex-auto text-brand-dark">
                <form
                    bind:this={decryptForm}
                    method="POST"
                    action="?/decryptData"
                    use:enhance={({ formData }) => {
                        formData.append("id_key", idKey);
                        idKey = "";

                        return async ({ update, result }) => {
                            update();
                            handleResult(result);
                        };
                    }}
                ></form>

                <div class="flex flex-col gap-8">
                    <h1 class="max-w-lg text-3xl font-bold tracking-tight sm:text-5xl mt-4 sm:mt-8">
                        Get Data
                    </h1>
                    <div class="sm:col-span-4">
                        <div class="mt-2 flex">
                            <Input
                                type="text"
                                autocomplete="off"
                                class="flex-grow placeholder-brand-dark/80 dark:bg-white dark:border-brand max-w-96 rounded-md px-2 border sm:text-sm sm:leading-6"
                                bind:value={idKey}
                                required
                                placeholder="ID.Key"
                            />
                        </div>
                    </div>

                    <VaultDialog
                        onTriggerClick={() => {
                            if (decryptForm) {
                                decryptForm.dispatchEvent(new Event("submit"));
                            }
                        }}
                        bind:data={decryptedData}
                        bind:open={dialogOpen}
                        type="decrypt"
                    />
                </div>
            </div>
        </div>
    </div>
</div>
