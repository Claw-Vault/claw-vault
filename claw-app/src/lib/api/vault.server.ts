import { BACKEND_URL } from "$env/static/private";
import type { DecryptResponse } from "./models/decrypt";
import type { ApiEmpty } from "./models/empty";
import { emptyEncryptResponse, type EncryptResponse } from "./models/encrypt";
import { getApplicationJsonHeader, mapApiResponse } from "./utils";

export async function encryptClawApi(data: string, validity: number): Promise<EncryptResponse> {
    const url = `${BACKEND_URL}/api/v1/encrypt`;
    const res = await fetch(url, {
        method: "POST",
        headers: getApplicationJsonHeader(),
        body: JSON.stringify({ data, validity })
    });
    return mapApiResponse(res, emptyEncryptResponse());
}

export async function decryptClawApi(id: string, key: string): Promise<DecryptResponse> {
    const url = `${BACKEND_URL}/api/v1/decrypt`;
    const res = await fetch(url, {
        method: "POST",
        headers: getApplicationJsonHeader(),
        body: JSON.stringify({ id, key })
    });
    return mapApiResponse(res, { data: "" });
}

export async function hasClawApi(id: string): Promise<ApiEmpty> {
    const url = `${BACKEND_URL}/api/v1/claw/${id}`;
    const res = await fetch(url, {
        method: "GET",
        headers: getApplicationJsonHeader(),
    });
    return mapApiResponse(res, { status: 400, message: "" });
}
