<script lang="ts">
    import { enhance } from "$app/forms";
    import { LockOpen } from "lucide-svelte";
    import Button from "./ui/button/button.svelte";
    import Textarea from "./ui/textarea/textarea.svelte";
    import * as RadioGroup from "$lib/components/ui/radio-group/index.js";
    import VaultDialog from "./VaultDialog.svelte";
    import type { EncryptResponse } from "$lib/api/models/encrypt";
    import type { ApiEmpty } from "$lib/api/models/empty";
    import type { ActionResult } from "@sveltejs/kit";

    type ExpiryMillis = 60 | 900 | 1800;

    let expiryOptions: { id: string; label: string; value: ExpiryMillis }[] = [
        {
            id: "1",
            label: "1 minute",
            value: 60,
        },
        {
            id: "15",
            label: "15 minutes",
            value: 900,
        },
        {
            id: "30",
            label: "30 minutes",
            value: 1800,
        },
    ];

    let textData = $state("");
    let selectedExpiry = $state<ExpiryMillis | undefined>(undefined);

    let encryptForm: any = null;
    let encryptedData = $state<EncryptResponse | ApiEmpty | undefined>(undefined);
    let dialogOpen = $state(false);

    function handleResult(result: ActionResult) {
        if (result.type === "failure") {
            let err = result.data as ApiEmpty;
            encryptedData = err;
        } else if (result.type === "success") {
            encryptedData = result.data as EncryptResponse;
        }
    }
</script>

<div class="px-6 lg:px-0 lg:pt-4">
    <div class="mx-auto max-w-2xl">
        <div class="max-w-lg">
            <form
                bind:this={encryptForm}
                method="POST"
                action="?/encryptData"
                use:enhance={({ formData }) => {
                    formData.append("text", textData);
                    formData.append("expiry", selectedExpiry?.toString() || "0");

                    textData = "";

                    return async ({ update, result }) => {
                        update();
                        handleResult(result);
                    };
                }}
            ></form>

            <div class="flex flex-col gap-6">
                <h1
                    class="mt-10 max-w-lg text-3xl font-bold tracking-tight text-brand-dark dark:text-brand sm:text-5xl"
                >
                    Encrypt your private data
                </h1>
                <div class="flex flex-col col-span-full">
                    <Textarea
                        class="w-full rounded-md min-h-32 border bg-brand/10 sm:text-sm sm:leading-6"
                        bind:value={textData}
                        placeholder="Enter your private text data"
                        required
                    ></Textarea>
                </div>
                <div class="space-y-8">
                    <div class="sm:col-span-6 text-brand-dark dark:text-brand">
                        <p class="text-lg font-semibold">Expiration</p>
                        <p class="text-sm">
                            How long do you want this data to be accessible to the recipient ?
                        </p>
                        <RadioGroup.Root
                            class="flex gap-6 py-4"
                            orientation="horizontal"
                            required
                            onValueChange={(v) => (selectedExpiry = Number(v) as ExpiryMillis)}
                        >
                            {#each expiryOptions as option}
                                <div
                                    class="flex items-center space-x-2 text-brand-dark dark:text-brand"
                                >
                                    <RadioGroup.Item
                                        class="text-brand-dark dark:text-brand"
                                        value={option.value.toString()}
                                        id={option.id}
                                    />
                                    <p class="text-sm">{option.label}</p>
                                </div>
                            {/each}
                        </RadioGroup.Root>
                    </div>

                    <VaultDialog
                        onTriggerClick={() => {
                            if (encryptForm) {
                                encryptForm.dispatchEvent(new Event("submit"));
                            }
                        }}
                        bind:data={encryptedData}
                        bind:open={dialogOpen}
                        type="encrypt"
                    />
                </div>
            </div>
        </div>
    </div>
</div>
