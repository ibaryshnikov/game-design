import init, { StageContainer } from './pkg/client_web.js';

window.addEventListener('load', onWindowLoad);

async function onWindowLoad() {
    console.log('Window loaded');
    await init();
    console.log('wasm initialized');

    let wsReady = false;
    let ws = new WebSocket(`ws://${location.host}/ws`);

    function wsWrite(data) {
        if (!wsReady) {
            console.log('ws not ready');
            return;
        }
        ws.send(data);
    }
    const container = new StageContainer(wsWrite);

    function draw() {
        requestAnimationFrame(draw);
        container.updateState();
        container.draw();
    }
    draw();

    ws.addEventListener('open', () => {
        console.log('ws connected')
        wsReady = true;
    });
    ws.addEventListener('error', (e) => {
        console.log('ws error', e);
    });
    ws.addEventListener('close', () => {
        console.log('ws closed');
        wsReady = false;
    });
    ws.addEventListener('message', async (e) => {
        // console.log('ws message', e.data);
        let buffer = await e.data.arrayBuffer();
        // console.log('buffer', buffer);
        let view = new Uint8Array(buffer);
        // console.log('view', view);
        container.handleWsMessage(view);
    });
}
