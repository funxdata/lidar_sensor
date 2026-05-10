import { show_btn } from "./button.ts";
import { ROISelector } from "./roi.ts";

// ========================
// 全局状态
// ========================
(globalThis as any).rotationOffset = 0;
(globalThis as any).points = [];

// UI
show_btn();

// ========================
// Canvas 初始化
// ========================
const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const ctx = canvas.getContext("2d")!;

// ROI
const roi = new ROISelector(canvas);

// ========================
// resize
// ========================
function resize() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
}

resize();
window.addEventListener("resize", resize);

// ========================
// WebSocket
// ========================
const ws = new WebSocket("ws://172.16.1.126/ws/");

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    (globalThis as any).points = data.points || [];
};

// ========================
// 常量
// ========================
const SCALE = 120;

// ========================
// 防止重复打印（关键）
// ========================
const printedSet = new Set<string>();
let lastPrintTime = 0;

// ========================
// ROI判断
// ========================
function isInsideROI(x: number, y: number) {
    const r = roi.rect;
    return (
        x >= r.x &&
        x <= r.x + r.width &&
        y >= r.y &&
        y <= r.y + r.height
    );
}

// ========================
// render
// ========================
function render() {
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    const points = (globalThis as any).points || [];
    const cx = canvas.width / 2;
    const cy = canvas.height / 2;

    const now = Date.now();

    // ROI 内点缓存
    const insidePoints: any[] = [];

    for (const p of points) {
        const angle = p.angle + (globalThis as any).rotationOffset;

        const x = Math.cos(angle) * p.range * SCALE;
        const y = Math.sin(angle) * p.range * SCALE;

        const sx = cx + x;
        const sy = cy - y;

        const inside = isInsideROI(sx, sy);

        ctx.fillStyle = inside ? "yellow" : "#00ff00";

        ctx.beginPath();
        ctx.arc(sx, sy, 1.5, 0, Math.PI * 2);
        ctx.fill();

        // 收集 ROI 内点
        if (inside) {
            insidePoints.push(p);
        }
    }

    // ========================
    // ⭐ 控制台输出（重点）
    // ========================
    if (insidePoints.length > 0) {
        // 每 300ms 才打印一次（避免刷屏）
        if (now - lastPrintTime > 300) {
            lastPrintTime = now;

            console.log("ROI points:", insidePoints);

            // 或者逐个打印
            // for (const p of insidePoints) {
            //     console.log("hit:", p);
            // }
        }
    }

    // ROI框
    roi.draw();

    requestAnimationFrame(render);
}

render();