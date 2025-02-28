<script lang="ts">
    import { enhance } from "$app/forms";
    import { LockOpen } from "lucide-svelte";
    import Button from "./ui/button/button.svelte";
    import Textarea from "./ui/textarea/textarea.svelte";
    import * as RadioGroup from "$lib/components/ui/radio-group/index.js";

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
</script>

<div class="px-6 lg:px-0 lg:pt-4">
    <div class="mx-auto max-w-2xl">
        <div class="max-w-lg">
            <form
                bind:this={encryptForm}
                method="POST"
                use:enhance={({ formData }) => {
                    if (selectedExpiry === undefined) {
                        return;
                    }
                    if (textData.trim().length === 0) {
                        return;
                    }

                    formData.append("text", textData);
                    formData.append("expiry", selectedExpiry.toString());

                    return async ({ update, result }) => {
                        update();
                    };
                }}
            ></form>

            <div class="flex flex-col gap-6">
                <h1
                    class="mt-10 max-w-lg text-4xl font-bold tracking-tight text-brand-dark sm:text-6xl"
                >
                    Encrypt your private data
                </h1>
                <div class="flex flex-col col-span-full">
                    <Textarea
                        class="w-full rounded-md min-h-32 border bg-brand/10 sm:text-sm sm:leading-6"
                        bind:value={textData}
                        required
                    ></Textarea>
                    <p class="mt-3 text-sm leading-6 text-brand-dark/70">
                        Enter your private text data
                    </p>
                </div>
                <div class="space-y-8">
                    <div class="sm:col-span-6 text-brand-dark">
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
                                <div class="flex items-center space-x-2 text-brand-dark">
                                    <RadioGroup.Item
                                        class="text-brand-dark"
                                        value={option.value.toString()}
                                        id={option.id}
                                    />
                                    <label for="option-one">{option.label}</label>
                                </div>
                            {/each}
                        </RadioGroup.Root>
                    </div>

                    <Button
                        type="button"
                        class="w-fit flex items-center gap-x-2 rounded-md bg-brand text-sm font-semibold text-brand-dark shadow-sm hover:bg-brand/70"
                        onclick={() => {
                            if (encryptForm) {
                                encryptForm.dispatchEvent(new Event("submit"));
                            }
                        }}
                    >
                        <LockOpen class="size-5" />
                        Encrypt
                    </Button>
                </div>
            </div>
        </div>
    </div>
</div>
