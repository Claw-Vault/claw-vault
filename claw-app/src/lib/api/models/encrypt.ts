export interface EncryptResponse {
    id: string;
    key: string;
    valid_for: string;
}

export function emptyEncryptResponse() {
    return {
        id: "",
        key: "",
        valid_for: "",
    } satisfies EncryptResponse
}
