import init, {new_ParticleWord, apply_rules, delete_ParticleWord, render} from './pkg/particle_life.js';

const uniqId = (() => {
    let seq = Date.now();
    return () => (seq++).toString(32);
})();



const initPromise = init();

const words = {};
const tasks = {};

const handlers = {
    createNewParticleWord({width, height, rule, canvas}) {
        const wordId = uniqId();
        words[wordId] = new ParticleWord(width, height, rule, canvas);
        return wordId;
    },
    play(wordId) {
        const word = words[wordId];
        if(!word) {
            return;
        }
        const taskId = uniqId();
        let stopped = false;

        const timmerId = setInterval(() => {
            // word.tick();
        }, 1);

        (function loop() {
            if(stopped) {
                return;
            }
            requestAnimationFrame(() => {
                word.render();
                word.tick();
                loop();
            });
        })();
        tasks[taskId] = () => {
            stopped = true;
            clearInterval(timmerId);
            delete tasks[taskId];
        };
        return taskId;
    },
    cancel(taskId) {
        const cancelTask = tasks[taskId];
        if(typeof cancelTask === 'function') {
            cancelTask();
        }
    }
};



class ParticleWord {
    constructor(width, height, rule, canvas) {
        this.ptr = new_ParticleWord(width, height, JSON.stringify(rule), canvas.getContext('2d'));
        this.canvas = canvas;
    }
    tick() {
        apply_rules(this.ptr);
    }
    render() {
        render(this.ptr)
    }
    free() {
        delete_ParticleWord(this.ptr);
    }
}

self.addEventListener('message', e => {
    const { action, args, callId } = e.data;
    
    const handler = handlers[action];
    
    if(typeof handler !== 'function') {
        return;
    }
    (async () => {
        await initPromise;
        return handler(...(args || []));
    })().then(result => {
        self.postMessage({
            callId, result
        });
    }, error => {
        self.postMessage({
            callId, error
        })
    })
})