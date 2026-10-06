import {invoke} from "@tauri-apps/api/core";
import {showToast} from "../toast/useToast";
import {Dictionary} from "../types/entity";
import {ref} from "vue";

export async function insert_dict(strategy: string, name: string) {
    try {
        const dictionary = await invoke<Dictionary>("insert_dict", {
            strategy: strategy,
            name: name,
        });

        showToast('success');
        return dictionary;
    } catch (e) {
        showToast(String(e), 'error')
    }
    return null
}

export async function find_dict(name: string): Promise<Dictionary[]> {
    const result = ref<Dictionary[]>([])

    try {
        result.value = await invoke<Dictionary[]>("find_strategy", {
            strategy: name
        })

    } catch (e: any) {
        showToast(e.toString(), 'error');
    }

    return result.value
}

