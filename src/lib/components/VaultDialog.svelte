<script lang="ts">
    import * as AlertDialog from "$lib/components/ui/alert-dialog/index.js";
    import { ClipboardCopy, LoaderCircle, Lock, LockOpen, X } from "lucide-svelte";
    import { Button, buttonVariants } from "./ui/button";
    import type { EncryptResponse } from "$lib/api/models/encrypt";
    import type { ApiEmpty } from "$lib/api/models/empty";
    import type { DecryptResponse } from "$lib/api/models/decrypt";
    import { isType } from "$lib/utils";
    import Input from "./ui/input/input.svelte";
    import Textarea from "./ui/textarea/textarea.svelte";

    let {
        onTriggerClick,
        data = $bindable(),
        open = $bindable(),
        type,
    }: {
        onTriggerClick: () => void;
        data: EncryptResponse | DecryptResponse | ApiEmpty | undefined;
        open: boolean;
        type: "encrypt" | "decrypt";
    } = $props();

    let remaining = $state(15);
    let timeStarted = $state(false);

    $effect(() => {
        if (!timeStarted && data !== undefined && !isType<ApiEmpty>(data, "status")) {
            startTimer();
        }
    });

    function startTimer() {
        if (timeStarted) {
            return;
        }
        timeStarted = true;
        let timer = 14;
        let interval = setInterval(() => {
            if (!open) {
                clearInterval(interval);
                return;
            }
            remaining = timer;
            timer--;
            if (timer < 0) {
                clearInterval(interval);
            }
        }, 1000);
        setTimeout(() => {
            if (!open) {
                return;
            }
            open = false;
            data = undefined;
            remaining = 0;
            timeStarted = false;
        }, 15 * 1000);
    }

    function resetDialog() {
        open = false;
        data = undefined;
        remaining = 15;
        timeStarted = false;
    }
</script>

<AlertDialog.Root
    bind:open
    onOpenChange={(open) => {
        if (!open) {
            resetDialog();
        }
    }}
>
    <AlertDialog.Trigger
        class={buttonVariants({
            variant: "outline",
            class: `${type === "encrypt" ? "bg-brand hover:bg-brand/70 dark:text-brand-dark" : "bg-brand-dark hover:bg-brand-dark/80 hover:text-brand text-brand border-brand"} w-fit`,
        })}
        onclick={onTriggerClick}
    >
        {#if type === "encrypt"}
            <LockOpen />
            Encrypt
        {:else}
            <Lock />
            Decrypt
        {/if}
    </AlertDialog.Trigger>
    <AlertDialog.Content class="sm:max-w-lg bg-brand text-brand-dark">
        {#if isType<EncryptResponse>(data, "valid_for")}
            <AlertDialog.Header>
                <AlertDialog.Title>Data encrypted !</AlertDialog.Title>
                <AlertDialog.Description class="text-brand-dark/70">
                    Click on copy button before dialog dismisses.
                </AlertDialog.Description>
            </AlertDialog.Header>
            <div class="grid gap-4 py-4">
                <div class="grid grid-cols-9 items-center gap-4">
                    <label for="id" class="text-right">ID</label>
                    <Input
                        id="id"
                        value={data.id}
                        class="col-span-8 bg-white border-brand"
                        disabled
                    />
                </div>
                <div class="grid grid-cols-9 items-center gap-4">
                    <label for="key" class="text-right">Key</label>
                    <Input
                        id="key"
                        value={data.key}
                        class="col-span-8 bg-white border-brand"
                        disabled
                    />
                </div>
                <div class="flex flex-col items-center gap-4">
                    <p class="text-sm">Valid for {data.valid_for}</p>
                    <p class="text-sm">Dialog will close in {remaining} seconds</p>
                </div>
            </div>
        {:else if isType<DecryptResponse>(data, "data")}
            <AlertDialog.Header>
                <AlertDialog.Title>Data decrypted !</AlertDialog.Title>
                <AlertDialog.Description class="text-brand-dark/70">
                    Click on copy button before dialog dismisses.
                </AlertDialog.Description>
            </AlertDialog.Header>
            <div class="grid gap-4 py-4">
                <div class="flex flex-col items-center gap-4">
                    <Textarea value={data.data} class="bg-white" />
                </div>
                <div class="flex flex-col items-center gap-4">
                    <p class="text-sm">Dialog will close in {remaining} seconds</p>
                </div>
            </div>
        {:else if isType<ApiEmpty>(data, "status")}
            <AlertDialog.Header>
                <AlertDialog.Title>Error occurred</AlertDialog.Title>
            </AlertDialog.Header>
            <div class="grid gap-4">
                <div class="flex flex-col items-center gap-4">
                    <X class="size-8 bg-red-100 rounded-full text-red-600" />

                    <p class="flex items-center gap-4 text-sm font-mono">
                        {data.status} - {data.message}
                    </p>
                </div>
            </div>
            <AlertDialog.Footer>
                <AlertDialog.Cancel class="bg-brand-dark text-brand">Close</AlertDialog.Cancel>
            </AlertDialog.Footer>
        {:else}
            <div class="w-full flex items-center justify-center">
                <LoaderCircle class="size-8 text-brand-dark animate-spin" />
            </div>
        {/if}

        {#if data !== undefined && !isType<ApiEmpty>(data, "status")}
            <AlertDialog.Footer>
                <AlertDialog.Action
                    onclick={() => {
                        let text = "";

                        if (isType<EncryptResponse>(data, "valid_for")) {
                            text = `${data.id}.${data.key}`;
                        } else if (isType<DecryptResponse>(data, "data")) {
                            text = data.data;
                        }

                        window.navigator.clipboard.writeText(text).then(
                            () => {
                                resetDialog();
                            },
                            () => {
                                resetDialog();
                            },
                        );
                    }}
                    class="flex items-center gap-x-2 bg-brand-dark hover:bg-brand-dark/70 hover:text-brand text-brand"
                >
                    <ClipboardCopy class="size-5" />
                    Copy data
                </AlertDialog.Action>
            </AlertDialog.Footer>
        {/if}
    </AlertDialog.Content>
</AlertDialog.Root>
