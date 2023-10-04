import { Terminal } from "xterm"
import { AttachAddon } from 'xterm-addon-attach'
import { FitAddon } from 'xterm-addon-fit'

let termAddr = 'ws://localhost:8000/ws'
let controlAddr = 'ws://localhost:8000/control'

function initTerm(element, termAddr, controlAddr) {
    const term = new Terminal({ fontFamily: '"FiraCode Nerd Font", monospace' });
    const fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(element)
    console.log(fitAddon.proposeDimensions())
    fitAddon.fit();
    new ResizeObserver((_) => {
        console.log('Terminal resized, new size is', fitAddon.proposeDimensions())
        fitAddon.fit()
    }).observe(element)
    connect(term, termAddr)
    connectControl(term, controlAddr, fitAddon)
}

function connectControl(term, controlAddr, fitAddon) {
    var ws = new WebSocket(controlAddr);

    ws.addEventListener("open", function () {
        console.log('Control socket is connected.');
    })

    ws.addEventListener("close", function (e) {
        console.log('Socket is closed. Reconnect will be attempted in 1 second.', e.reason);
        setTimeout(function () {
            connectControl(term);
        }, 1000);
    })

    ws.addEventListener("error", function (err) {
        console.error('Socket encountered error: ', err.message, 'Closing socket');
        ws.close();
    })

    term.onResize(function (dim) {
        console.log('Terminal resize detected, new size is', dim)
        ws.send(JSON.stringify(dim))
    })
}

function connect(term, termAddr) {
    var ws = new WebSocket(termAddr);

    ws.addEventListener("open", function () {
        console.log('Socket is connected.');
        const attachAddon = new AttachAddon(ws)
        term.loadAddon(attachAddon)
    })

    ws.addEventListener("close", function (e) {
        console.log('Socket is closed. Reconnect will be attempted in 1 second.', e.reason);
        setTimeout(function () {
            connect(term);
        }, 1000);
    })

    ws.addEventListener("error", function (err) {
        console.error('Socket encountered error: ', err.message, 'Closing socket');
        ws.close();
    })
}

initTerm(document.getElementById('terminal'), termAddr, controlAddr)