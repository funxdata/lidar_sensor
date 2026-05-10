const connect=()=> {
const ws = new WebSocket("ws://172.16.1.126/ws/");
    ws.onopen = () => {
        console.log("✅ WS connected");
    };
    ws.onerror = (err) => {
        console.log("❌ WS error", err);
    };
    ws.onclose = (e) => {
        console.log("❌ WS closed", e.code, e.reason);
        // 自动重连
        setTimeout(() => {
            connect();
        }, 1000);
    };
    ws.onmessage = (event) => {
        const data = JSON.parse(event.data);
        // 更新全局点云
        (globalThis as any).points = data.points || [];
    };
}
connect();