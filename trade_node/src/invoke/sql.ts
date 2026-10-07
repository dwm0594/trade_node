import {invoke} from "@tauri-apps/api/core";
import {showToast} from "../toast/useToast";
import {AccountSetting, Dictionary} from "../types/entity";
import {ref} from "vue";
import Account from "../views/Account.vue";

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


export async function insert_attribute(type: string, name: string, value: number) {
    try {
        return await invoke<Dictionary>("insert_or_update_attribute", {
            dictType: type,
            name: name,
            value: value,
        });
    } catch (e) {
        showToast(String(e), 'error')
    }
    return null
}

export async function find_attribute(type: string) {
    try {
        return await invoke<Dictionary>("find_attribute", {
            dictType: type,
        });
    } catch (e) {
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

export const FAVORITE_CURRENCY = 'favorite_currency'
export const USD_ACCOUNT_SETTING = 'usd_account_setting'
export const CNY_ACCOUNT_SETTING = 'cny_account_setting'


export async function insert_account_setting(type: string, account: AccountSetting) {
    try {
        return await invoke<Dictionary>("account_setting", {
            dictType: type,
            account: account,
        });
    } catch (e) {
        showToast(String(e), 'error')
    }
    return null
}


export async function find_account(type: string) {
    try {
        let dictionary = await invoke<Dictionary>("find_account_setting", {
            dictType: type,
        });

        let parse: AccountSetting = JSON.parse(dictionary.name);
        return parse;
    } catch (e) {
    }

    return {
        initialBalance: 0,
        monthlyRisk: 0,
        singleRisk: 0
    }
}