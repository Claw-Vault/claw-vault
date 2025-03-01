import type { ApiEmpty } from "$lib/api/models/empty";
import { decryptClawApi, encryptClawApi } from "$lib/api/vault.server";
import { isType } from "$lib/utils";
import { fail, type Actions } from "@sveltejs/kit";

export const actions: Actions = {
    encryptData: async ({ request }: any) => {
        const formData = (await request.formData()) as any;
        const data = formData.get("text") as string | undefined || "";
        const validity = Number(formData.get("expiry")) || 0;

        if (data.trim().length === 0 || validity === 0) {
            return fail(400, { status: 400, message: "Invalid information provided" });
        }

        try {
            const res = await encryptClawApi(data, validity);
            return { ...res };
        } catch (e) {
            let err = isType<ApiEmpty>(e, "status") ? e as ApiEmpty : { status: 500, message: `${e}` } as ApiEmpty;
            return fail(err.status, { ...err });
        }
    },
    decryptData: async ({ request }: any) => {
        const formData = (await request.formData()) as any;
        const idKey = formData.get("id_key") as string | undefined || "";
        const tokens = idKey.split(".");
        if (tokens.length < 2) {
            return fail(400, { status: 400, message: "Invalid ID.Key format" });
        }

        const id = tokens[0];
        const key = tokens[1];

        try {
            const res = await decryptClawApi(id, key);
            return { ...res };
        } catch (e) {
            let err = isType<ApiEmpty>(e, "status") ? e as ApiEmpty : { status: 500, message: `${e}` } as ApiEmpty;
            return fail(err.status, { ...err });
        }
    },
}
