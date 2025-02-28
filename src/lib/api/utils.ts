import type { ApiEmpty } from "./models/empty";

export function getApplicationJsonHeader() {
    return {
        "Content-Type": "application/json",
        Accept: "application/json",
    };
}

export async function mapApiResponse<T, R>(
    response: Response,
    def: T,
    transform: (_: T, res: any) => R = (v) => { return v as any as R; }
): Promise<R> {
    let msg = "";
    let data: T;
    let res: any = null;
    // let req_id = response.headers.get("x-cv-id") || "";

    if (response.status === 200 || response.status === 201) {
        let json = await response.json();
        try {
            data = json as T;
        } catch (e) {
            data = def;
        }
    } else {
        data = def;
        msg = await response.text();
        try {
            let json = JSON.parse(msg);
            msg = json.message;
        } catch (e) { }

        throw {
            status: response.status,
            message: msg,
        } as ApiEmpty;
    }

    return transform(data, res ? res : JSON.parse("{}"));
}
