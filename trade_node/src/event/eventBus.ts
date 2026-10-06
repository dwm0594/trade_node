// eventBus.ts
import mitt from 'mitt'

type Events = {
    'trade-node-updated': void,
    'search':{
        start: number,
        end: number,
    }
}

export const emitter = mitt<Events>()