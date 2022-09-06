import init, {new_ParticleWord, apply_rules, delete_ParticleWord, render} from './pkg/particle_life.js';

const uniqId = (() => {
    let seq = Date.now();
    return () => (seq++).toString(32);
})();


self.performance_now = () => {
    return performance.now();
};

const initPromise = init();

const words = {};
const tasks = {};

const handlers = {
    createNewParticleWord({width, height, rule, canvas, atomsCount}) {
        const wordId = uniqId();
        words[wordId] = new ParticleWord(width, height, rule, canvas, atomsCount);
        return wordId;
    },
    play(wordId) {
        const word = words[wordId];
        if(!word) {
            return;
        }
        const taskId = uniqId();
        let stopped = false;

        (function loop() {
            if(stopped) {
                return;
            }
            // let start = performance.now();
            word.apply_rules();
            // console.log('apply_rules takes:', performance.now() - start, 'ms');
            word.render();
            requestAnimationFrame(loop);
        })();
        tasks[taskId] = () => {
            stopped = true;
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
    constructor(width, height, rule, canvas, atomsCount) {
        this.ptr = new_ParticleWord(width, height, JSON.stringify(rule), canvas.getContext('2d'), atomsCount);
        this.canvas = canvas;
    }
    apply_rules() {
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