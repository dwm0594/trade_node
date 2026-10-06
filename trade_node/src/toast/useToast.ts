import {ref} from 'vue'

const visible = ref(false)
const message = ref('')
const type = ref<'success' | 'error'>('success')

let timer: number | undefined

export function showToast(
    msg: string,
    toastType: 'success' | 'error' = 'success',
    duration = 2500
) {
    message.value = msg
    type.value = toastType
    visible.value = true

    if (timer) {
        clearTimeout(timer)
    }

    if (toastType === 'error') {
        console.error('[Toast Error]', msg)
    }

    timer = window.setTimeout(() => {
        visible.value = false
    }, duration)
}

export function useToast() {
    return {
        visible,
        message,
        type,
    }
}